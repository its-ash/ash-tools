//#region src/node/constants.ts
/** Stable devframe id for the code-server plugin. */
const PLUGIN_ID = "devframes_plugin_code-server";
/**
* Shared-state key holding the serializable, secret-free server status and
* detection result. Authentication material is never published here; it is
* returned only from the `start` / `status` RPCs to the already-authorized
* client (see {@link import('./types').CodeServerConnect}).
*/
const STATE_KEY = "devframes:plugin:code-server:state";
/** Default dev-server port for the plugin's own launcher SPA (standalone CLI). */
const DEFAULT_PORT = 9013;
/** Preferred port for the spawned editor process (falls back if taken). */
const DEFAULT_CODE_SERVER_PORT = 8080;
/** How long to wait for the editor to answer its readiness probe. */
const DEFAULT_START_TIMEOUT = 3e4;
/** code-server's session cookie base name (see `getCookieSessionName`). */
const SESSION_COOKIE_BASE = "code-server-session";
/**
* Title for the read-only terminal session surfaced when the plugin is mounted
* in a hub (`ctx.terminals`). Mirrors the plugin's own name.
*/
const TERMINAL_SESSION_TITLE = "Code Server";
/**
* Icon for that terminal session. Mirrors the plugin's declared icon so the
* hub's terminals panel shows the same glyph as the rest of the plugin.
*/
const TERMINAL_SESSION_ICON = "ph:code-duotone";
/**
* Compute code-server's session cookie name for an optional `--cookie-suffix`.
* Mirrors code-server's own `getCookieSessionName` so the client sets the
* exact cookie the server reads.
*/
function getCookieSessionName(suffix) {
	return suffix ? `${SESSION_COOKIE_BASE}-${suffix.replace(/[^a-z0-9-]/gi, "-")}` : SESSION_COOKIE_BASE;
}
//#endregion
export { DEFAULT_CODE_SERVER_PORT, DEFAULT_PORT, DEFAULT_START_TIMEOUT, PLUGIN_ID, SESSION_COOKIE_BASE, STATE_KEY, TERMINAL_SESSION_ICON, TERMINAL_SESSION_TITLE, getCookieSessionName };
