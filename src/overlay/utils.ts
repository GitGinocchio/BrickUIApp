

export async function sendResponseRequest(port: MessagePort, key : string, type : string, payload: any, timeout = 5000) {
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
    setTimeout(() => {
      window.removeEventListener("message", handler);
      reject(new Error(`Timeout waiting for get-${type}`));
    }, timeout);
  });
}