import { fileURLToPath } from "node:url";
//#region src/dirs.ts
const dirDist = fileURLToPath(new URL("../dist", import.meta.url));
const dirAssets = fileURLToPath(new URL("../assets", import.meta.url));
/**
* The base path for the static assets that are shipped with Vite DevTools. These
* assets are served from the DevTools hub at `/__devtools-assets/` and include the
* Vite+ logo, the Vite DevTools branding wordmark, and the per-scheme light/dark
* variants of the wordmark. The assets are vendored into the `@vitejs/devtools-kit`
* package and are copied into the DevTools hub's static assets directory at build
* time.
*/
const DEVTOOLS_ASSETS_BASE = "/__devtools-assets/";
//#endregion
export { DEVTOOLS_ASSETS_BASE, dirAssets, dirDist };
