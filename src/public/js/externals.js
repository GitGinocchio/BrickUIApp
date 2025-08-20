
const { readTextFile, BaseDirectory } = window.tauri.fs;
const { appDataDir : getAppDataDir, join } = window.tauri.path;
import jsyaml from 'https://cdn.jsdelivr.net/npm/js-yaml@4.1.0/+esm';
const { appendScript, appendStyle } = await import('./utils.js');

const appDataDir = await getAppDataDir();

const vueScript = {
  src: "https://unpkg.com/vue@3.5.18/dist/vue.global.prod.js",
  integrity: "sha256-WCjzMsGm4FrAjof1HjwoqId2CabTEy2JnJfUjaEscno=",
  crossorigin: "anonymous"
};

const sfcLoaderScript = {
  src: "https://unpkg.com/vue3-sfc-loader@0.9.5/dist/vue3-sfc-loader.js",
  integrity: "sha256-ARV0Eah86yaV1qsmKVNH83FEpxlFUONZ5Nyas3jVdzs=",
  crossorigin: "anonymous"
};

export const scripts = [vueScript, sfcLoaderScript];
export const styles = [];

try {
    const yamlText = await readTextFile(await join(appDataDir, 'externals.yml'), { baseDir: BaseDirectory.AppData });
    const config = jsyaml.load(yamlText);

    if (config.styles && Array.isArray(config.styles)) {
        styles.push(...config.styles);
        config.styles.forEach(style => {
            console.log(`[externals]: loading external style ${style.rel}`)
            appendStyle(
                style.href,
                style.rel || 'stylesheet',
                style.media,
                style.integrity,
                style.crossorigin
            )
        });
    }

    if (config.scripts && Array.isArray(config.scripts)) {
        scripts.push(...config.scripts);
        config.scripts.forEach(script => {
            console.log(`[externals]: loading external script ${script.src}`)
            appendScript(
                script.src, 
                script.type, 
                script.async,
                script.defer,
                script.integrity,
                script.crossorigin
            );
        });
    }

    console.log('[externals]: Scripts and styles loaded from YAML');
} catch (e) {
    console.error('Failed to load or parse YAML config:', e);
}