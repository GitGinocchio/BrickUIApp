//const { appDataDir : getAppDataDir } = window.tauri.path;
//const { convertFileSrc } = window.tauri.core;

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