// SPA mode: no SSR, no prerendering. Tauri serves the built static assets and
// all routing happens client-side in the native webview.
export const ssr = false;
export const prerender = false;
export const trailingSlash = 'always';
