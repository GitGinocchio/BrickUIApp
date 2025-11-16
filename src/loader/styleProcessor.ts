import { normalizePath } from "./utils";

export function scopeCss(code: string, id: string): string {
  const scope = `data-v-${id}`;

  // Rinomina i keyframe: @keyframes name → @keyframes name-data-v-xxxx
  code = code.replace(
    /@keyframes\s+([a-zA-Z0-9-_]+)/g,
    (_, name) => `@keyframes ${name}-${scope}`
  );

  // Rinomina anche le dichiarazioni "animation: name ..."
  code = code.replace(
    /animation:\s*([a-zA-Z0-9-_]+)/g,
    (_, name) => `animation: ${name}-${scope}`
  );

  // Normale scoping dei selettori
  code = code.replace(/([^\r\n,{}]+)(?=\s*{)/g, (selector) => {
    const trimmed = selector.trim();

    // Non scopa:
    if (
      trimmed.startsWith('@') ||  // @rules
      /^\d+%$/.test(trimmed) ||   // keyframe steps
      trimmed === 'from' ||
      trimmed === 'to'
    ) {
      return selector;
    }

    // Aggiunge [data-v-xxxx]
    return trimmed
      .split(',')
      .map((s) => `${s.trim()}[${scope}]`)
      .join(', ');
  });

  return code;
}



export function processStyle(source: string, lang: string = "css", componentPath: string, id: string) {
    source = scopeCss(source, id);

    return source.replace(/url\((['"]?)(.+?)\1\)/g, (match, quote, path: string) => {
        // Se è assoluto, non toccarlo
        if (path.startsWith('https://')) {
            return match;
        }
        
        return `url(${normalizePath(path, { root: componentPath })})`;
    });
}