
- [ ] Rimuovere l'alt+f4 sulla finestra dell'overlay

- [ ] Creare un header comune a tutte le pagine

- [ ] Forse bisognerebbe fare che quando una persona apre l'app per la prima volta
  la prima cosa che deve fare e' creare un wall (un tema) da li in poi potra' creare i brick
  e quando si crea un brick verra' chiesto in quale wall si vuole mettere quel brick

- [ ] In ogni Wall dovra' esserci una validazione e controllo a runtime per i brick, in modo che non collidano
  o competano tra di loro o che se dovesse capitare che sia una cosa voluta dal creatore

- [ ] Le persone potranno condividere i brick in due modi:
  1. tramite link ad una repository github 
    (quindi il brick viene importato e preso da internet, puo' essere copiato, ma perde gli aggiornamenti in caso ce ne fossero)
  2. tramite un file .brick (un brick compresso che contiene brick.yml e brick.vue) scaricato e importato nell'app
    (questo diventa modificabile ma perde gli update dalla versione online)

Colore del logo:
- #cb4153ff

- [x] Modificare i prop di tipo collections per far si che esista solo Array e Select
  e all'interno di questi si puo' decidere se devono essere di interi, float, stringhe, ecc.

- [ ] Esempio di implementazione per la finestra background per gli sfondi animati:
  ```rust
  use windows::Win32::Foundation::*;
  use windows::Win32::UI::WindowsAndMessaging::*;
  use wry::{Application, Attributes, webview::WebViewBuilder};

  fn main() -> wry::Result<()> {
      // Step 1: Trova Progman
      let progman = unsafe { FindWindowA("Progman", None) };

      // Step 2: Manda il messaggio per creare WorkerW
      unsafe { SendMessageA(progman, 0x052C, WPARAM(0), LPARAM(0)) };

      // Step 3: Trova la finestra WorkerW
      let mut workerw = HWND(0);
      let mut hwnd = unsafe { FindWindowExA(HWND(0), HWND(0), "WorkerW", None) };

      while hwnd.0 != 0 {
          let shell_view = unsafe { FindWindowExA(hwnd, HWND(0), "SHELLDLL_DefView", None) };
          if shell_view.0 != 0 {
              workerw = hwnd;
              break;
          }
          hwnd = unsafe { FindWindowExA(HWND(0), hwnd, "WorkerW", None) };
      }

      if workerw.0 == 0 {
          println!("WorkerW non trovato!");
          return Ok(());
      }

      // Step 4: Crea l'app Wry
      let app = Application::new()?;

      // Step 5: Crea la finestra web come child di WorkerW
      let window = app.add_window(Attributes {
          title: "Web Wallpaper".to_string(),
          width: 1920,
          height: 1080,
          decorations: false,      // senza bordi
          visible: true,
          parent: Some(workerw.0 as *mut _), // agganciata a WorkerW
          ..Default::default()
      })?;

      // Step 6: Costruisci il WebView
      let _webview = WebViewBuilder::new(window)?
          .with_url("https://example.com")?   // la pagina web che vuoi mostrare
          .build()?;

      app.run()?;

      Ok(())
  }
  ```

- [ ] I tipi nei bricks devono essere presi dalla cartella resource al posto di essere copiati ogni volta...

- [ ] Da usare il plugin Opener per aprire file e url in app.

- [x] Da utilizzare il plugin Single Instance per essere sicuri che ci sia solo un istanza dell'app avviata

- [x] Utilizzare anche i plugin Autostart

- [ ] Utilizzare anche il plugin Deep linking

- [x] Forse si puo' utilizzare il contesto isolato:
  
  ```json
  // tauri.conf.json
  "security"
    "pattern": {
    "use":"isolation",
    "options" : {
      "dir": ""
    }
  },
  ```

- [x] Usare questo esempio di chatgpt per suddividere BrickCard in diversi componenti:
  ```
  components/
    BrickCard.vue
    BrickHeader.vue
    BrickDescription.vue
    BrickProps.vue
    BrickEmits.vue
    modals/
      DeleteBrickModal.vue
      DeletePropModal.vue
      PropModal.vue
  composables/
    useBrickProps.ts
    useBrickPersistence.ts
  ```


- [x] La descrizione se modificata con l'icona della penna a destra alla fine della modifica non viene modificata veramente

- [ ] Aggiungere il prop type regex

- [x] Al proptype color manca il parametro skip alpha e gli swatches (che possono essere fatti come avevo gia' fatto per la selezione dell'utente...)

- Ai proptype di tipo collections manca un parametro per dire il valore minimo e massimo dei valori all'interno 
  (per quanto riguarda le stringhe potrebbe diventare la lunghezza di esse)

- [x] Ai proptype di tipo select manca un vero limite per quanto riguarda la quantita' dei valori selezionati...

- [x] la tendina per la scelta dei proptype dovrebbe avere delle icone riconoscibili per ogni proptype...

- [ ] Finire di coprire tutti i tipi di prop per quanto riguarda la modifica e la creazione

- [x] Il proptype Any sarebbe da rimuovere in quanto realmente inutilizzabile (forse)
  per renderlo piu' utilizzabile si potrebbe utilizzare una textarea al posto di una input singleline...

- [x] Aggiungere la funzionalita' di eliminare i prop

- Sistemare la pagina dei settings

- (inutile)
  Inviare un avviso quando si duplica un prop in quanto attivando due prop uguali si potrebbero andare in contro a dei problemi di navigazione

- Quando si sta modificando un prop select, e si toglie un opzione che e' attualmente selezionata come default questa rimane nel default
  il default in quel caso dovrebbe tornare vuoto, senza un valore (solo in quel caso)

- [x] Sostituire Edit nei tre puntini del brick con dei pulsanti edit sui campi come name, description, props, ecc.

- [x] Non esporre direttamente le api di tauri come invoke, ma creare un wrapper tipizzato con metodi specifici (per aiutare anche l'utente)

- Errore durante l'ottenimento del percorso dell'eseguibile: Accesso negato. (0x80070005), questo avviene quando si cerca di ottenere l'icona
  di un processo avviato come amministratore quando il nostro non lo e'

- [x] Anche la descrizione del prop dovrebbe supportare markdown

- [ ] Aggiungere i proptype: bool, date, time, datetime, color, shortcut (Anche a UI)

- Aggiungere la possibilita' negli input di tipo stringa di essere validati attraverso un regex

- (Vedere se ha ancora senso implementarlo)
  Sistemare i permessi per l'api tauri magari con un token generato a runtime che deve essere scambiato per emettere o ascoltare eventi particolari che solo la finestra overlay puo' ascoltare

- Problema con le cartelle per i nomi dei brick perche' possono essere uguali infatti taskBar e TaskBar vengono viste allo stesso modo
  questo e' un problema da risolvere e si potrebbe sistemare con il kebab-case (non e' vero ma dettagli)

- Gestire il caso in cui un'altra app e' a schermo intero, quindi tutti i brick (o quelli configurati per farlo) devono essere tolti

- [x] Creare un wrapper per l'api `listen` di tauri, (come e' stato gia' fatto in `src/overlay/apis.ts`)
  di modo da permettere all'utente di ascoltare solo alcuni eventi

- [x] Bisogna aggiungere nel backend rust in `src-tauri/src/state.rs` la creazione del file `settings.yml`
  dal template `settings-template.yml` in `src-tauri/assets/settings-template.yml`

- [ ] Bisogna aggiungere la funzionalita' che quando viene modificato un brick vengono ricaricati tutti i brick nuovamente
  (senza il bisogno di riavviare l'app)
  e aggiungere un pulsante "ricarica brick"
  magari si puo' mettere come modalita' "dev" per chi crea brick

- [x] Bisogna aggiungere la possibilita' di creare un nuovo brick in app (quindi copiare il template `/src-tauri/assets/brick-template` nella cartella bricks)
  e dare la possibilita di aprire la cartella per sviluppare il brick (successivamente anche per l'editor visuale)

- (Questo non ha molto senso pensandoci di nuovo, potrebbe creare confusione e sarebbe difficile da gestire per l'utente)
  Implementare i global bricks (cartella `/globals` nella directory in AppData)
  cioe' dei brick riutilizzabili all'interno di altri brick
  (quindi rendere impossibile l'utilizzo di brick nella cartella bricks all'interno di altri bricks)

- [x] Bisogna implementare la possibilita' di abilitare e disabilitare i brick a runtime (non dovrebbe essere difficile)

- [x] Bisogna creare la logica per il caricamento dei dati per ogni brick (da mostrare anche nella pagina apposita)
  quindi dal file `brick.yml` (guardare anche il template in `src-tauri/assets/brick-template`)

- [x] Implementare anche il fatto che ogni elemento in `defineProps` presente in un brick deve essere visto come 
  delle impostazioni per il relativo brick e quando vengono modificate nei settings devono cambiare in tempo reale

- Il caricamento di dati dovrebbe avvenire sempre (o il piu' possibile) dal backend Rust
  questo perche' piu' sicuro e perche' i dati possono essere mantenuti nello stato generale dell'app
  e possono essere passati al frontend con dei metodi `invoke` in modo facile

- [x] Cercare di capire e risolvere l'ottenimento delle applicazioni nella taskbar nativa di windows
  e rendere disponibile il comando con invoke

- [x] Creare la logica Rust per l'invio di eventi globali (mouse, click, ecc.) che magari possono essere utilizzati dall'utente

- (non serve al momento)
  Al momento la creazione delle finestre la lasciamo a tauri
  piu avanti sarebbe meglio utilizzare `overlay::window::Overlay` per l'overlay

Ho modificato questo in package.json, sarebbe da rimettere per controlli piu' stringenti su typescript
```json
"build": "vue-tsc --noEmit && vite build"
```