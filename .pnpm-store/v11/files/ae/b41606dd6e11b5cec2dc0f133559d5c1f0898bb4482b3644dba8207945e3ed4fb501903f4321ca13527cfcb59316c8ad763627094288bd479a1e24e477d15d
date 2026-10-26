import { DEFAULT_PORT, MESSAGES_UPDATED_EVENT, PLUGIN_ID } from "./constants.mjs";
import { DevframeDefinition } from "devframe";
import { DevframeMessageEntry, DevframeMessageEntryFrom, DevframeMessageEntryInput, DevframeMessageLevel, DevframeMessagesListDelta } from "@devframes/hub/types";
//#region src/node/index.d.ts
/** The panel `clientScript` bundle (`dist/client`) stays in this node package. */
export interface MessagesDevframeOptions {
  /** Override the devframe id (and default CLI command / mount path). */
  id?: string;
  /** Override the display name shown in a host dock. */
  name?: string;
  /** Override the dock icon. */
  icon?: string;
  /**
   * Override the mount path. Left unset, the SPA mounts at `/` standalone
   * and `/__<id>/` when hosted (Vite/embedded).
   */
  basePath?: string;
  /** Preferred standalone CLI port. */
  port?: number;
  /**
   * Require the trust handshake on the standalone server. Enabled by
   * default; `--open` embeds the current OTP in the opened URL, so the
   * tab authenticates automatically without extra prompts. Hosted adapters
   * manage their own auth and ignore this.
   */
  auth?: boolean;
}
/**
 * Build a {@link DevframeDefinition} for the hub message feed panel,
 * a portable view over `ctx.messages`, ported from vitejs/devtools'
 * built-in Messages view. The same definition runs standalone
 * (`/cli`, `/build`) and mounts into a host (`/vite`, hub);
 * a hub host is what feeds it live entries.
 *
 * @experimental This plugin is experimental and may change without a major
 * version bump until it stabilizes.
 */
export declare function createMessagesDevframe(options?: MessagesDevframeOptions): DevframeDefinition;
//#endregion
export { DEFAULT_PORT, type DevframeMessageEntry, type DevframeMessageEntryFrom, type DevframeMessageEntryInput, type DevframeMessageLevel, type DevframeMessagesListDelta, MESSAGES_UPDATED_EVENT, PLUGIN_ID, createMessagesDevframe as default };