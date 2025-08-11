
export function toCustomElementName(name) {
  // trasforma in kebab-case
  const kebab = name
    .replace(/([a-z])([A-Z])/g, '$1-$2')
    .toLowerCase();

  // se la prima lettera originale non era maiuscola, aggiungo "a-"
  if (name[0] !== name[0].toUpperCase()) {
    return `l-${kebab}`;
  }
  return kebab;
}


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