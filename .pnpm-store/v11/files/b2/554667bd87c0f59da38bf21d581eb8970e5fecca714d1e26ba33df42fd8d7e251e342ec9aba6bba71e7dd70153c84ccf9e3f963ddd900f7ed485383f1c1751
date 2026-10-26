import { n as AssetInfo, r as AssetType } from "../types-Bspm5Ylo.mjs";
import { DevframeNodeContext, RpcStreamingChannel } from "devframe";
//#region src/node/context.d.ts
interface AssetsConfig {
  /** Directory this devframe manages. */
  dir: string;
  /** Whether upload / rename / delete / mkdir are registered. */
  write: boolean;
  /** Extensions `upload` accepts, or `'*'` to accept any. */
  uploadExtensions: readonly string[] | '*';
  /**
   * URL base the managed directory is served at. Normally the host
   * (Vite / Nuxt / …) serves `public/` at `/`, so asset `publicPath`s are
   * `joinURL(baseURL, path)` and the plugin serves nothing itself.
   */
  baseURL: string;
  /** The upload streaming channel, created once in `setupAssets` when `write` is enabled. */
  uploadChannel?: RpcStreamingChannel<Uint8Array>;
}
interface AssetsContext extends AssetsConfig {
  /**
   * Resolve a root-relative path to an absolute one, rejecting lexical
   * escapes. Symlink-aware containment for reads and mutations lives in
   * `node/paths` (`resolveAssetReadPath` / `assertAssetMutationPath`), which
   * the RPC handlers call directly with {@link AssetsContext.dir}.
   */
  resolvePath: (relativePath: string) => string;
}
/**
 * Record the managed directory and write policy for a context. Called from
 * the devframe's `setup` before any RPC handler runs.
 */
export declare function configureAssets(ctx: DevframeNodeContext, config: AssetsConfig): void;
/**
 * Per-`DevframeNodeContext` assets state. Each RPC function pulls its
 * managed directory and path resolver from here instead of re-reading
 * `configureAssets`'s config on every call.
 */
export declare function getAssetsContext(ctx: DevframeNodeContext): AssetsContext;
//#endregion
//#region src/node/paths.d.ts
/**
 * Resolve a client-supplied, root-relative path against the managed
 * directory, rejecting anything that would escape it lexically (`..`
 * traversal, a rogue absolute path). The first guard every RPC handler runs;
 * symlink-aware containment is layered on by {@link resolveAssetReadPath}
 * (reads) and {@link assertAssetMutationPath} (mutations).
 */
export declare function resolveAssetPath(root: string, relativePath: string): string;
//#endregion
//#region src/node/scanner.d.ts
/** Classifies a path by extension, driving preview and icon choice. */
export declare function guessAssetType(path: string): AssetType;
/** Recursively lists every file under `dir`, sorted alphabetically by path. */
export declare function scanAssets(dir: string, baseURL: string, includeFsPath?: boolean): Promise<AssetInfo[]>;
//#endregion
//#region src/node/setup.d.ts
export interface SetupAssetsOptions {
  /** Directory this devframe manages. */
  dir: string;
  /** Whether upload / rename / delete / mkdir are registered. */
  write: boolean;
  /** Extensions `upload` accepts, or `'*'` to accept any. */
  uploadExtensions: readonly string[] | '*';
  /**
   * URL base the managed directory is served at, the base every asset's
   * `publicPath` is resolved against.
   */
  baseURL: string;
  /**
   * Whether this devframe should serve the managed directory's bytes
   * itself. Left `false` when mounted into a host (Vite / Nuxt / …) that
   * already serves `public/` at {@link baseURL}; set `true` only for the
   * standalone CLI, which is its own host.
   */
  serveStatic: boolean;
  /** Start the live file watcher (dev mode only). */
  watch: boolean;
}
/**
 * Register the assets RPC surface on a devframe node context: ensures the
 * managed directory exists, registers the read RPCs (always) and write
 * RPCs (when enabled), and starts a live file watcher in dev mode.
 *
 * Raw asset bytes (for `<img>`/`<video>`/download previews) are served by
 * the **host** the plugin is attached to: Vite, Nuxt, or any framework
 * already serves its `public/` dir at {@link SetupAssetsOptions.baseURL}.
 * The plugin only serves them itself when `serveStatic` is set (the
 * standalone CLI, which is its own host).
 *
 * Called from the definition's `setup(ctx)` and reusable by host adapters
 * that wire their own context.
 */
export declare function setupAssets(ctx: DevframeNodeContext, options: SetupAssetsOptions): Promise<void>;
/**
 * Stop the live file watcher started by {@link setupAssets}. Test harnesses
 * wrap their dev-server `close()` with this (see `test/_utils.ts`), so a
 * leaked chokidar watcher doesn't keep the process alive.
 */
export declare function disposeAssetsWatcher(ctx: DevframeNodeContext): Promise<void>;
//#endregion