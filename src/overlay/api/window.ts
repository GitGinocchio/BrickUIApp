export const windowWrapper =  {
    ...window, // tutte le proprietà sicure rimangono accessibili

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