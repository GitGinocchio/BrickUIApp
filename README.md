

- Usare questo esempio di chatgpt per suddividere BrickCard in diversi componenti:
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

- Aggiungere il prop type regex

- Al proptype color manca il parametro skip alpha e gli swatches (che possono essere fatti come avevo gia' fatto per la selezione dell'utente...)

- Ai proptype di tipo collections manca un parametro per dire il valore minimo e massimo dei valori all'interno 
  (per quanto riguarda le stringhe potrebbe diventare la lunghezza di esse)

- Ai proptype di tipo select manca un vero limite per quanto riguarda la quantita' dei valori selezionati...

- la tendina per la scelta dei proptype dovrebbe avere delle icone riconoscibili per ogni proptype...

- Finire di coprire tutti i tipi di prop per quanto riguarda la modifica e la creazione

- Il proptype Any sarebbe da rimuovere in quanto realmente inutilizzabile (forse)
  per renderlo piu' utilizzabile si potrebbe utilizzare una textarea al posto di una input singleline...

- [x] Aggiungere la funzionalita' di eliminare i prop

- Sistemare la pagina dei settings

- Inviare un avviso quando si duplica un prop in quanto attivando due prop uguali si potrebbero andare in contro a dei problemi di navigazione

- Inviare un avviso quando si modifica un prop e si cambia il tipo di prop, in quanto cambiare il tipo di prop causa a tutti i dati presenti prima
  di essere persi

- Quando si sta modificando un prop select, e si toglie un opzione che e' attualmente selezionata come default questa rimane nel default
  il default in quel caso dovrebbe tornare vuoto, senza un valore (solo in quel caso)

- [x] Sostituire Edit nei tre puntini del brick con dei pulsanti edit sui campi come name, description, props, ecc.

- Non esporre direttamente le api di tauri come invoke, ma creare un wrapper tipizzato con metodi specifici (per aiutare anche l'utente)

- Errore durante l'ottenimento del percorso dell'eseguibile: Accesso negato. (0x80070005), questo avviene quando si cerca di ottenere l'icona
  di un processo avviato come amministratore quando il nostro non lo e'

- Passare da usare solo `serde_yaml` a `yaml-rust` per la formattazione del file yaml, magari utilizzando un serializzatore custom

- [x] Anche la descrizione del prop dovrebbe supportare markdown

- Aggiungere i proptype: bool, date, time, datetime, color, shortcut

- Aggiungere la possibilita' negli input di tipo stringa di essere validati attraverso un regex

- Sistemare i permessi per l'api tauri magari con un token generato a runtime che deve essere scambiato per emettere o ascoltare eventi particolari che solo la finestra overlay puo' ascoltare

- Problema con le cartelle per i nomi dei brick perche' possono essere uguali infatti taskBar e TaskBar vengono viste allo stesso modo
  questo e' un problema da risolvere e si potrebbe sistemare con il kebab-case

- Gestire il caso in cui un'altra app e' a schermo intero, quindi tutti i brick (o quelli configurati per farlo) devono essere tolti

- [x] Creare un wrapper per l'api `listen` di tauri, (come e' stato gia' fatto in `src/overlay/apis.ts`)
  di modo da permettere all'utente di ascoltare solo alcuni eventi

- [x] Bisogna aggiungere nel backend rust in `src-tauri/src/state.rs` la creazione del file `settings.yml`
  dal template `settings-template.yml` in `src-tauri/assets/settings-template.yml`

- Bisogna aggiungere la funzionalita' che quando viene modificato un brick vengono ricaricati tutti i brick nuovamente
  (senza il bisogno di riavviare l'app)
  magari si puo' mettere come modalita' "dev" per chi crea brick

- Bisogna aggiungere la possibilita' di creare un nuovo brick in app (quindi copiare il template `/src-tauri/assets/brick-template` nella cartella bricks)
  e dare la possibilita di aprire la cartella per sviluppare il brick (successivamente anche per l'editor visuale)

- (Questo non ha molto senso pensandoci di nuovo, potrebbe creare confusione e sarebbe difficile da gestire per l'utente)
  Implementare i global bricks (cartella `/globals` nella directory in AppData)
  cioe' dei brick riutilizzabili all'interno di altri brick
  (quindi rendere impossibile l'utilizzo di brick nella cartella bricks all'interno di altri bricks)

- [x] Bisogna implementare la possibilita' di abilitare e disabilitare i brick a runtime (non dovrebbe essere difficile)

- Bisogna creare la logica per il caricamento dei dati per ogni brick (da mostrare anche nella pagina apposita)
  quindi dal file `brick.yml` (guardare anche il template in `src-tauri/assets/brick-template`)

- Implementare anche il fatto che ogni elemento in `defineProps` presente in un brick deve essere visto come 
  delle impostazioni per il relativo brick e quando vengono modificate nei settings devono cambiare in tempo reale

- Il caricamento di dati dovrebbe avvenire sempre (o il piu' possibile) dal backend Rust
  questo perche' piu' sicuro e perche' i dati possono essere mantenuti nello stato generale dell'app
  e possono essere passati al frontend con dei metodi `invoke` in modo facile

- Cercare di capire e risolvere l'ottenimento delle applicazioni nella taskbar nativa di windows
  e rendere disponibile il comando con invoke

- Creare la logica Rust per l'invio di eventi globali (mouse, click, ecc.) che magari possono essere utilizzati dall'utente

- Al momento la creazione delle finestre la lasciamo a tauri
  piu avanti sarebbe meglio utilizzare `overlay::window::Overlay` per l'overlay
  e la stessa cosa per i settings `settings::window::Settings`
  in modo da avere piu controllo sulle singole finestre e separare la logica


Ho modificato questo in package.json, sarebbe da rimettere per controlli piu' stringenti su typescript
```json
"build": "vue-tsc --noEmit && vite build"
```