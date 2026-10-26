import { CodeServerBackend, CodeServerConnect, CodeServerDetection, CodeServerLogin, CodeServerMode, CodeServerOptions, CodeServerServerInfo, CodeServerSharedState, CodeServerStartRequest, CodeServerStartResult, CodeServerStatus, CodeServerStatusResult, CodeServerTunnelOptions } from "./types.mjs";
import { i as PLUGIN_ID, l as getCookieSessionName, n as DEFAULT_PORT, o as STATE_KEY, t as DEFAULT_CODE_SERVER_PORT } from "../constants-DTS2klFC.mjs";
import { DevframeDefinition } from "devframe";
//#region src/node/index.d.ts
/**
 * Build a {@link DevframeDefinition} for the code-server panel. The same
 * definition runs standalone (`createCac`), mounts into a Vite host
 * (`/vite`), or docks inside a hub, since its `setup` only relies on the core
 * devframe RPC + shared-state surface.
 *
 * @experimental This plugin is experimental and may change without a major
 * version bump until it stabilizes.
 *
 * @example
 * ```ts
 * import { createCodeServerDevframe } from '@devframes/plugin-code-server'
 *
 * export default createCodeServerDevframe({ serverPort: 8080 })
 * ```
 */
export declare function createCodeServerDevframe(options?: CodeServerOptions): DevframeDefinition;
//#endregion
export { type CodeServerBackend, type CodeServerConnect, type CodeServerDetection, type CodeServerLogin, type CodeServerMode, type CodeServerOptions, type CodeServerServerInfo, type CodeServerSharedState, type CodeServerStartRequest, type CodeServerStartResult, type CodeServerStatus, type CodeServerStatusResult, type CodeServerTunnelOptions, DEFAULT_CODE_SERVER_PORT, DEFAULT_PORT, PLUGIN_ID, STATE_KEY, createCodeServerDevframe as default, getCookieSessionName };