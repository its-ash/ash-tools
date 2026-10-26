import * as _nuxt_schema from '@nuxt/schema';
import { ViteDevToolsNodeContext } from '@vitejs/devtools-kit';
import { BirpcGroup } from 'birpc';
import { ChildProcess } from 'node:child_process';
import { Result } from 'tinyexec';
import { NuxtDevtoolsServerContext, ModuleCustomTab, NuxtDevtoolsInfo, SubprocessOptions, TerminalState } from './types.mjs';
import * as nostics from 'nostics';
import { defineDiagnostics } from 'nostics';
export { createConsoleReporter, defineDiagnostics } from 'nostics';
import { Nuxt } from 'nuxt/schema';
import 'vue';
import 'unimport';
import 'vue-router';
import 'unstorage';
import 'vite';
import 'nitro/types';
import 'nitropack';

/**
 * Canonical docs URL for a Nuxt DevTools diagnostic code.
 *
 * Each code links to a per-code anchor in the v4 migration guide, e.g.
 * `NDT_DEP_0001` → `.../module/migration-v4#ndt_dep_0001`.
 */
declare function diagnosticsDocsBase(code: string | number): string;
/**
 * Parameters shared by every deprecation code: the API being used and its
 * recommended replacement. Interpolated into the `why`/`fix` messages.
 */
interface DeprecationParams {
    /** The deprecated API / option being used. */
    api: string;
    /** The recommended replacement to migrate to. */
    replacement: string;
}
/**
 * The Nuxt DevTools diagnostics catalog.
 *
 * Codes are grouped by prefix:
 * - `NDT_DEP_xxxx` — soft/hard deprecations (this is the only range used today).
 *
 * Severity is **not** encoded here; it is chosen per emission via the reporter
 * `method` (`warn` by default, `error` for hard breaks).
 */
declare const diagnosticCodes: {
    /** `startSubprocess().getProcess()` → `getResult()`. */
    NDT_DEP_0001: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** `extendServerRpc` → `onDevtoolsReady((ctx) => ctx.rpc.register(...))`. */
    NDT_DEP_0003: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** `startSubprocess` → `onDevtoolsReady((ctx) => ctx.terminals.startChildProcess(...))`. */
    NDT_DEP_0004: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** `addCustomTab` → `onDevtoolsReady((ctx) => ctx.docks.register(...))`. */
    NDT_DEP_0005: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** `refreshCustomTabs` → `onDevtoolsReady((ctx) => ctx.docks.register(...))`. */
    NDT_DEP_0006: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** Direct `nuxt.devtools.rpc` access (`broadcast` / `functions`). */
    NDT_DEP_0007: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** Removed `vscode` module option → `codeServer`. */
    NDT_DEP_0008: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** `getServerData()` RPC → the Data Inspector panel's `Nuxt Application` source. */
    NDT_DEP_0009: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
};
type NuxtDiagnosticCode = keyof typeof diagnosticCodes;
/**
 * Standalone catalog that prints to the terminal via nostics'
 * {@link createConsoleReporter} (default method `warn`). Works before the Vite
 * DevTools kit connects, so it is the fallback sink for pre-connect emissions.
 */
