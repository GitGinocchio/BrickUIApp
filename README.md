

- Bisogna aggiungere nel backend rust in `src-tauri/src/state.rs` la creazione del file `settings.yml`
  dal template `settings-template.yml` in `src-tauri/assets/settings-template.yml`

- Bisogna aggiungere la funzionalita' che quando viene modificato un brick vengono ricaricati tutti i brick nuovamente
  (senza il bisogno di riavviare l'app)
  magari si puo' mettere come modalita' "dev" per chi crea brick

- Bisogna aggiungere la possibilita' di creare un nuovo brick in app (quindi copiare il template `/src-tauri/assets/brick-template` nella cartella bricks)
  e dare la possibilita di aprire la cartella per sviluppare il brick (successivamente anche per l'editor visuale)

- Implementare i global bricks (cartella `/globals` nella directory in AppData)
  cioe' dei brick riutilizzabili all'interno di altri brick
  (quindi rendere impossibile l'utilizzo di brick nella cartella bricks all'interno di altri bricks)

- Il caricamento di dati dovrebbe avvenire sempre (o il piu' possibile) dal backend Rust
  questo perche' piu' sicuro e perche' i dati possono essere mantenuti nello stato generale dell'app
  e possono essere ottenuti con dei metodi invoke in modo facile

- Bisogna implementare la possibilita' di abilitare e disabilitare i brick a runtime (non dovrebbe essere difficile)

- Implementare anche il fatto che ogni elemento in `defineProps` presente in un brick deve essere visto come 
  delle impostazioni per il relativo brick e quando vengono modificate nei settings devono cambiare in tempo reale

- Bisogna creare la logica per il caricamento dei dati per ogni brick (da mostrare anche nella pagina apposita)
  quindi dal file `brick.yml` (guardare anche il template in `src-tauri/assets/brick-template`)

- Al momento la creazione delle finestre la lasciamo a tauri
  piu avanti sarebbe meglio utilizzare `overlay::window::Overlay` per l'overlay
  e la stessa cosa per i settings `settings::window::Settings`
  in modo da avere piu controllo sulle singole finestre e separare la logica


Ho modificato questo in package.json, sarebbe da rimettere per controlli piu' stringenti su typescript
```json
"build": "vue-tsc --noEmit && vite build"
```