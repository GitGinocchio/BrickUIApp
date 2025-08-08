const { appDataDir : getAppDataDir } = window.__TAURI__.path;
const { convertFileSrc } = window.__TAURI__.core; 

const appDataDir = await getAppDataDir();

export function normalizePath(input) {
  const isUrl = /^https?:\/\//i.test(input);

  // Riconosco i path assoluti Unix/Android (es. /data/...) e Windows (C:\...)
  const isAbsolutePath =
    input.startsWith('/') || /^[a-zA-Z]:\\/.test(input);

  if (isUrl) {
    // Se è un URL web lo restituisco così com’è
    return input;
  }

  if (isAbsolutePath) {
    // Se è un path assoluto Unix/Android o Windows passo diretto a convertFileSrc
    return convertFileSrc(input);
  }

  // Se è un percorso relativo, lo concateno a appDataPath
  // Pulisco gli slash per evitare doppi o mancanti
  const cleanAppDataPath = appDataDir.replace(/[/\\]+$/, '');  // rimuove slash finali
  const cleanInput = input.replace(/^[/\\]+/, '');             // rimuove slash iniziali

  const joined = `${cleanAppDataPath}/${cleanInput}`;

  return convertFileSrc(joined);
}

export function appendScript(src, type = null, async = false, defer = false, integrity, crossorigin) {
  return new Promise((resolve, reject) => {
    const script = document.createElement("script");
    script.src = src;
    script.async = async;
    script.defer = defer;
    if (type) script.type = type;
    if (integrity) script.integrity = integrity;
    if (crossorigin) script.crossOrigin = crossorigin;

    script.onload = () => resolve();
    script.onerror = () => reject(new Error(`Failed to load script: ${src}`));

    document.body.appendChild(script);
  });
}

export function appendStyle(href, rel = "stylesheet", media = null, integrity = null, crossorigin = null) {
  return new Promise((resolve, reject) => {
    const link = document.createElement("link");
    link.rel = rel;
    link.href = href;
    if (media) link.media = media;
    if (integrity) link.integrity = integrity;
    if (crossorigin) link.crossOrigin = crossorigin;

    link.onload = () => resolve();
    link.onerror = () => reject(new Error(`Failed to load stylesheet: ${href}`));

    document.head.appendChild(link);
  });
}
