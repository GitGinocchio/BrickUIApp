import { normalizePath } from "./path";

const overlay = document.getElementById("overlay");

const { readDir } = window.tauri.fs;
const { BaseDirectory, join, appDataDir: getAppDataDir } = window.tauri.path;

const appDataDir = await getAppDataDir();

const { loadModule } = window['vue3-sfc-loader'];

const app = Vue.createApp();

const folders = await readDir('bricks', { baseDir: BaseDirectory.AppData });

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

    if (!url.endsWith(`${baseLoaderOptions.__brick_name}/widget.vue`)) {
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

for (const folder of folders) {
    if (!folder.name) continue

    const name = folder.name
    const widgetsDir = await join(appDataDir, 'bricks');
    const widgetDir = await join(widgetsDir, name);
    const vueFilePath = normalizePath(await join(widgetDir, 'brick.vue'));

    const loaderOptions = {
      ...baseLoaderOptions,
      __brick_path: widgetDir,
      __brick_name: name
    }

    try {
      const component = await loadModule(vueFilePath, loaderOptions)
      app.component(name, component)

      const element = document.createElement(name);
      overlay.appendChild(element);

      console.log(`[loader]: ✅ Widget "${name}" loaded`)
    } catch (err) {
        console.error(`[loader]: ❌ Errore while loading widget "${name}":`, err)
    }
}

app.mount('#overlay');