declare const consoleDiagnostics: nostics.Diagnostics<{
    /** `startSubprocess().getProcess()` → `getResult()`. */
    NDT_DEP_0001: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** `extendServerRpc` → `onDevtoolsReady((ctx) => ctx.rpc.register(...))`. */
    NDT_DEP_0003: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** `startSubprocess` → `onDevtoolsReady((ctx) => ctx.terminals.startChildProcess(...))`. */
    NDT_DEP_0004: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** `addCustomTab` → `onDevtoolsReady((ctx) => ctx.docks.register(...))`. */
    NDT_DEP_0005: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** `refreshCustomTabs` → `onDevtoolsReady((ctx) => ctx.docks.register(...))`. */
    NDT_DEP_0006: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** Direct `nuxt.devtools.rpc` access (`broadcast` / `functions`). */
    NDT_DEP_0007: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** Removed `vscode` module option → `codeServer`. */
    NDT_DEP_0008: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
    /** `getServerData()` RPC → the Data Inspector panel's `Nuxt Application` source. */
    NDT_DEP_0009: {
        why: (p: DeprecationParams) => string;
        fix: (p: DeprecationParams) => string;
    };
}, readonly [nostics.DiagnosticReporter<{
    method?: nostics.ConsoleMethod;
}, any>], never>;
/**
 * Register the Nuxt deprecation codes into the connected Vite DevTools kit's
 * diagnostics host so they are known to DevTools and post-connect emissions
 * surface in the DevTools diagnostics UI. Safe to call when no host is present.
 *
 * Called from `connectDevToolsKit`.
 */
declare function registerHostDiagnostics(ctx: NuxtDevtoolsServerContext): void;
/**
 * Options for {@link deprecate}.
 */
interface DeprecateOptions {
    /**
     * Dedupe key appended to the code. Defaults to the code itself, i.e. the
     * deprecation warns once per process. Pass a finer key (e.g. a subprocess id)
     * to warn once per distinct call site instead.
     */
    key?: string;
    /**
     * Reporter method / severity. `warn` (default) for soft deprecations, `error`
     * for hard breaks. The returned {@link Diagnostic} can be thrown to abort.
     */
    method?: 'warn' | 'error';
}
/**
 * Emit a Nuxt DevTools deprecation diagnostic.
 *
 * Routing: when the Vite DevTools kit is connected the emission goes through the
 * DevTools host catalog (terminal **and** the DevTools diagnostics UI); before
 * connect it falls back to the terminal-only console catalog. A single emission
 * per call — no double printing.
 *
 * Deduped per `${code}:${key ?? code}` on the resolved server context, so a hot
 * path warns only once.
 *
 * @returns the built `Diagnostic` (which extends `Error`, so it can be thrown
 * for hard breaks), or `undefined` if the emission was deduped.
 */
declare function deprecate(nuxt: Nuxt, code: NuxtDiagnosticCode, params: DeprecationParams, options?: DeprecateOptions): Error | undefined;
/**
 * Build a standalone nostics catalog that prints to the terminal.
 *
 * Used by the connect-safe `nuxt.devtools.diagnostics` accessor so module
 * authors can define + emit diagnostics before the Vite DevTools kit connects.
 * A `createConsoleReporter()` is added automatically unless the caller supplies
 * its own `reporters`.
 */
declare function defineStandaloneDiagnostics(options: Parameters<typeof defineDiagnostics>[0]): ReturnType<typeof defineDiagnostics>;
/**
 * Convenience wrapper for call sites that don't already hold the Nuxt instance.
 */
declare function deprecateWithNuxt(code: NuxtDiagnosticCode, params: DeprecationParams, options?: DeprecateOptions): Error | undefined;

/**
 * The public `Nuxt` dock group id registered on the Vite DevTools framework
 * category. Module authors join the group natively by pointing their own
 * dock entries at it — no special Nuxt API required:
 *
 * @example
 * ```ts
 * import { NUXT_DEVTOOLS_GROUP_ID, onDevtoolsReady } from '@nuxt/devtools-kit'
 *
 * onDevtoolsReady((ctx) => {
 *   ctx.docks.register({
 *     id: 'my-module',
 *     type: 'iframe',
 *     title: 'My Module',
 *     icon: 'i-ph-puzzle-piece',
 *     url: '/my-module/',
 *     groupId: NUXT_DEVTOOLS_GROUP_ID,
 *   })
 * })
 * ```
 */
