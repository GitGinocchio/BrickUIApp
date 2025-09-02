import { loadModule } from 'vue3-sfc-loader';
import { compileScript, compileTemplate, compileStyleAsync, parse } from '@vue/compiler-sfc';
import { Options } from 'vue3-sfc-loader/dist/types/vue3-esm/types.js';
import * as Babel from '@babel/standalone';
import * as Vue from 'vue';

import core from './api/core.ts';
import event from './api/event.ts';
import { windowWrapper } from './api/window.ts';

import { Brick } from '../interfaces/brick.ts';
import { appDataDir as getAppDataDir } from '@tauri-apps/api/path';
import { normalizePath, normalizeProps } from './utils/normUtils.js';
import { InvokeArgs, InvokeOptions } from '@tauri-apps/api/core';
import { catchBrickError } from './utils/errors.ts';

const appDataDir = await getAppDataDir();

const bricksState = Vue.reactive(new Map<string,{ component: any, props: any }>);

export let app;

function runInSandbox(code: string, sandbox: Record<string, any>) {
  const sandboxProxy = new Proxy(sandbox, {
    has: () => true, // permette l’uso di qualsiasi variabile definita nel sandbox
  });

  const func = new Function("sandbox", `
    with (sandbox) {
      "use strict";
      return (function() {
        ${code}
      })();
    }
  `);

  return func(sandboxProxy);
}

