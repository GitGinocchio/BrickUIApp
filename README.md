

- Al momento la creazione delle finestre la lasciamo a tauri
  piu avanti sarebbe meglio utilizzare `overlay::window::Overlay` per l'overlay
  e la stessa cosa per i settings `settings::window::Settings`
  in modo da avere piu controllo sulle singole finestre e separare la logica


Ho modificato questo in package.json, sarebbe da rimettere per controlli piu' stringenti su typescript
```json
"build": "vue-tsc --noEmit && vite build"
```