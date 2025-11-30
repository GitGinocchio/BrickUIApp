
export function capitalize(str: string): string {
  if (!str) return '';
  return str.charAt(0).toUpperCase() + str.slice(1);
}

export function deepEqual(a: any, b: any): boolean {
  if (a === b) return true;

  if (typeof a !== "object" || typeof b !== "object" || a == null || b == null) {
    return false;
  }

  const keysA = Object.keys(a);
  const keysB = Object.keys(b);

  if (keysA.length !== keysB.length) return false;

  return keysA.every(key => deepEqual(a[key], b[key]));
}

export function waitUntil(cond: () => boolean, interval = 100): Promise<void> {
  return new Promise(resolve => {
    const check = () => {
      if (cond()) {
        resolve();
      } else {
        setTimeout(check, interval);
      }
    };
    check();
  });
}

export async function replaceAsync(
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