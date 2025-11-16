import { normalizePath } from "./utils";

export function scopeCss(code: string, id: string): string {
  const scope = `data-v-${id}`;

  return code.replace(/([^\r\n,{}]+)(?=\s*{)/g, (selector) => {
    const trimmed = selector.trim();
    if (trimmed.startsWith('@')) return selector;
    return trimmed
      .split(',')
      .map((s) => `${s.trim()}[${scope}]`)
      .join(', ');
  });
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