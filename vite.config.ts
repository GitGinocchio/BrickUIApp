import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { resolve } from 'path';

// process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [vue({
    include: [/\.vue$/, /\.brick$/]
  })],

  resolve: {
    alias: {
      vue: 'vue/dist/vue.esm-bundler.js',
      '@assets/js': resolve(__dirname, 'src/assets/js'),
      '@public/js': resolve(__dirname, 'src/public/js')
    }
  },
  build: {
    assetsInlineLimit: 0,
    outDir: "dist",
    target: "esnext",
    rollupOptions: {
      input: {
        settings: resolve(__dirname, 'src/index.html'),
        "overlay/index": resolve(__dirname, 'src/overlay/index.html')
      }
    }
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  base: './',
  root: './src',
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    cors: true,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
