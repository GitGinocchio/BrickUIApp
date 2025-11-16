
const blockedProps = [
  "onbeforeunload",
  "onerror",
  "document",
  "location",
  "localStorage",
  "sessionStorage",
  "indexedDB",
  "fetch",
];

const allowedProps: (keyof Window)[] = [
  "innerWidth",
  "innerHeight",
  "devicePixelRatio",
  "navigator",
  "screen",
  "requestAnimationFrame",
  "cancelAnimationFrame",
  "setTimeout",
  "clearTimeout",
  "setInterval",
  "clearInterval",
  "performance",
  "scrollX",
  "scrollY",
  "pageXOffset",
  "pageYOffset",
  "addEventListener",
  "removeEventListener"
];

export const windowWrapper = new Proxy(window, {
  get(target, prop: string | symbol, _receiver) {
    if (typeof prop === "string") {
      if (!allowedProps.includes(prop as keyof Window)) {
        throw new Error(
          `Access to window.${prop} is disabled in brick context`
        );
      }
    }

    const value = target[prop as keyof Window];

    // Bind delle funzioni al window originale per evitare "Illegal invocation"
    if (typeof value === "function") {
      return value.bind(target);
    }

    return value;
  },

  set(target, prop, value) {
    if (blockedProps.includes(String(prop))) {
      throw new Error(
        `Setting window.${String(prop)} is disabled in brick context`
      );
    }
    return Reflect.set(target, prop, value);
  },
});
