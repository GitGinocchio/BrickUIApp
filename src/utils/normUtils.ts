import { convertFileSrc } from "@tauri-apps/api/core";
import { appDataDir as getAppDataDir } from "@tauri-apps/api/path";
import { Prop } from "interfaces/brick";

const appDataDir = await getAppDataDir();

export function normalizeProps(propsArray: Prop[]): Record<string, any> {
  return Object.fromEntries(
    propsArray.map((p) => [p.prop_name, p.value ?? p.default])
  );
}

function normalizeRelativeSegments(p: string, root: string): string {
  const isRootUrl = /^https?:\/\//i.test(root) || /^asset:/.test(root);

  const stack: string[] = [];
  let rootParts = [];

  if (!/^([a-zA-Z]:)?[\\/]/.test(p)) {
    // path relativo → inizializza con root
    const rootParts = root.replace(/[/\\]+$/, "").split(/[/\\]+/);
    stack.push(...rootParts);
  }

  const parts = p.split(/[/\\]+/);

  for (const part of parts) {
    if (part === "" || part === ".") continue;
    if (part === "..") {
      if (stack.length > rootParts.length) stack.pop(); // non uscire dalla root
    } else {
      stack.push(part);
    }
  }

  const normalized = stack.join("/");
  return isRootUrl ? normalized : normalized.replace(/\\/g, "/");
}

export function normalizePath(
  path: string,
  options?: { protocol?: string; root?: string },
  useConvertFileSrc: boolean = true
) {
  const protocol = options?.protocol ?? "asset";
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
  console.log("normalized", normalized);
  return useConvertFileSrc ? convertFileSrc(normalized, protocol) : normalized;
}
