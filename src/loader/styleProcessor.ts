import { normalizePath } from "./utils";

export function processStyle(source: string, lang: string = "css", componentPath: string) {
    return source.replace(/url\((['"]?)(.+?)\1\)/g, (match, quote, path: string) => {
        // Se è assoluto, non toccarlo
        if (path.startsWith('https://')) {
            return match;
        }
        
        return `url(${normalizePath(path, { root: componentPath })})`;
    });
}