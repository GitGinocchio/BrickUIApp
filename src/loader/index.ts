import { h, createApp, Fragment } from "vue";
import {
  bricksState,
  disableBrick,
  getBrickFromState,
  removeBrickFromState,
  setBrickState,
} from "./state";
import { addBrickToCache, getBrickFromCache, removeBrickFromCache } from "./cache";
import type { Brick, Prop } from "#interfaces";
import { loadVueModuleToCJS } from "./vueLoader";
import { createModuleCache } from "./moduleCache";
import { sanitizePath } from "#utils/path";
import { formatPropValue } from "#utils/format";
import { BaseDirectory, readTextFile } from "@tauri-apps/plugin-fs";
import { convertFileSrc } from "@tauri-apps/api/core";
import { catchBrickError } from "#utils/errors";
import { initBrickWatcher } from "./watcher";

export const app = createApp({
  render() {
    const nodes = [];
    for (const [name, state] of bricksState) {
      if (!state.enabled) continue;

      let component = getBrickFromCache(name);

      if (!component) {
        console.error(`Brick ${name} not saved in cache!`);
        continue;
      }

      nodes.push(h(component, { key: name, ...state.props }));
    }
    return h(Fragment, nodes);
  },
});

export async function loadBrickComponent(brick: Brick, force: boolean = false) {
  let component = getBrickFromCache(brick.name);

  const path = await sanitizePath(
    `./bricks/${brick.name}/brick.vue`,
    { convertToAssetURL: false }
  );

  if (!component || force) {
    const moduleCache = createModuleCache({
      path: convertFileSrc(path),
      name: brick.name,
    });

    try {
      const source = await readTextFile(`./bricks/${brick.name}/brick.vue`, {
        baseDir: BaseDirectory.AppData,
      });
      const module = await loadVueModuleToCJS(
        source,
        path,
        path,
        moduleCache,
        brick
      );
      component = module.default;
    } catch (error) {
      catchBrickError(
        error,
        { name: brick.name, author: brick.author },
        "importing"
      );
      return Promise.resolve();
    }

    addBrickToCache(brick.name, component);
  }

  setBrickState(brick);
}

export async function toggleBrick(brick: Brick) {
  const state = getBrickFromState(brick.name);

  if (!state) await loadBrickComponent(brick);
  else if (!state.enabled) await loadBrickComponent(brick);
  else disableBrick(brick.name);
}

export async function updateBrickProp(name: string, prop: Prop) {
  const brick = getBrickFromState(name);
  if (brick && isValidProp(prop) && isNotNullProp(prop)) brick.props[prop.prop_name] = formatPropValue(prop);
}

export async function deleteBrick(brick: Brick) {
  disableBrick(brick.name);
  removeBrickFromState(brick.name);
  removeBrickFromCache(brick.name);
}

export async function reloadBrick(brick: Brick) {
  await loadBrickComponent(brick, true);
}

export async function initLoader(
  bricks: Brick[],
  errorHandler: (error, instance, info) => {},
  warnHandler: (message, instance, trace) => {}
) {
  app.mount("#overlay");

  app.config.errorHandler = errorHandler;
  app.config.warnHandler = warnHandler;

  for (const brick of bricks) {
    if (brick.enabled) await loadBrickComponent(brick);
  }

  await initBrickWatcher();
}
