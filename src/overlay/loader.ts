import { loadModule } from '../public/vendor/vue3-sfc-loader.esm.js';
import * as Vue from '../public/vendor/vue.esm-browser.prod.js';

// @ts-ignore
//const Vue = await import('https://unpkg.com/vue@3.5.18/dist/vue.esm-browser.prod.js');
// @ts-ignore
//const { loadModule } = await import('https://unpkg.com/vue3-sfc-loader@0.9.5/dist/vue3-sfc-loader.esm.js');

import * as core from '@tauri-apps/api/core';
import * as event from '@tauri-apps/api/event';

import { Brick } from '../interfaces/brick.ts';
import { appDataDir as getAppDataDir, join } from '@tauri-apps/api/path';
import { normalizePath, normalizeProps } from './utils/loader.js';
import { convertFileSrc } from '@tauri-apps/api/core';

const appDataDir = await getAppDataDir();

const bricksState = Vue.reactive(new Map());

function createLoaderOptions({ brickName }) {
  return {
    moduleCache: { 
      vue: Vue,
      core: core,
      event: event,
      fetch: async (input, init) => {
        console.log(input, init)
      }
    },
    /*
    getResource: (path, options) => {
      console.log(`Brick ${brickName} requested resource:`, path, 'with options:', options);
    },
    */
    processStyles(src, lang, filename) {
      console.log(`Processing styles for filename: ${filename} with lang: ${lang}`);

      const brickPath = convertFileSrc(`${appDataDir}/bricks/${brickName}`);

      const result = src.replace(/url\((['"]?)(.+?)\1\)/g, (match, quote, path) => {
        return `url(${brickPath}/${path})`;
      });

      console.log(result);

      return result;
    },
    handleModule: async function (type, getContentData, path, options) {
      console.log(type, getContentData, path, options);
      switch (type) { 
        case '.png':
          return getContentData(true); // load as binary
        case '.gif':
          return getContentData(false); // load as binary
      }
    },
    getFile: async (url) => {
      console.log(`Brick ${brickName} requested file: ${url}`);
      if (/^https?:\/\//.test(url)) {
        const res = await fetch(url);
        if (!res.ok) throw Object.assign(new Error(`Failed to load ${url}`), { res });
        return res.text();
      }
  
      let path = url.startsWith("/") 
        ? normalizePath(await join(`${appDataDir}/bricks/${brickName}`, url.slice(1)))
        : url;

      console.log(path);

      const response = await fetch(path);
      if (!response.ok) throw new Error(`Failed to load ${url}`);
      return response.text();
    },
    addStyle: (text) => {
      const style = document.createElement("style");
      style.textContent = text;
      document.head.appendChild(style);
    },
    log: console.log,
  };
}

export async function loadBrickComponent(brick : Brick) {
  if (bricksState.has(brick.name)) return;

  const loaderOptions = createLoaderOptions({ brickName: brick.name });

  const brickPath = convertFileSrc(`${appDataDir}/bricks/${brick.name}/brick.vue`);

  const component = await loadModule(brickPath, loaderOptions);

  bricksState.set(brick.name, { component, props: Vue.reactive(normalizeProps(brick.props)) });
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

export async function initLoader(bricks: Array<Brick>) {
    const app = Vue.createApp({
        render() {
        const nodes = [];
        for (const [name, { component, props }] of bricksState) {
            nodes.push(Vue.h(component, { key: name, ...props }));
        }
        return Vue.h('div', nodes);
        }
    });

    app.mount('#overlay');

  for (const brick of bricks) {
    if (brick.enabled) await loadBrickComponent(brick);
  }
}
