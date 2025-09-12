

export function rewriteImports(code: string, moduleCache: Record<string, any>): string {
  function moduleVar(moduleName: string): string {
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
      return `const ${defaultName} = ${moduleVar(moduleName)}.default ?? ${moduleVar(moduleName)}; const { ${names} } = ${moduleVar(moduleName)};`;
    }
  );

  return code;
}