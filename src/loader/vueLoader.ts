import { Brick } from "interfaces/brick";
import { App, Component } from "vue";

import { BaseDirectory, readTextFile } from "@tauri-apps/plugin-fs";
import { compileScript, compileStyleAsync, compileTemplate, parse } from "@vue/compiler-sfc";
import { appDataDir, normalizePath } from "./utils";
import { rewriteImports } from "./scriptProcessor";
import { transform } from "@babel/standalone";

import { windowWrapper } from "../api/window";
import { processStyle } from "./styleProcessor";
import { addErrorToBrickState } from "./state";
import { catchBrickError } from "../utils/errors";

export async function loadVueModuleToCJS(
  brick: Brick, 
  moduleCache: object = {}
): Promise<Component> {
  const source = await readTextFile(`./bricks/${brick.name}/brick.vue`, { baseDir: BaseDirectory.AppData });
  const path = normalizePath(`./bricks/${brick.name}/brick.vue`, { root: appDataDir });

  const errors = [];

  // 1. Fa il parse del file .vue
  const parsed = parse(source, { filename: path });
  const descriptor = parsed.descriptor;
  const id = btoa(path);

  errors.push(...parsed.errors);

  // 2. Compila lo script
  const script = compileScript(descriptor, { id });
  const scriptContent = script.content.replace(/export\s+default\s+/, 'const __script = ');
  if (script.warnings) errors.push(...script.warnings.map(message => new Error(message)));

  console.log(script);
  // Qui va fatta la riscrittura degli import

  // 3. Compila il template
  let renderCode = '';
  if (descriptor.template) {
    const template = compileTemplate({
      source: descriptor.template.content,
      filename: path,
      id,
      compilerOptions: { 
        mode: 'module', 
        bindingMetadata: script?.bindings 
      },
    })

    console.log(template);

    errors.push(...template.errors.map(message => typeof message === "string" ? new Error(message) : message));

    renderCode = template.code.replace(/export function render/, 'function render');
  }

  // 4. Compila gli stili e aggiungili all'<head>
  for (const style of descriptor.styles) {
    const css = await compileStyleAsync({ 
      source: style.content, 
      filename: path, 
      id 
    });

    errors.push(...css.errors);

    if (css.code) {
      const code = processStyle(css.code, 'css', brick);
      const styleEl = document.createElement('style');
      styleEl.textContent = code;
      document.head.appendChild(styleEl);
    }
  }

  // 5. Costruisce il codice completo typescript prima di convertirlo in javascript
  let fullCode = `
    ${scriptContent}
    ${renderCode}
    if (typeof render !== 'undefined') __script.render = render;

    __script.__brickContext = {
      name: "${brick.name}",
      author: "${brick.author}"
    };

    const fetch = (...args) => window.fetch(args);

    module.exports = { default: __script, render: __script.render };
  `;

  // 6. Riscrive gli import verso moduleCache
  fullCode = rewriteImports(fullCode, moduleCache);

  // 7. Trasforma con Babel (JS moderno/CJS → JS compatibile)
  const babelResult = transform(fullCode, {
    presets: [
      ['es2015', { modules: false }], // lascia export/import come sono già stati riscritti
      'typescript'
    ],
    plugins: ['proposal-class-properties', 'transform-typescript'],
    filename: 'file.js',
    sourceMaps: false
  });

  errors.forEach(error => {
    console.log(error);
    catchBrickError(error, { name: brick.name, author: String(brick.author) }, "parsing")
  });

  // 8. Esegue il codice in un modulo CommonJS per ottenere la render funcion
  const mod: { exports: { default: any; render?: any } } = { exports: { default: undefined } };
  try {
    const fn = new Function('module', 'exports', 'moduleCache', 'window', babelResult.code);
    const result = fn(mod, mod.exports, moduleCache, windowWrapper);
    if (result instanceof Promise) {
      result.catch(error => {
        //console.error(`[Brick: ${brick.name}] Runtime async error:`, error);
        catchBrickError(error, { name: brick.name, author: String(brick.author) }, "executing");
        return Promise.reject(error);
      });
    }
  } catch (error) {
    //console.error(`[Brick: ${brick.name}] Runtime sync error:`, error);
    catchBrickError(error, { name: brick.name, author: String(brick.author) }, "executing");
    return Promise.reject(error);
  }

  return mod.exports.default;
}