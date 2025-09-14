import { SFCDescriptor } from "@vue/compiler-sfc";
import { normalizePath } from "./utils";

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

export function rewriteImports(code: string, moduleCache: Record<string, any>, brickDirName: string): string {
  function moduleVar(moduleName: string): string {
    if (moduleName.startsWith('./')) {
      moduleName = normalizePath(moduleName, { root: brickDirName }, false);
    }

    // se il modulo è presente nel moduleCache, usalo
    return moduleCache[moduleName]
      ? `moduleCache["${moduleName}"]`
      : moduleName; // fallback
  }

  // import { a, b as c } from "vue"
  code = code.replace(
    /import\s+\{([^}]+)\}\s+from\s+['"]([^'"]+)['"];?/g,
    (_, imports, moduleName) => {
      const names = imports
        .split(",")
        .map(s => s.trim())
        .filter(Boolean)
        .map(s => {
          if (s.includes(" as ")) {
            const [orig, alias] = s.split(" as ").map(x => x.trim());
            return `${orig}: ${alias}`;
          }
          return s;
        })
        .join(", ");
      return `const { ${names} } = ${moduleVar(moduleName)};`;
    }
  );

  // import defaultExport from "vue"
  code = code.replace(
    /import\s+([a-zA-Z0-9_$]+)\s+from\s+['"]([^'"]+)['"];?/g,
    (_, defaultName, moduleName) => {
      return `const ${defaultName} = ${moduleVar(moduleName)}.default ?? ${moduleVar(moduleName)};`;
    }
  );

  // import * as name from "vue"
  code = code.replace(
    /import\s+\*\s+as\s+([a-zA-Z0-9_$]+)\s+from\s+['"]([^'"]+)['"];?/g,
    (_, namespace, moduleName) => {
      return `const ${namespace} = ${moduleVar(moduleName)};`;
    }
  );

  // import defaultExport, { a, b as c } from "vue"
  code = code.replace(
    /import\s+([a-zA-Z0-9_$]+)\s*,\s*\{([^}]+)\}\s+from\s+['"]([^'"]+)['"];?/g,
    (_, defaultName, imports, moduleName) => {
      const names = imports
        .split(",")
        .map(s => s.trim())
        .filter(Boolean)
        .map(s => {
          if (s.includes(" as ")) {
            const [orig, alias] = s.split(" as ").map(x => x.trim());
            return `${orig}: ${alias}`;
          }
          return s;
        })
        .join(", ");
      return `const ${defaultName} = ${moduleVar(moduleName)}.default ?? ${moduleVar(moduleName)}; 
              const { ${names} } = ${moduleVar(moduleName)};`;
    }
  );

  return code;
}