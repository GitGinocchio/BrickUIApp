let key;
let port;
let handlers = {};

function waitForInitMessage() {
  return new Promise((resolve, reject) => {
    const handler = (event) => {
      if (!event.data.key || event.data.type !== "init") return;
      if (!event.ports || !(event.ports.length > 0)) return;

      window.removeEventListener("message", handler);

      resolve(event);
    };

    window.addEventListener("message", handler, { once: true });
    setTimeout(() => reject(new Error("Timeout init")), 10000);
  });
}

function onMessage(event) {
  if (event.data.key !== key) return;

  const listeners = handlers[event.data.type];
  if (listeners && listeners.length > 0) {
    for (const cb of listeners) {
      cb(event, port);
    }
  }
}

export function on(type, callback) {
  if (!handlers[type]) { handlers[type] = []; }
  handlers[type].push(callback);
}

export function off(type, callback) {
  if (!handlers[type]) return;
  handlers[type] = handlers[type].filter(cb => cb !== callback);
}

export async function sendResponseRequest(type, payload, timeout = 5000) {
  return new Promise((resolve, reject) => {
    // Listener temporaneo per il messaggio di risposta
    const handler = (event) => {
      const data = event.data;

      // Controllo chiave e tipo messaggio
      if (!data || data.key !== key || data.type !== type) return;

      // Risolvo la promise con i dati ricevuti
      resolve(data.payload);

      // Rimuovo il listener appena ricevuta la risposta
      port.removeEventListener("message", handler);
    };

    port.addEventListener("message", handler);

    // Invio la richiesta
    port.postMessage({ type: `get-${type}`, key, payload });

    // Timeout per evitare che la promise resti pendente per sempre
    const timer = setTimeout(() => {
      port.removeEventListener("message", handler);
      reject(new Error(`Timeout waiting for get-${type}`));
    }, timeout);
  });
}

export async function getFileContent(path) {
  return (await sendResponseRequest("file-content", { path })).content
}

export async function getFetchResult(url) {
  return await sendResponseRequest("fetch-result", { url })
}

export async function getAppDataDir() {
  return (await sendResponseRequest("app-data-dir", {})).path
}

export async function convertFileSrc(path) {
  return (await sendResponseRequest("convert-file-src", { path })).path
}

export async function initBridge(customHandlers = {}) {
  const event = await waitForInitMessage();
  key = event.data.key;
  port = event.ports[0];

  for (const [type, cb] of Object.entries(customHandlers)) {
    on(type, cb);
  }

  port.onmessage = onMessage;
  port.onmessageerror = (e) => console.error(e);

  console.log("Handshake completed successfully!");
  return { key, port, ...event.data };
}