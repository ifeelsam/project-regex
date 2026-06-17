import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    // SPA mode so the Tauri webview can serve the built assets statically.
    adapter: adapter({
      fallback: 'index.html'
    }),
    alias: {
      $lib: './src/lib'
    }
  }
};

export default config;
