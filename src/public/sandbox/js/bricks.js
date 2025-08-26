import * as Vue from "./vendor/vue.esm-browser.prod.js";
import { loadModule } from "./vendor/vue3-sfc-loader.esm.js";
import { getAppDataDir, getFileContent, getFetchResult } from "./bridge.js";
import { normalizeProps } from "./utils.js";

export const bricksState = Vue.reactive(new Map());

let appDataDir;

export async function loadBrickComponent(brick) {
  if (bricksState.has(brick.name)) return;

  if (!appDataDir) {
    appDataDir = await getAppDataDir();
  }

  const loaderOptions = {
    moduleCache: { vue: Vue },
    getFile: async (url) => {
      if (url === "brick.vue") {
        return await getFileContent(`${appDataDir}/bricks/${brick.name}/brick.vue`);
      }
      else if (url.startsWith("/")) {
        const path = `${appDataDir}/bricks/${brick.name}${url}`;
        return await getFileContent(path);
      }

      const response = getFetchResult(url);
      if (!response.ok) throw new Error(`Failed to load ${url}`);
      return response.text();
    },
    addStyle: (text) => {
      const style = document.createElement("style");
      style.textContent = text;
      document.head.appendChild(style);
    },
    log: console.log,
    timeout: 30000,
  };

  const component = await loadModule("brick.vue", loaderOptions);
  bricksState.set(brick.name, { component, props: Vue.reactive(normalizeProps(brick.props)) });
}

export function unloadBrickComponent(name) {
  bricksState.delete(name);
}

export async function toggleBrick(brick) {
  if (bricksState.has(brick.name)) unloadBrickComponent(brick.name);
  else await loadBrickComponent(brick);
}

export async function updateBrick(name, prop_name, prop_value) {
  const brick = bricksState.get(name);
  if (brick) brick.props[prop_name] = prop_value;
}
