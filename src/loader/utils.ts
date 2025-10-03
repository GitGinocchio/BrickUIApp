import { Prop, PropType } from "interfaces/brick";
import { convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";

export const appDataDir = await getAppDataDir();

export function formatPropValue(prop: Prop): any {
  let value: any;

  // Se il prop e' un select e il massimo di scelte e' una, passiamo solo l'unica scelta
  // al posto di un array con una scelta
  if (Array.isArray(prop.value) && prop.prop_type === 'Select' && prop.value.length == 1 && prop.max === 1) {
    value = prop.value[0]
  }
  else if (Array.isArray(prop.value) && prop.value.length > 0) {
    value = prop.value;
  }
  else if (Array.isArray(prop.value)) {
    value = prop.default;
  }
  else {
    value = prop.value;
  }

  return value;
}

export function formatProps(props: Prop[]): Record<string, any> {
  return Object.fromEntries(
    props.map((p: Prop) => {
      return [p.prop_name, formatPropValue(p)]
    })
  );
}


function normalizeRelativeSegments(p: string, root: string): string {
  const isRootUrl = /^https?:\/\//i.test(root) || /^asset:/.test(root);
  
  const stack: string[] = [];
  let rootParts = [];

  if (!/^([a-zA-Z]:)?[\\/]/.test(p)) {
    // path relativo → inizializza con root
    const rootParts = root.replace(/[/\\]+$/, '').split(/[/\\]+/);
    stack.push(...rootParts);
  }

  const parts = p.split(/[/\\]+/);

  for (const part of parts) {
    if (part === '' || part === '.') continue;
    if (part === '..') {
      if (stack.length > rootParts.length) stack.pop(); // non uscire dalla root
    } else {
      stack.push(part);
    }
  }

  const normalized = stack.join('/');
  return isRootUrl ? normalized : normalized.replace(/\\/g, '/');
}

export function normalizePath(path: string, options?: { protocol?: string; root?: string }, useConvertFileSrc: boolean = true) {
  const protocol = options?.protocol ?? 'asset';
  const root = options?.root ?? appDataDir;

  // Se è un URL asset
  const assetMatch = path.match(/^http:\/\/asset\.localhost\/(.+)$/);
  if (assetMatch) {
    const absPath = decodeURIComponent(assetMatch[1]);
    const normalized = normalizeRelativeSegments(absPath, root);
    return convertFileSrc(normalized, protocol);
  }

  // Se è un URL esterno (non asset)
  if (/^https?:\/\//i.test(path)) {
    return path;
  }

  // Percorso relativo o assoluto
  const normalized = normalizeRelativeSegments(path, root);
  console.log('normalized', normalized);
  return useConvertFileSrc ? convertFileSrc(normalized, protocol) : normalized;
}

export function dirname(filePath: string): string {
  const decoded = decodeURIComponent(filePath);

  // Normalizza separatori (Unix/Windows)
  const normalized = decoded.replace(/\\/g, "/");
  
  // Rimuove eventuale trailing slash
  const cleanPath = normalized.replace(/\/+$/, "");

  // Trova l'ultimo slash
  const lastSlashIndex = cleanPath.lastIndexOf("/");

  // Se non c'è slash, restituisce "."
  if (lastSlashIndex === -1) return ".";

  // Ritorna tutto fino all'ultimo slash
  return cleanPath.slice(0, lastSlashIndex);
}

export function filename(filePath: string, ext: boolean = true): string {
  // Decodifica eventuali caratteri URL-encoded
  const decoded = decodeURIComponent(filePath);

  // Normalizza separatori (Unix/Windows)
  const normalized = decoded.replace(/\\/g, "/").replace(/\/+$/, "");

  // Trova l'ultimo slash
  const lastSlashIndex = normalized.lastIndexOf("/");
  let base = lastSlashIndex === -1 ? normalized : normalized.slice(lastSlashIndex + 1);

  // Se ext è false, rimuove l'estensione
  if (!ext) {
    const lastDotIndex = base.lastIndexOf(".");
    if (lastDotIndex > 0) {
      base = base.slice(0, lastDotIndex);
    }
  }

  return base;
}

export function getRelativePath(filePath: string, root: string): string {
  // Decodifica temporaneamente
  const decodedFilePath = decodeURIComponent(filePath).replace(/\\/g, "/").replace(/\/+$/, "");
  const decodedRoot = decodeURIComponent(root).replace(/\\/g, "/").replace(/\/+$/, "");

  const fileSegments = decodedFilePath.split("/");
  const rootSegments = decodedRoot.split("/");

  // Trova la parte comune
  let commonLength = 0;
  for (; commonLength < Math.min(fileSegments.length, rootSegments.length); commonLength++) {
    if (fileSegments[commonLength] !== rootSegments[commonLength]) break;
  }

  // Segmenti da risalire dal root
  const upSegments = rootSegments.length - commonLength;
  const relativeParts = Array(upSegments).fill("..").concat(fileSegments.slice(commonLength));

  return relativeParts.join("/") || ".";
}
