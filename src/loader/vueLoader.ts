import { Brick } from "interfaces/brick";
import { Component } from "vue";

import { BaseDirectory, readTextFile } from "@tauri-apps/plugin-fs";
import { compileScript, compileStyleAsync, compileTemplate, parse } from "@vue/compiler-sfc";
import { appDataDir, dirname, basename, relative, sanitizePath } from "../utils/path";
import { extractImports, rewriteImports } from "./scriptProcessor";
import { transform } from "@babel/standalone";

import { windowWrapper } from "../api/window";
import { processStyle } from "./styleProcessor";
import { catchBrickError } from "../utils/errors";
//import { addErrorToBrickState } from "./state";

export async function loadVueModuleToCJS(
  source: string,
  componentPath: string,
  brickFilePath: string,
  moduleCache: object = {},
  brick: Brick | null = null,
): Promise<{ default: Component, render?: () => void}> {
  const brickDirName = await dirname(brickFilePath);
  const componentDirName = await dirname(componentPath);
  const errors = [];

  // 1. Fa il parse del file .vue
  const parsed = parse(source, { filename: componentPath });
  const descriptor = parsed.descriptor;

  const bytes = new TextEncoder().encode(componentPath);
  const base64 = btoa(String.fromCharCode(...bytes));
  const id = base64
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");

  const imports = extractImports(descriptor);
  
  await Promise.all(
    imports.map(async (binding) => {
      if (binding.imported === 'default' && binding.source.endsWith('.vue')) {
        console.log(binding.source, componentDirName);
        const importPath = await sanitizePath(binding.source, { root: componentDirName, convertToAssetURL: false });

        if (importPath === componentPath) {
          const error = new Error(`You can't import module ${await basename(importPath)} in the same module!`);
          return Promise.reject(error);
        }

        if (importPath in moduleCache) return Promise.resolve();

        const relativeImportPath = relative(`${componentDirName}/${binding.source}`, appDataDir);
        const normRelImportPath = await sanitizePath(relativeImportPath, { root: appDataDir, convertToAssetURL: false });

        if (normRelImportPath === brickFilePath) {
          const error = new Error(`You can't import module ${await basename(brickFilePath)} in module ${await basename(componentPath)}!`);
          return Promise.reject(error);
        }

        try {
          const source = await readTextFile(normRelImportPath, { baseDir: BaseDirectory.AppData });
          moduleCache[normRelImportPath] = await loadVueModuleToCJS(source, normRelImportPath, brickFilePath, moduleCache, brick);
        } catch (error) {
          return Promise.reject(new Error(error));
        }
      }
    })
  );

  console.log(moduleCache);

  errors.push(...parsed.errors);

  // 2. Compila lo script
  let scriptContent = "";
  let bindings: any = undefined;
  try {
    if (descriptor.scriptSetup || descriptor.script) {
      const compiled = compileScript(descriptor, { id });
      if (compiled.warnings) {
        errors.push(...compiled.warnings.map(message => new Error(message)));
      }

      scriptContent = compiled.content.replace(/export\s+default\s+/, "const __script = ");
      scriptContent  = scriptContent.replace(/export\s*{[^}]+};?/g, "");
      bindings = compiled.bindings;
    } else {
      scriptContent = "const __script = {}";
    }
  } catch (error) {
    return Promise.reject(error);
  }

  // 3. Compila il template
  let renderCode = '';
  if (descriptor.template) {
    const template = compileTemplate({
      source: descriptor.template.content,
      filename: componentPath,
      id,
      compilerOptions: { 
        mode: 'module', 
        bindingMetadata: bindings
      }
    })

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
      const code = await processStyle(css.code, 'css', await dirname(componentPath), id);
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

    __script.__scopeId = "data-v-${id}";

    ${brick !== null ? `
    __script.__brickContext = {
      name: "${brick.name}",
      author: "${brick.author}",
      component: "${(await basename(componentPath)).split(".")[0]}",
    };` : ''
    }

    const fetch = (...args) => window.fetch(args);

    console.log(moduleCache);

    module.exports = { 
      default: __script, 
      render: __script.render
    };
  `;

  // 6. Riscrive gli import verso moduleCache
  fullCode = await rewriteImports(fullCode, moduleCache, brickDirName);

  // 7. Trasforma con Babel (JS moderno/CJS → JS compatibile)
  const babelResult = transform(fullCode, {
    presets: [
      ['es2015', { modules: false }], // lascia export/import come sono già stati riscritti
      'typescript'
    ],
    plugins: ['proposal-class-properties', 'transform-typescript'],
    filename: 'file.js',
    sourceMaps: true
  });

  errors.forEach(error => {
    console.log(error);
    catchBrickError(error, { name: brick?.name, author: String(brick?.author) }, "parsing")
  });

  // 8. Esegue il codice in un modulo CommonJS per ottenere la render funcion
  const mod: { exports: { default: Component; render?: () => void } } = { exports: { default: undefined } };
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
  return mod.exports;
}