function createLoaderOptions({ brickName }) {
  const brickPath = `${appDataDir}/bricks/${brickName}`;
  const brickFilename = `${appDataDir}/bricks/${brickName}/brick.vue`;

  const defaultOptions = {
    caller: {
      name: brickName,
      path: brickPath,
      filename: brickFilename
    },
  }

  return {
    moduleCache: { 
      vue: {
        ...Vue,
        createApp: () => {throw new Error("Access denied: bricks are not allowed to create a new Vue app.");},
      },
      core: {
        ...core,
        invoke: (cmd: string, args?: InvokeArgs, options?: InvokeOptions) => core.invoke(cmd, args, { ...options, ...defaultOptions }),
        convertFileSrc: (path: string, protocol?: string) => core.convertFileSrc(path, { protocol, root: brickPath }),
      },
      event: event
    },
    /*
    compiledCache: {
      set(key: string, str: string) {
        console.log('compiledCache set', key, str);
        let attempts = 5;
        while (attempts > 0) {
          try {
            window.localStorage.setItem(key, str);
            return;
          } catch (ex) {
            // fallback: rimuovi la chiave più vecchia
            const firstKey = window.localStorage.key(0);
            if (!firstKey) {
              console.warn("localStorage full, cannot remove any more keys");
              break;
            }
            try {
              window.localStorage.removeItem(firstKey);
            } catch (err) {
              console.warn("Failed to remove localStorage key:", firstKey, err);
              break;
            }
            attempts--;
          }
        }
        console.warn("Failed to set localStorage key:", key);
      },

      get(key: string) {
        console.log('compiledCache get', key);
        try {
          const val = window.localStorage.getItem(key);
          return val ?? null; // sicurezza: non ritorna undefined
        } catch (ex) {
          console.warn("Failed to get localStorage key:", key, ex);
          return null;
        }
      }
    },
    */
    /*
    getResource: ({ refPath, relPath }, options: any) => {
      console.log(`Brick ${brickName} requested resource:`, relPath, 'with options:', options);
      return { 
        id: relPath, 
        path: relPath, 
        getContent: async () => {
          const { getContentData, type } = await options.getFile(relPath);
          return {
            getContentData, // <-- passiamo direttamente la funzione
            type,           // <-- passiamo direttamente il tipo
          };
        }
      }
    },
    */
    processStyles(src, lang, filename) {
      console.log(`Processing styles for filename: ${filename} with lang: ${lang}`);

      const result = src.replace(/url\((['"]?)(.+?)\1\)/g, (match, quote, path: string) => {
        // Se è assoluto, non toccarlo
        if (path.startsWith('https://')) {
          return match;
        }
        
        return `url(${normalizePath(path, { root: brickPath })})`;
      });

      return result;
    },
    handleModule: async function (type, getContentData, path, options) {
      console.log(type, getContentData, path, options);
      switch (type) {
        case '.vue':
          return undefined;
        case '.css':
          options.addStyle(await getContentData(false));
          return null;
        // images
        case '.png':
        case '.jpg':
        case '.jpeg':
        case '.webp':
        // audio
        case '.mp3':
        case '.wav':
        case '.ogg':
        // compressed files
        case '.zip':
        case '.tar':
        case '.gz':
          return getContentData(true); // load as binary
        // text based
        case '.svg':
        case '.gif':
        case '.txt':
        case '.md':
        case '.json':
        case '.csv':
        case '.xml':
          return getContentData(false);
        default:
          console.warn(`File type ${type} not handled, loading as binary by default`);
          return getContentData(true);
      }
    },
    getFile: async (url) => {
      console.log(`Brick ${brickName} requested file: ${url}`);
      let path = url;

      if (url.startsWith("/")) {
        path = normalizePath(url.slice(1), { root: brickPath });
      }

      const match = path.match(/\.([^.]+)$/);
      const type = match ? `.${match[1].toLowerCase()}` : null;

      const response = await fetch(path);
      if (!response.ok) throw new Error(`Failed to load ${url}`);

      /*
      return {
        type,
        getContentData: async (asBinary: boolean) => {
          if (asBinary) {
            const buffer = await response.arrayBuffer();
            return new Uint8Array(buffer);
          } else {
            return await response.text();
          }
        }
      };
      */
      return await response.text();
    },
    addStyle: (text) => {
      console.log(`Brick ${brickName} requested style: ${text}`);
      const style = document.createElement("style");
      style.textContent = text;
      document.head.appendChild(style);

      //const style = Object.assign(document.createElement('style'), { text });
      //const ref = document.head.getElementsByTagName('style')[0] || null;
      //document.head.insertBefore(style, ref);
    },
    __name: brickName,
    __dirname: brickPath,
    __filename: brickFilename,
    log(type, ...args) {
      console.log(type, ...args);
    },
    devMode: true
  }; // as Options;
}

export async function loadBrickComponent(brick : Brick) {
  if (bricksState.has(brick.name)) return;

  const loaderOptions = createLoaderOptions({ brickName: brick.name });

  const brickPath = normalizePath(`${appDataDir}/bricks/${brick.name}/brick.vue`);

  const component = await loadModule(brickPath, loaderOptions);
  bricksState.set(brick.name, { component: Vue.markRaw(component), props: Vue.reactive(normalizeProps(brick.props)) });
}

export function unloadBrickComponent(name : string) {
  bricksState.delete(name);
}

export async function toggleBrick(brick : Brick) {
  if (bricksState.has(brick.name)) unloadBrickComponent(brick.name);
  else await loadBrickComponent(brick);
}

export async function updateBrick(name : string, prop_name : string, prop_value : any) {
  const brick = bricksState.get(name);
  if (brick) brick.props[prop_name] = prop_value;
}

export async function initLoader(bricks: Array<Brick>, errorHandler: (error, instance, info) => {}, warnHandler: (error, instance, info) => {}) {
  app = Vue.createApp({
    render() {
      const nodes = [];
      for (const [name, { component, props }] of bricksState) {
        nodes.push(Vue.h(component, { key: name, ...props }));
      }
      return Vue.h('div', nodes);
    }
  });
  
  app.config.errorHandler = errorHandler;
  app.config.warnHandler = warnHandler;

  app.config.throwUnhandledErrorInProduction = true;

  app.mount('#overlay');

  for (const brick of bricks) {
    if (brick.enabled) await loadBrickComponent(brick);
  }
}
