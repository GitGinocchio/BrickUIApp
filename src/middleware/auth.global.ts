// src/middleware/auth.global.ts
import { invoke } from '@tauri-apps/api/core'

export default defineNuxtRouteMiddleware(async (to, _from) => {
  // Verifichiamo lo stato del login su Rust
  const isLoggedIn: boolean = await invoke("auth_is_logged_in");

  // Se l'utente è loggato e prova ad andare su /get-started, mandalo su /user
  if (to.path === '/get-started' && isLoggedIn) {
    return navigateTo('/user')
  }

  // Controlliamo se la pagina richiede auth (meta definita nella pagina)
  if (to.meta.requiresAuth && !isLoggedIn) {
    return navigateTo('/get-started')
  }
})