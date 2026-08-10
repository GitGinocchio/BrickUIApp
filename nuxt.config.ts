// https://nuxt.com/docs/api/configuration/nuxt-config

import { resolve } from "path";

export default defineNuxtConfig({
  modules: ['@nuxtjs/i18n', '@nuxt/ui'],
  css: ['~/assets/css/main.css'],

  compatibilityDate: '2025-05-15',
  // (optional) Enable the Nuxt devtools
  devtools: { enabled: true },
  // Enable SSG
  ssr: false,

  experimental: {
    payloadExtraction: 'client'
  },

  // Assicurati che il router usi il sistema di file relativo (opzionale ma consigliato per Tauri)
  router: {
    options: {
      hashMode: true // Molto utile in Tauri per evitare problemi con i percorsi al refresh
    }
  },

  // Altre ottimizzazioni per la build
  nitro: {
    preset: 'service-worker', // Evita la generazione di un server Node pesante
    /*prerender: {
      routes: ['/', '/overlay'] // Elenca qui tutte le tue pagine
    },*/
    static: true,
    output: {
      publicDir: './src/dist' // Qui finiranno i tuoi file statici per Tauri
    }
  },

  // Enables the development server to be discoverable by other devices when running on iOS physical devices
  devServer: {
    host: '0.0.0.0',
    port: 1420
  },

  srcDir: './src',

  alias: {
    // Definizione di alias personalizzati
    '#interfaces': resolve(__dirname, './src/interfaces'),
    '#components': resolve(__dirname, './src/components'),
    '#composables': resolve(__dirname, './src/composables'),
    '#utils': resolve(__dirname, './src/utils'),
    '#assets': resolve(__dirname, './src/assets')
  },

  i18n: {
    strategy: 'no_prefix',
    defaultLocale: 'en',
    restructureDir: './src',
    langDir: 'locales',
    locales: [
      { code: 'en', file: 'en.json' },
      { code: 'it', file: 'it.json' }
    ],
  },

  typescript: {
    strict: false,
    typeCheck: false,
    shim: false,
    tsConfig: {
      compilerOptions: {
        target: "ES2024",
        lib: ["ES2024", "ES2024.Collection", "DOM", "DOM.Iterable"],
        useDefineForClassFields: true,
        noUnusedLocals: true,
        noUnusedParameters: true,
        noFallthroughCasesInSwitch: true,
        skipLibCheck: true,
      }
    }
  },

  vite: {
    build: {
      target: 'esnext'
    },

    // Better support for Tauri CLI output
    clearScreen: false,
    // Enable environment variables
    // Additional environment variables can be found at
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      // Tauri requires a consistent port
      strictPort: true,
    },
  },

  // Avoids error [unhandledRejection] EMFILE: too many open files, watch
  ignore: ['**/src-tauri/**'],
});