declare const NUXT_DEVTOOLS_GROUP_ID = "nuxt";
/**
 * Hooks to extend a custom tab in devtools.
 *
 * Provide a function to pass a factory that can be updated dynamically.
 *
 * @deprecated Register a dock entry from the `devtools:ready` hook instead:
 * `onDevtoolsReady((ctx) => ctx.docks.register(...))`. Still works as a shim, but
 * emits the `NDT_DEP_0005` deprecation diagnostic. Note the docks host does not
 * yet cover `vnode` views or tab categories.
 */
declare function addCustomTab(tab: ModuleCustomTab | (() => ModuleCustomTab | Promise<ModuleCustomTab>), nuxt?: _nuxt_schema.Nuxt): void;
/**
 * Retrigger update for custom tabs, `devtools:customTabs` will be called again.
 *
 * @deprecated Update dock entries directly via the handle returned by
 * `ctx.docks.register(...)` inside the `devtools:ready` hook. Still works as a
 * shim, but emits the `NDT_DEP_0006` deprecation diagnostic.
 */
declare function refreshCustomTabs(nuxt?: _nuxt_schema.Nuxt): void | Promise<any>;
interface StartSubprocessReturn {
    /** @deprecated Use `getResult()` instead */
    getProcess: () => ChildProcess | undefined;
    getResult: () => Result;
    terminate: () => void;
    restart: () => void;
    clear: () => void;
}
/**
 * Create a subprocess that handled by the DevTools.
 *
 * @deprecated Use the Vite DevTools terminals host from the `devtools:ready`
 * hook instead: `onDevtoolsReady((ctx) => ctx.terminals.startChildProcess(...))`.
 * Still works as a shim, but emits the `NDT_DEP_0004` deprecation diagnostic.
 */
declare function startSubprocess(execaOptions: SubprocessOptions, tabOptions: TerminalState, nuxt?: _nuxt_schema.Nuxt): StartSubprocessReturn;
/**
 * Extend server RPC with namespaced functions.
 *
 * Returns an object with a `broadcast` proxy for calling client functions.
 *
 * @deprecated Register RPC functions from the `devtools:ready` hook instead:
 * `onDevtoolsReady((ctx) => ctx.rpc.register(defineRpcFunction(...)))`. Still
 * works as a shim, but emits the `NDT_DEP_0003` deprecation diagnostic.
 */
declare function extendServerRpc<ClientFunctions extends object = Record<string, unknown>, ServerFunctions extends object = Record<string, unknown>>(namespace: string, functions: ServerFunctions, nuxt?: _nuxt_schema.Nuxt): BirpcGroup<ClientFunctions, ServerFunctions>;
declare function onDevToolsInitialized(fn: (info: NuxtDevtoolsInfo) => void, nuxt?: _nuxt_schema.Nuxt): void;
/**
 * Run a callback once the Vite DevTools kit has connected, receiving the
 * connected `ViteDevToolsNodeContext`.
 *
 * This is the recommended entry point for DevTools integration: the kit is
 * guaranteed available, so you can use `ctx.docks` / `ctx.terminals` /
 * `ctx.messages` / `ctx.commands` / `ctx.rpc` / `ctx.diagnostics` directly
 * without the connect-safe accessors on `nuxt.devtools`.
 *
 * @example
 * ```ts
 * onDevtoolsReady((ctx) => {
 *   ctx.docks.register({ id: 'my-module', title: 'My Module', type: 'iframe', url: '/…' })
 * })
 * ```
 */
declare function onDevtoolsReady(fn: (ctx: ViteDevToolsNodeContext) => void | Promise<void>, nuxt?: _nuxt_schema.Nuxt): void;

export { NUXT_DEVTOOLS_GROUP_ID, addCustomTab, consoleDiagnostics, defineStandaloneDiagnostics, deprecate, deprecateWithNuxt, diagnosticCodes, diagnosticsDocsBase, extendServerRpc, onDevToolsInitialized, onDevtoolsReady, refreshCustomTabs, registerHostDiagnostics, startSubprocess };
export type { DeprecateOptions, DeprecationParams, NuxtDiagnosticCode, StartSubprocessReturn };
