const { BaseDirectory, join, appDataDir: getAppDataDir } = window.tauri.path;
const { invoke } = window.tauri.core;
const { listen } = window.tauri.event;
const { loadModule } = window['vue3-sfc-loader'];
import { normalizePath } from "./path";
import { toCustomElementName } from "./utils";

const overlay = document.getElementById("overlay");

const appDataDir = await getAppDataDir();

let app = Vue.createApp();

const baseLoaderOptions = {
  moduleCache: {
    vue: Vue, 
    tauri: window.tauri
  },
  getFile: async (url) => {
    // Se è un URL assoluto (es. http:// o https://), fai fetch normalmente
    if (/^https?:\/\//.test(url)) {
      const res = await fetch(url);
      if (!res.ok) {
        throw Object.assign(new Error(`Failed to load ${url}: ${res.statusText}`), { res });
      }
      return res.text();
    }

    if (!url.endsWith(`${baseLoaderOptions.__brick_name}/brick.vue`)) {
      console.log(`[loader]: Component "${baseLoaderOptions.__brick_name}" requesting ${url}`);
    }

    // relative imports
    let resolvedPath = url;
    if (url.startsWith("http:/asset.localhost/") || url.startsWith("/")) {
      resolvedPath = normalizePath(await join(baseLoaderOptions.__brick_path, url.replace('http:/asset.localhost/','')));
    }

    console.log(`[loader]: Component ${url} requested by "${baseLoaderOptions.__brick_name}" resolved to ${resolvedPath}`)

    const response = await fetch(resolvedPath);
    if (!response.ok) {
      throw Object.assign(new Error(`Failed to load ${resolvedPath}: ${response.statusText}`), { response });
    }

    return response.text();
  },
  addStyle: text => {
    const style = document.createElement('style')
    style.textContent = text
    document.head.appendChild(style)
  },
  log: console.log,
  timeout: 30000,
}

// Mappa per tenere traccia degli elementi montati
const mountedBricks = new Set();

async function loadBrick(name) {
  const widgetsDir = await join(appDataDir, 'bricks');
  const widgetDir = await join(widgetsDir, name);
  const vueFilePath = normalizePath(await join(widgetDir, 'brick.vue'));

  console.log(vueFilePath);

  const loaderOptions = {
    ...baseLoaderOptions,
    __brick_path: widgetDir,
    __brick_name: name
  };

  try {
    const component = await loadModule(vueFilePath, loaderOptions);
    app.component(name, component);

    const element = document.createElement(toCustomElementName(name));
    overlay.appendChild(element);

    console.log(`[loader]: ✅ Widget "${name}" loaded`);
  } catch (err) {
    console.error(`[loader]: ❌ Errore while loading widget "${name}":`, err);
  }
}

listen('toggle_brick', async (event) => {
  const { name }  = event.payload;

  if (mountedBricks.has(name)) {
    mountedBricks.delete(name);
  } else {
    mountedBricks.add(name);
  }

  app.unmount(null, true);

  app = Vue.createApp();

  for (const name of mountedBricks) {
    await loadBrick(name);
  }

  app.mount('#overlay');
});

const bricks = await invoke('get_bricks');

for (const brick of bricks) {
  if (!brick.name) continue;
  await loadBrick(brick.name);
  mountedBricks.add(brick.name);
}

app.mount('#overlay');
