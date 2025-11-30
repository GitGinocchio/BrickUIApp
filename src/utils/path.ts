import { convertFileSrc } from "@tauri-apps/api/core";
import { extname, appDataDir as getAppDataDir, resolve, dirname, basename } from "@tauri-apps/api/path";

export const appDataDir = await getAppDataDir();
export { extname, basename, resolve, dirname };

export function isURL(str: string): boolean {
  return /^[a-zA-Z][a-zA-Z0-9+.-]*:\/\//.test(str);
}

export function isTauriAssetURL(url: string): boolean {
  if (!isURL(url)) return false;
  try {
    const u = new URL(url);
    return u.protocol === "asset:";
  } catch {
    return false;
  }
}

export function isSecureURL(url: string): boolean {
  if (!isURL(url)) return false;
  try {
    return new URL(url).protocol === "https:";
  } catch {
    return false;
  }
}

export function safeDecodeURI(str: string): string {
  try { return decodeURIComponent(str); } catch { return str; }
}

export function normalizeURLPathname(pathname: string): string {
  const parts = pathname.split("/"); 
  const stack: string[] = [];
  for (const p of parts) {
    if (!p || p === ".") continue;
    if (p === "..") { if (stack.length > 0) stack.pop(); continue; }
    stack.push(p);
  }
  return "/" + stack.join("/");
}

export function normalizeURL(url: string): string {
  try {
    const u = new URL(url);
    u.pathname = normalizeURLPathname(u.pathname);
    return u.toString();
  } catch {
    return url; // fallback se non è URL valido
  }
}

export async function ensureInsideRoot(path: string, root: string): Promise<string> {
  const isRootUrl = /^https?:\/\//i.test(root) || /^asset:\/\//i.test(root);

  let rootPath = root;
  if (isRootUrl) {
    try { rootPath = new URL(root).pathname; } catch {}
  }
  rootPath = rootPath.replace(/[/\\]+$/, "");
  const rootParts = rootPath.split(/[/\\]+/).filter(Boolean);
  const stack: string[] = [...rootParts];

  // Se path è assoluto Windows o Unix → torna subito
  if (/^[a-zA-Z]:[\\/]/.test(path) || path.startsWith("/")) {
    return path.replace(/\\/g, "/").replace(/\/+$/, "");
  }

  // Path relativo → normalizza dentro root
  path = path.replace(/^[/\\]+/, "");
  const parts = path.split(/[/\\]+/);
  for (const part of parts) {
    if (!part || part === ".") continue;
    if (part === "..") { if (stack.length > rootParts.length) stack.pop(); continue; }
    stack.push(part);
  }

  const joined = stack.join("/");
  return isRootUrl ? joined : joined.replace(/\\/g, "/");
}

export function relative(filePath: string, root: string): string {
  const decodedFilePath = decodeURIComponent(filePath).replace(/\\/g, "/").replace(/\/+$/, "");
  const decodedRoot = decodeURIComponent(root).replace(/\\/g, "/").replace(/\/+$/, "");
  const fileSegments = decodedFilePath.split("/").filter(Boolean);
  const rootSegments = decodedRoot.split("/").filter(Boolean);

  let commonLength = 0;
  for (; commonLength < Math.min(fileSegments.length, rootSegments.length); commonLength++) {
    if (fileSegments[commonLength] !== rootSegments[commonLength]) break;
  }

  const upSegments = rootSegments.length - commonLength;
  const relativeParts = Array(upSegments).fill("..").concat(fileSegments.slice(commonLength));
  return relativeParts.join("/") || ".";
}

export async function sanitizePath(
  path: string,
  options?: {
    root?: string,
    allowedExtensions?: string[],
    allowDirectories?: boolean,
    convertToAssetURL?: boolean,
    allowExternalURLs?: boolean,
    allowOnlyHTTPS?: boolean
  }
): Promise<string> {
  const root = options?.root ?? appDataDir;
  const convertToAssetURL = options?.convertToAssetURL ?? true;
  const allowDirectories = options?.allowDirectories ?? false;
  const allowedExtensions = options?.allowedExtensions?.map(e => e.toLowerCase()) ?? [];
  const allowExternalURLs = options?.allowExternalURLs ?? true;
  const allowOnlyHTTPS = options?.allowOnlyHTTPS ?? true;

  let decoded = safeDecodeURI(path);
  const isUrl = isURL(decoded);
  const isAsset = isTauriAssetURL(decoded);

  // Percorso locale / asset Tauri
  if (!isUrl || isAsset) {
    if (isAsset) {
      const url = new URL(decoded);
      decoded = url.pathname.replace(/^\/+/, "");
    }

    const safePath = await ensureInsideRoot(decoded, root);

    if (allowedExtensions.length > 0) {
      const ext = (await extname(safePath)).toLowerCase();
      if (!allowDirectories && ext === "") throw new Error(`Path must point to a file: ${path}`);
      if (ext && !allowedExtensions.includes(ext)) throw new Error(`File extension not allowed: ${ext}`);
    }

    return convertToAssetURL ? convertFileSrc(safePath) : safePath;
  }

  // URL esterno
  if (!allowExternalURLs) throw new Error(`External URLs are not allowed: ${path}`);
  if (allowOnlyHTTPS && !isSecureURL(decoded)) throw new Error(`Only HTTPS URLs are allowed: ${path}`);

  return normalizeURL(decoded);
}
