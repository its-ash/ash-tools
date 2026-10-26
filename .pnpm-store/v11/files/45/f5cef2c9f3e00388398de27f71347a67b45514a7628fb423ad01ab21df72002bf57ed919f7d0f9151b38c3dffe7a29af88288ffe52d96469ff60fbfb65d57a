//#region src/node/constants.d.ts
/** Stable devframe id for the code-server plugin. */
declare const PLUGIN_ID = "devframes_plugin_code-server";
/**
 * Shared-state key holding the serializable, secret-free server status and
 * detection result. Authentication material is never published here; it is
 * returned only from the `start` / `status` RPCs to the already-authorized
 * client (see {@link import('./types').CodeServerConnect}).
 */
declare const STATE_KEY = "devframes:plugin:code-server:state";
/** Default dev-server port for the plugin's own launcher SPA (standalone CLI). */
declare const DEFAULT_PORT = 9013;
/** Preferred port for the spawned editor process (falls back if taken). */
declare const DEFAULT_CODE_SERVER_PORT = 8080;
/** How long to wait for the editor to answer its readiness probe. */
declare const DEFAULT_START_TIMEOUT = 30000;
/** code-server's session cookie base name (see `getCookieSessionName`). */
declare const SESSION_COOKIE_BASE = "code-server-session";
/**
 * Title for the read-only terminal session surfaced when the plugin is mounted
 * in a hub (`ctx.terminals`). Mirrors the plugin's own name.
 */
declare const TERMINAL_SESSION_TITLE = "Code Server";
/**
 * Icon for that terminal session. Mirrors the plugin's declared icon so the
 * hub's terminals panel shows the same glyph as the rest of the plugin.
 */
declare const TERMINAL_SESSION_ICON = "ph:code-duotone";
/**
 * Compute code-server's session cookie name for an optional `--cookie-suffix`.
 * Mirrors code-server's own `getCookieSessionName` so the client sets the
 * exact cookie the server reads.
 */
declare function getCookieSessionName(suffix?: string): string;
//#endregion
export { SESSION_COOKIE_BASE as a, TERMINAL_SESSION_TITLE as c, PLUGIN_ID as i, getCookieSessionName as l, DEFAULT_PORT as n, STATE_KEY as o, DEFAULT_START_TIMEOUT as r, TERMINAL_SESSION_ICON as s, DEFAULT_CODE_SERVER_PORT as t };