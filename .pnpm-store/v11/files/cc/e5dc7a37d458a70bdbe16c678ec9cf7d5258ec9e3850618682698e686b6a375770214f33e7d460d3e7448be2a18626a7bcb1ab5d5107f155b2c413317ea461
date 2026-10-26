import { TerminalsOptions } from "./types.mjs";
import { DevframeViteBridgeOptions, DevframeVitePlugin } from "@devframes/vite/single";
//#region src/node/vite.d.ts
export interface TerminalsViteOptions extends TerminalsOptions {
  /** Forwarded to the underlying `devframeViteBridge`/`devframeVitePlugin` (mount base, etc.). */
  vite?: DevframeViteBridgeOptions;
}
/**
 * Mount the terminals panel into an existing Vite dev server. Returns two
 * plugins: a bridge that starts the devframe RPC + WebSocket server (so the
 * panel can stream terminal output), and a static mount that serves the
 * bundled SPA at the mount base. The bridge is listed first so its
 * `__connection.json` route is matched ahead of the SPA fallback.
 */
export declare function terminalsVite(options?: TerminalsViteOptions): DevframeVitePlugin[];
//#endregion