const { BaseDirectory, join, appDataDir: getAppDataDir } = window.tauri.path;
const { invoke } = window.tauri.core;
const { listen } = window.tauri.event;
const { loadModule } = window['vue3-sfc-loader'];
import { normalizePath } from "./path";
import { normalizeProps } from './utils';

const appDataDir = await getAppDataDir();

// Loader base
const baseLoaderOptions = {
  moduleCache: { vue: Vue, tauri: window.tauri },
  getFile: async (url) => {
    if (/^https?:\/\//.test(url)) {
      const res = await fetch(url);
      if (!res.ok) throw Object.assign(new Error(`Failed to load ${url}`), { res });
      return res.text();
    }

    let resolvedPath = url.startsWith("/") 
      ? normalizePath(await join(baseLoaderOptions.__brick_path, url.slice(1)))
      : url;

    const response = await fetch(resolvedPath);
    if (!response.ok) throw Object.assign(new Error(`Failed to load ${resolvedPath}`), { response });
    return response.text();
  },
  addStyle: text => {
    const style = document.createElement('style')
    style.textContent = text
    document.head.appendChild(style)
  },
  log: console.log,
  timeout: 30000,
};

const bricksState = Vue.reactive(new Map()); // mappa globale dei brick montati

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

// Funzioni sicure di load/unload
async function loadBrickComponent(brick) {
  if (bricksState.has(brick.name)) return;

  const widgetsDir = await join(appDataDir, 'bricks');
  const widgetDir = await join(widgetsDir, brick.name);
  const vueFilePath = normalizePath(await join(widgetDir, 'brick.vue'));

  const loaderOptions = {
    ...baseLoaderOptions,
    __brick_path: widgetDir,
    __brick_name: brick.name
  };

  const component = await loadModule(vueFilePath, loaderOptions);
  bricksState.set(brick.name, { component, props: Vue.reactive(normalizeProps(brick.props)) });
}

function unloadBrickComponent(name) {
  bricksState.delete(name);
}

// Toggle brick
async function toggleBrick(brick) {
  if (bricksState.has(brick.name)) {
    unloadBrickComponent(brick.name);
  } else {
    await loadBrickComponent(brick);
  }
}

async function updateBrick(brick_name, prop_name, prop_value) {
  const brick = bricksState.get(brick_name);
  if (brick) {
    brick.props[prop_name] = prop_value;
  }
}

// Event listener
listen('toggle_brick', async (event) => {
  await toggleBrick(event.payload.brick);
});

listen('update_prop', async (event) => {
  await updateBrick(event.payload.brick_name, event.payload.prop_name, event.payload.prop_value)
});

// Carico i brick iniziali
const bricks = await invoke('get_bricks');
for (const brick of bricks) {
  if (brick.enabled) await loadBrickComponent(brick);
}

