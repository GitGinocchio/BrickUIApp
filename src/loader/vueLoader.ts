import { Brick } from "interfaces/brick";
import { App, Component } from "vue";

import { BaseDirectory, readTextFile } from "@tauri-apps/plugin-fs";
import { compileScript, compileStyleAsync, compileTemplate, parse } from "@vue/compiler-sfc";
import { appDataDir, dirname, filename, getRelativePath, normalizePath } from "./utils";
import { extractImports, rewriteImports } from "./scriptProcessor";
import { transform } from "@babel/standalone";

import { windowWrapper } from "../api/window";
import { processStyle } from "./styleProcessor";
import { addErrorToBrickState } from "./state";
import { catchBrickError } from "../utils/errors";

export async function loadVueModuleToCJS(
  source: string,
  componentPath: string,
  brickFilePath: string,
  moduleCache: object = {},
  brick: Brick | null = null,
): Promise<Component> {
  const brickDirName = dirname(brickFilePath);
  const componentDirName = dirname(componentPath);
  const errors = [];

  // 1. Fa il parse del file .vue
  const parsed = parse(source, { filename: componentPath });
  const descriptor = parsed.descriptor;
  const id = btoa(componentPath);

  const imports = extractImports(descriptor);
  
  await Promise.all(
    imports.map(async (binding) => {
      if (binding.imported === 'default' && binding.source.endsWith('.vue')) {
        const importPath = normalizePath(binding.source, { root: componentDirName }, false);

        if (importPath === componentPath) {
          const error = new Error(`You can't import module ${filename(importPath)} in the same module!`);
          return Promise.reject(error);
        }

        if (importPath in moduleCache) return Promise.resolve();

        const relativeImportPath = getRelativePath(`${componentDirName}/${binding.source}`, appDataDir);
        const normRelImportPath = normalizePath(relativeImportPath, { root: appDataDir }, false);

        if (normRelImportPath === brickFilePath) {
          const error = new Error(`You can't import module ${filename(normRelImportPath)} in ${filename(brickFilePath)} module!`);
          return Promise.reject(error);
        }

        try {
          const source = await readTextFile(normRelImportPath, { baseDir: BaseDirectory.AppData });
          moduleCache[normRelImportPath] = await loadVueModuleToCJS(source, normRelImportPath, importPath, moduleCache, brick);
        } catch (error) {
          return Promise.reject(new Error(error));
        }
      }
    })
  );

  errors.push(...parsed.errors);

  // 2. Compila lo script
  let script;
  try {
    script = compileScript(descriptor, { id });
  }
  catch (error) {
    return Promise.reject(error);
  }
  
  let scriptContent = script.content.replace(/export\s+default\s+/, 'const __script = ');
  if (script.warnings) errors.push(...script.warnings.map(message => new Error(message)));

  // 3. Compila il template
  let renderCode = '';
  if (descriptor.template) {
    const template = compileTemplate({
      source: descriptor.template.content,
      filename: componentPath,
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
      filename: componentPath, 
      id 
    });

    errors.push(...css.errors);

    if (css.code) {
      const code = processStyle(css.code, 'css', dirname(componentPath));
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

    ${brick !== null ? `
    __script.__brickContext = {
      name: "${brick.name}",
      author: "${brick.author}",
      component: "${filename(componentPath, false)}"
    };` : ''
    }

    const fetch = (...args) => window.fetch(args);

    module.exports = { default: __script, render: __script.render };
  `;

  // 6. Riscrive gli import verso moduleCache
  fullCode = rewriteImports(fullCode, moduleCache, brickDirName);

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
    catchBrickError(error, { name: brick?.name, author: String(brick?.author) }, "parsing")
  });

  // 8. Esegue il codice in un modulo CommonJS per ottenere la render funcion
  const mod: { exports: { default: any; render?: any } } = { exports: { default: undefined } };
  try {
    const fn = new Function('module', 'exports', 'moduleCache', 'window', babelResult.code);
    const result = fn(mod, mod.exports, moduleCache, windowWrapper);
    if (result instanceof Promise) {
      result.catch(error => {
        catchBrickError(error, { name: brick?.name, author: String(brick?.author) }, "executing");
        return Promise.reject(error);
      });
    }
  } catch (error) {
    catchBrickError(error, { name: brick?.name, author: String(brick?.author) }, "executing");
    return Promise.reject(error);
  }

  return mod.exports.default;
}