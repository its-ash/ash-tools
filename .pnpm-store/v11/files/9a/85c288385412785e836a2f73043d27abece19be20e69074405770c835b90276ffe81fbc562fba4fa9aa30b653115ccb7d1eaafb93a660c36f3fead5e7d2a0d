import { a as DEFAULT_ALLOWED_UPLOAD_EXTENSIONS, i as CodeSnippet, n as AssetInfo, r as AssetType, t as AssetImageMeta } from "../types-Bspm5Ylo.mjs";
import { DevframeDefinition } from "devframe";
//#region src/node/index.d.ts
export interface AssetsDevframeOptions {
  id?: string;
  name?: string;
  icon?: string;
  /**
   * Directory this devframe manages. Defaults to `<cwd>/public`, the
   * conventional static-asset folder for Nuxt, Vite, and most static-site
   * frameworks. Pass an absolute path, or one relative to the devframe's
   * own `cwd`.
   */
  dir?: string;
  /**
   * URL base the **host** serves the managed directory at, used to build
   * each asset's `publicPath`. Defaults to `/`; Vite, Nuxt, and most
   * frameworks serve their `public/` folder at the site root. Set this to
   * match a non-root deployment base (e.g. Nuxt's `app.baseURL`).
   */
  baseURL?: string;
  /**
   * Serve the managed directory's bytes from this devframe itself.
   * Defaults to `false`: when mounted into a host (Vite/Nuxt/…) that host
   * already serves `public/` at {@link baseURL}, so the plugin references
   * those URLs instead of standing up its own route. The standalone CLI
   * sets this to `true` (it is its own host), serving the directory under
   * a dedicated base so it doesn't collide with the SPA at `/`.
   */
  serveStatic?: boolean;
  basePath?: string;
  distDir?: string;
  /** Preferred standalone CLI port (default 9015). */
  port?: number;
  /**
   * Enable upload, rename, delete, and folder creation from the UI.
   * Enabled by default; set `false` (or pass `--read-only` on the
   * standalone CLI) for a browse-only deployment.
   */
  write?: boolean;
  /**
   * Extensions `upload` accepts, or `'*'` to accept any file. Defaults to
   * the same allow-list Nuxt DevTools ships.
   */
  uploadExtensions?: readonly string[] | '*';
  /**
   * Watch the managed directory in dev mode and push live listing updates.
   * Enabled by default. Set `false` to skip the file watcher, useful when a
   * host mounts the devframe in a short-lived or repeatedly-recreated
   * context (tests, some SSR dev servers) where a background watcher isn't
   * wanted.
   */
  watch?: boolean;
  /**
   * Require the trust handshake on the standalone server. Enabled by
   * default, because this devframe can read, write, and delete real files.
   */
  auth?: boolean;
  /**
   * Register the `build` CLI subcommand. Disabled by default: a static
   * export can only ever list file metadata from a baked snapshot; there
   * is no live host serving the files, and every write action is inherently
   * excluded from a static dump, so the command would produce a broken,
   * write-less shell of the tool. Opt back in if that degraded export is
   * still useful to you.
   */
  build?: boolean;
}
/**
 * Create the assets manager devframe, a framework-neutral port of Nuxt
 * DevTools' Assets tab. Mount it into any host via devframe's adapters, or
 * run it standalone with the bundled CLI (`devframe-assets`).
 *
 * @experimental This plugin is experimental and may change without a major
 * version bump until it stabilizes.
 */
export declare function createAssetsDevframe(options?: AssetsDevframeOptions): DevframeDefinition;
//#endregion
export { type AssetImageMeta, type AssetInfo, type AssetType, type CodeSnippet, DEFAULT_ALLOWED_UPLOAD_EXTENSIONS, createAssetsDevframe as default };