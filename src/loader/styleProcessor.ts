import { sanitizePath } from "#utils/path";

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

export async function processStyle(source: string, _lang: string = "css", componentPath: string, id: string) {
  source = scopeCss(source, id);

  const parts = [];
  let lastIndex = 0;
  const regex = /url\((['"]?)(.+?)\1\)/g;
  let match;

  while ((match = regex.exec(source)) !== null) {
    const [full, _quote, path] = match;
    parts.push(source.slice(lastIndex, match.index));

    if (path.startsWith("https://")) {
      parts.push(full);
    } else {
      const sanitized = await sanitizePath(path, { root: componentPath });
      parts.push(`url(${sanitized})`);
    }

    lastIndex = match.index + full.length;
  }

  parts.push(source.slice(lastIndex));

  return parts.join("");
}