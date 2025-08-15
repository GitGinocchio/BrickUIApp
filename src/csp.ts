import { scripts, styles } from "@assets/js/externals.js";

export async function buildCsp() {
  const scriptHashesAndUrls = await Promise.all(
    scripts.map(async s => {
      const hash = s.integrity;
      return { src: s.src, hash };
    })
  );

  const styleHashesAndUrls = await Promise.all(
    styles.map(async s => {
      const hash = s.integrity;
      return { href: s.href, hash };
    })
  );

  // Ora crei le direttive CSP
  const scriptSrcs = scriptHashesAndUrls.map(item => `'${item.hash}' ${item.src}`).join(' ');
  const styleSrcs = styleHashesAndUrls.map(item => `'${item.hash}' ${item.href}`).join(' ');

  return [
    `default-src 'self' asset: data: blob: http://asset.localhost ipc: http://ipc.localhost;`,
    `script-src 'self' 'unsafe-eval' ${scriptSrcs}`,
    `style-src 'self' 'unsafe-inline' ${styleSrcs}`,
    `img-src 'self' asset: data: blob: https: http://asset.localhost`,
    `connect-src 'self' ws: http://asset.localhost http://ipc.localhost`,
    `media-src 'self' asset: data: blob: https: http://asset.localhost`,
    `frame-src 'none'`,
    `object-src 'none'`,
    `base-uri 'self'`
  ].join('; ');
}

export async function applyCsp(csp: string) {
  let meta = document.querySelector('meta[http-equiv="Content-Security-Policy"]');
  if (!meta) {
    meta = document.createElement('meta');
    meta.setAttribute('http-equiv', 'Content-Security-Policy');
    document.head.appendChild(meta);
  }

  meta.setAttribute('content', csp);
}
