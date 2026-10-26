//#region src/node/constants.ts
/** Stable devframe id for the terminals plugin. */
const PLUGIN_ID = "devframes_plugin_terminals";
/**
* Streaming channel carrying terminal output. Each session is a stream
* keyed by the session id, so clients subscribe by id the moment they
* see a session in the shared-state list.
*/
const TERMINAL_STREAM_CHANNEL = "devframes:plugin:terminals:output";
/**
* Streaming channel the hub's own terminals subsystem (`ctx.terminals`) uses
* for aggregated sessions contributed by *other* devframes (e.g. code-server).
* Mirrors `@devframes/hub`'s internal channel name; the plugin surfaces those
* sessions read-only and reads their output from here. Kept as a literal so the
* plugin needs no build dependency on the hub.
*/
const HUB_TERMINAL_STREAM_CHANNEL = "devframe:terminals";
/** Shared-state key holding the serializable session list. */
const SESSIONS_STATE_KEY = "devframes:plugin:terminals:sessions";
/** Shared-state key holding the spawnable command presets. */
const PRESETS_STATE_KEY = "devframes:plugin:terminals:presets";
/**
* Shared-state key the hub (`@devframes/hub`) mirrors the most recent dock
* activation into. When a mounted devframe asks the hub to switch to this
* dock (e.g. Vite DevTools navigating to the build it just spawned), the
* request lands here as `{ activation: { dockId, params } }`. The UI reads
* `params.sessionId` off it (when `dockId` is this plugin) to focus a specific
* session, converging even when it mounts *because* of the switch. Kept as a
* literal so the plugin needs no build dependency on the hub.
*/
const DOCKS_ACTIVE_STATE_KEY = "devframe:docks:active";
/** Default dev-server port for the standalone CLI. */
const DEFAULT_PORT = 9011;
/** Default number of output chunks retained for replay on reconnect. */
const DEFAULT_SCROLLBACK = 5e3;
/** Default terminal geometry before the client reports its real size. */
const DEFAULT_COLS = 80;
const DEFAULT_ROWS = 24;
//#endregion
export { DEFAULT_COLS, DEFAULT_PORT, DEFAULT_ROWS, DEFAULT_SCROLLBACK, DOCKS_ACTIVE_STATE_KEY, HUB_TERMINAL_STREAM_CHANNEL, PLUGIN_ID, PRESETS_STATE_KEY, SESSIONS_STATE_KEY, TERMINAL_STREAM_CHANNEL };
