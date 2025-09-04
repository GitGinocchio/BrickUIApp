/*
export const windowWrapper = { 
    // tutte le proprietà sicure rimangono accessibili 
    ...window, 

    // Blocchi diretti 
    open: (_) => { throw new Error("window.open is disabled in brick context"); }, 
    close: (_) => { throw new Error("window.close is disabled in brick context"); }, 
    fetch: (_) => { throw new Error("window.fetch is disabled in brick context"); }, 
    eval: (_) => { throw new Error("eval is disabled in brick context"); }, 
    // Eventi 
    set onbeforeunload(_) { throw new Error("onbeforeunload is disabled in brick context"); }, 
    set onerror(_) { throw new Error("onerror is disabled in brick context"); }, 
    // Blocchi tramite getter 
    get document() { throw new Error("window.document is disabled in brick context"); }, 
    get localStorage() { throw new Error("localStorage is disabled in brick context"); }, 
    get sessionStorage() { throw new Error("sessionStorage is disabled in brick context"); }, 
    get navigator() { throw new Error("navigator is disabled in brick context"); }, 
    get location() { throw new Error("location is disabled in brick context"); }, 
    get indexedDB() { throw new Error("indexedDB is disabled in brick context"); } 
}
*/

const blockedProps = [
    'onbeforeunload', 'onerror', 'document', 'location', 
    'localStorage', 'sessionStorage', 'indexedDB'
];

const allowedProps: (keyof Window)[] = [
    'innerWidth','innerHeight', 'devicePixelRatio', 'navigator', 'screen', 'requestAnimationFrame',
    'cancelAnimationFrame', 'setTimeout', 'clearTimeout', 'setInterval', 'clearInterval',
    'performance', 'scrollX', 'scrollY', 'pageXOffset', 'pageYOffset'
];

export const windowWrapper = new Proxy(window, {
  get(target, prop: string | symbol, receiver) {
    if (typeof prop === 'string') {
      if (!allowedProps.includes(prop as keyof Window)) {
        throw new Error(`Access to window.${prop} is disabled in brick context`);
      }
    }

    const value = target[prop as keyof Window];

    // Bind delle funzioni al window originale per evitare "Illegal invocation"
    if (typeof value === 'function') {
      return value.bind(target);
    }

    return value;
  },

  set(target, prop, value) {
    if (blockedProps.includes(String(prop))) {
      throw new Error(`Setting window.${String(prop)} is disabled in brick context`);
    }
    return Reflect.set(target, prop, value);
  }
});


