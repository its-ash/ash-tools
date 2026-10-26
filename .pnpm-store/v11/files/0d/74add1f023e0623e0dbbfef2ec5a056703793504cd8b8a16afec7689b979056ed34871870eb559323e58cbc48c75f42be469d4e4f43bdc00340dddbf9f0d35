import { KnownEditor } from "devframe/utils/launch-editor";
import { DevframeServiceDefinition } from "devframe/types";
//#region src/index.d.ts
export declare const OPEN_SERVICE_PACKAGE = "@devframes/service-open";
export declare const OPEN_SERVICE_SCOPE = "devframes:service:open";
export interface OpenServiceOptions {
  /**
   * Preferred editor command, one of the `KNOWN_EDITORS` `launch-editor`
   * recognizes. Auto-detected (via `LAUNCH_EDITOR` and common defaults)
   * when omitted. On merge, the later installer's choice wins.
   */
  editor?: KnownEditor;
  /**
   * Additional directories files may be opened from, on top of the
   * context's `workspaceRoot`, e.g. a plugin's managed storage dir that
   * lives outside the workspace. Merged as a union across installers.
   */
  roots?: string[];
}
export interface OpenInEditorInput {
  /**
   * File to open, absolute or relative to the service's `workspaceRoot`
   * (so a client with only a workspace-relative path, e.g. a message's file
   * position, can call this directly without a server-side bridge).
   */
  path: string;
  line?: number;
  column?: number;
  /** Per-call editor override (one of `KNOWN_EDITORS`). */
  editor?: KnownEditor;
}
export interface OpenServiceApi {
  /** Open a file (optionally at a line/column) in the user's editor. */
  openInEditor: (input: OpenInEditorInput) => Promise<void>;
  /** Reveal a path in the OS file explorer. */
  openInFinder: (input: {
    path: string;
  }) => Promise<void>;
}
declare module 'devframe' {
  interface DevframeRpcServerFunctions {
    'devframes:service:open:open-in-editor': (input: OpenInEditorInput) => Promise<void>;
    'devframes:service:open:open-in-finder': (input: {
      path: string;
    }) => Promise<void>;
  }
  interface DevframeServicesRegistry {
    '@devframes/service-open': OpenServiceApi;
  }
  interface DevframeServicesScopeRegistry {
    '@devframes/service-open': 'devframes:service:open';
  }
}
/**
 * The open wire service: `open-in-editor` / `open-in-finder` RPC shared by
 * every plugin on the host. Paths may be
 * absolute or relative to the `workspaceRoot`; the service refuses paths
 * outside the workspace root and the configured extra
 * {@link OpenServiceOptions.roots} (`DS_OPEN_0002`), and gates editor
 * commands to the `KNOWN_EDITORS` picklist so the RPC surface can't spawn an
 * arbitrary command.
 */
export declare function createOpenService(options?: OpenServiceOptions): DevframeServiceDefinition<OpenServiceApi, OpenServiceOptions>;
//#endregion
export { createOpenService as default };