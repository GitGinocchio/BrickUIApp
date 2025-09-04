import { h, createApp } from "vue";
import { bricksState, disableBrick, getBrickFromState, isBrickInState, setBrickState } from "./state";
import { addBrickToCache, getBrickFromCache } from "./cache";
import { Brick } from "interfaces/brick";
import { loadVueModuleToCJS } from "./vueLoader";
import { createModuleCache } from "./moduleCache";
import { appDataDir, normalizePath } from "./utils";

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
    return h('div', nodes);
  }
});

export async function loadBrickComponent(brick: Brick) {
  let component = getBrickFromCache(brick.name);

  if (!component) {
    const moduleCache = createModuleCache({
      path: normalizePath(`./bricks/${brick.name}/brick.vue`, { root: appDataDir }),
      name: brick.name,
    });

    component = await loadVueModuleToCJS(brick, moduleCache);
    addBrickToCache(brick.name, component);
  }

  setBrickState(brick);
}

export async function toggleBrick(brick : Brick) {
  const state = getBrickFromState(brick.name);
  if (state.enabled) disableBrick(brick.name);
  else await loadBrickComponent(brick);
}

export async function updateBrick(name : string, prop_name : string, prop_value : any) {
  const brick = getBrickFromState(name);
  if (brick) brick.props[prop_name] = prop_value;
}

export async function initLoader(
  bricks: Brick[], 
  errorHandler: (error, instance, info) => {}, 
  warnHandler: (message, instance, trace) => {}
) {
  app.mount('#overlay');

  app.config.errorHandler = errorHandler;
  app.config.warnHandler = warnHandler;

  for (const brick of bricks) {
    if (brick.enabled) await loadBrickComponent(brick);
  }
}