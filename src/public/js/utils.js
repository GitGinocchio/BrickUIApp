
export function appendScript(src, type = null, async = false, defer = false, integrity, crossorigin) {
  return new Promise((resolve, reject) => {
    if (document.querySelector(`script[src="${src}"]`)) {
      return resolve();
    }

    const script = document.createElement("script");
    script.src = src;
    script.async = async;
    script.defer = defer;
    if (type) script.type = type;
    if (integrity) script.integrity = integrity;
    if (crossorigin) script.crossOrigin = crossorigin;

    script.onload = () => resolve();
    script.onerror = () => reject(new Error(`Failed to load script: ${src}`));

    document.body.appendChild(script);
  });
}
/*
export async function appendScript(src, type = 'module', async = false, defer = false, integrity, crossorigin) {
  // evita di caricare due volte lo stesso script
  if (document.querySelector(`script[data-src="${src}"]`)) return;

  try {
    // fetch del contenuto del file
    const res = await fetch(src);
    if (!res.ok) throw new Error(`Failed to fetch script: ${src}`);

    const code = await res.text();

    // crea un Blob con MIME corretto
    const blob = new Blob([code], { type: 'application/javascript' });
    const blobUrl = URL.createObjectURL(blob);

    return new Promise((resolve, reject) => {
      const script = document.createElement('script');
      script.src = blobUrl;
      script.async = async;
      script.defer = defer;
      script.type = type;
      script.dataset.src = src; // identifica lo script originale
      if (integrity) script.integrity = integrity;
      if (crossorigin) script.crossOrigin = crossorigin;

      script.onload = () => {
        URL.revokeObjectURL(blobUrl); // libera la memoria del Blob
        resolve();
      };
      script.onerror = () => reject(new Error(`Failed to load script: ${src}`));

      document.body.appendChild(script);
    });
  } catch (e) {
    throw new Error(`Error loading script: ${e.message}`);
  }
}
*/


export function appendStyle(href, rel = "stylesheet", media = null, integrity = null, crossorigin = null) {
  return new Promise((resolve, reject) => {
    const link = document.createElement("link");
    link.rel = rel;
    link.href = href;
    if (media) link.media = media;
    if (integrity) link.integrity = integrity;
    if (crossorigin) link.crossOrigin = crossorigin;

    link.onload = () => resolve();
    link.onerror = () => reject(new Error(`Failed to load stylesheet: ${href}`));

    document.head.appendChild(link);
  });
}

export function normalizeProps(propsArray) {
  return Object.fromEntries(
    propsArray.map(p => [p.prop_name, p.value ?? p.default])
  );
}
