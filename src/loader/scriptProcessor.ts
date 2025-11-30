import { SFCDescriptor } from "@vue/compiler-sfc";
import { sanitizePath } from "../utils/path";

interface ImportBinding {
  isType: boolean;
  imported: string;   // il nome originale esportato
  local: string;      // il nome usato localmente
  source: string;     // da dove viene importato (es: "./Comp.vue")
  isFromSetup: boolean;
  isUsedInTemplate: boolean;
}

export function extractImports(descriptor: SFCDescriptor): ImportBinding[] {
  const bindings: ImportBinding[] = [];

  const scripts = [
    { code: descriptor.script?.content ?? "", isSetup: false },
    { code: descriptor.scriptSetup?.content ?? "", isSetup: true },
  ];

  for (const { code, isSetup } of scripts) {
    const regex = /import\s+(type\s+)?((?:[\w*$\s{},]+?))\s+from\s+['"](.+?)['"]/g;
    let match: RegExpExecArray | null;

    while ((match = regex.exec(code))) {
      const isType = !!match[1];
      const importPart = match[2].trim();
      const source = match[3];

      // Default + named import: import Foo, { bar as baz } from "..."
      let defaultImport = "";
      let namedPart = "";

      if (importPart.includes("{")) {
        const [first, rest] = importPart.split("{");
        defaultImport = first.replace(",", "").trim();
        namedPart = "{" + rest;
      } else {
        defaultImport = importPart;
      }

      // default import
      if (defaultImport) {
        bindings.push({
          isType,
          imported: "default",
          local: defaultImport,
          source,
          isFromSetup: isSetup,
          isUsedInTemplate: false,
        });
      }

      // named imports
      if (namedPart) {
        const inside = namedPart.replace(/^{|}$/g, "").trim();
        const names = inside.split(",").map(s => s.trim());

        for (const n of names) {
          if (!n) continue;

          if (n.includes(" as ")) {
            const [imported, local] = n.split(" as ").map(s => s.trim());
            bindings.push({
              isType,
              imported,
              local,
              source,
              isFromSetup: isSetup,
              isUsedInTemplate: false,
            });
          } else {
            bindings.push({
              isType,
              imported: n,
              local: n,
              source,
              isFromSetup: isSetup,
              isUsedInTemplate: false,
            });
          }
        }
      }

      // namespace import (* as X)
      if (importPart.startsWith("* as ")) {
        const local = importPart.replace("* as", "").trim();
        bindings.push({
          isType,
          imported: "*",
          local,
          source,
          isFromSetup: isSetup,
          isUsedInTemplate: false,
        });
      }
    }
  }

  return bindings;
}

export async function rewriteImports(
  code: string,
  moduleCache: Record<string, any>,
  brickDirName: string
): Promise<string> {
  async function moduleVar(moduleName: string): Promise<string> {
    if (moduleName.startsWith("./")) {
      for (const [path, _] of Object.entries(moduleCache)) {
        if (path.endsWith(moduleName.replace("./", ""))) {
          moduleName = path;
          break;
        }
      }
    } else if (moduleName.startsWith("/")) {
      moduleName = await sanitizePath(moduleName, { root: brickDirName, convertToAssetURL: false });
    }

    return moduleCache[moduleName]
      ? `moduleCache["${moduleName}"]`
      : moduleName;
  }

  // Helper async per replace
  async function replaceAsync(
    str: string,
    regex: RegExp,
    asyncFn: (...args: any[]) => Promise<string>
  ): Promise<string> {
    const matches = Array.from(str.matchAll(regex));
    const results = await Promise.all(
      matches.map((m) => asyncFn(...m))
    );

    let out = str;
    let offset = 0;
    matches.forEach((m, i) => {
      out = out.slice(0, m.index! + offset) + results[i] + out.slice(m.index! + offset + m[0].length);
      offset += results[i].length - m[0].length;
    });

    return out;
  }

  // import { a, b as c } from "vue"
  code = await replaceAsync(code, /import\s+\{([^}]+)\}\s+from\s+['"]([^'"]+)['"];?/g,
    async (_, imports, moduleName) => {
      const names = imports
        .split(",")
        .map(s => s.trim())
        .filter(Boolean)
        .map(s => s.includes(" as ") ? s.split(" as ").map(x => x.trim()).join(": ") : s)
        .join(", ");
      return `const { ${names} } = ${await moduleVar(moduleName)};`;
    }
  );

  // import defaultExport from "vue"
  code = await replaceAsync(code, /import\s+([a-zA-Z0-9_$]+)\s+from\s+['"]([^'"]+)['"];?/g,
    async (_, defaultName, moduleName) => {
      const modVar = await moduleVar(moduleName);
      return `const ${defaultName} = ${modVar}.default ?? ${modVar};`;
    }
  );

  // import * as name from "vue"
  code = await replaceAsync(code, /import\s+\*\s+as\s+([a-zA-Z0-9_$]+)\s+from\s+['"]([^'"]+)['"];?/g,
    async (_, namespace, moduleName) => {
      return `const ${namespace} = ${await moduleVar(moduleName)};`;
    }
  );

  // import defaultExport, { a, b as c } from "vue"
  code = await replaceAsync(code, /import\s+([a-zA-Z0-9_$]+)\s*,\s*\{([^}]+)\}\s+from\s+['"]([^'"]+)['"];?/g,
    async (_, defaultName, imports, moduleName) => {
      const modVar = await moduleVar(moduleName);
      const names = imports
        .split(",")
        .map(s => s.trim())
        .filter(Boolean)
        .map(s => s.includes(" as ") ? s.split(" as ").map(x => x.trim()).join(": ") : s)
        .join(", ");
      return `const ${defaultName} = ${modVar}.default ?? ${modVar}; const { ${names} } = ${modVar};`;
    }
  );

  return code;
}
