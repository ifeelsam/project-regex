import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

const host = process.env.TAURI_DEV_HOST;

const isTauri = !!process.env.TAURI_ENV_PLATFORM;

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],

  // Tauri expects a fixed port and fails if it is not available.
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421
        }
      : undefined,
    watch: {
      // Don't watch the Rust backend for the frontend dev server.
      ignored: ['**/src-tauri/**']
    }
  },
  // Produce a leaner build for the native webview.
  build: {
    target: isTauri ? ['es2021', 'chrome105', 'safari13'] : 'modules'
  }
});
