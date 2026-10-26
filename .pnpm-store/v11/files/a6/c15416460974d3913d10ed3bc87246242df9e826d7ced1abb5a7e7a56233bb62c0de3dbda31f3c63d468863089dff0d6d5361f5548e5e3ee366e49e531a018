import { DevToolsMessageLevel, DevToolsMessageFilePosition, ViteDevToolsNodeContext } from '@vitejs/devtools-kit';
import { VNode, MaybeRefOrGetter } from 'vue';
import { BirpcGroup } from 'birpc';
import { Component, NuxtOptions, NuxtPage, NuxtLayout, NuxtApp, Nuxt, NuxtDebugModuleMutationRecord } from 'nuxt/schema';
import { Import, UnimportMeta } from 'unimport';
import { RouteRecordNormalized } from 'vue-router';
import { StorageValue } from 'unstorage';
import { ResolvedConfig } from 'vite';
import { NuxtAnalyzeMeta } from '@nuxt/schema';
import { Nitro as Nitro$1, StorageMounts as StorageMounts$1 } from 'nitro/types';
import { Nitro, StorageMounts } from 'nitropack';
import { SpawnOptions } from 'node:child_process';

type TabCategory = 'pinned' | 'app' | 'analyze' | 'server' | 'modules' | 'documentation' | 'advanced';

interface ModuleCustomTab {
    /**
     * The name of the tab, must be unique
     */
    name: string;
    /**
     * Icon of the tab, support any Iconify icons, or a url to an image
     */
    icon?: string;
    /**
     * Title of the tab
     */
    title: string;
    /**
     * Main view of the tab
     */
    view: ModuleView;
    /**
     * Category of the tab
     * @default 'app'
     */
    category?: TabCategory;
    /**
     * Insert static vnode to the tab entry
     *
     * Advanced options. You don't usually need this.
     */
    extraTabVNode?: VNode;
    /**
     * @deprecated No longer enforced client-side. Access to sensitive RPC
     * methods is gated by Vite DevTools' own connection authorization instead.
     */
    requireAuth?: boolean;
}
interface ModuleLaunchView {
    /**
     * A view for module to lazy launch some actions
     */
    type: 'launch';
    title?: string;
    icon?: string;
    description: string;
    /**
     * Action buttons
     */
    actions: ModuleLaunchAction[];
}
interface ModuleIframeView {
    /**
     * Iframe view
     */
    type: 'iframe';
    /**
     * Url of the iframe
     */
    src: string;
    /**
     * Persist the iframe instance even if the tab is not active
     *
     * @default true
     */
    persistent?: boolean;
    /**
     * Additional permissions to allow in the iframe
     * These will be merged with the default permissions (clipboard-write, clipboard-read)
     *
     * @example ['camera', 'microphone', 'geolocation']
     */
    permissions?: string[];
}
interface ModuleVNodeView {
    /**
     * Vue's VNode view
     */
    type: 'vnode';
    /**
     * Send vnode to the client, they must be static and serializable
     *
     * Call `nuxt.hook('devtools:customTabs:refresh')` to trigger manual refresh
     */
    vnode: VNode;
}
interface ModuleLaunchAction {
    /**
     * Label of the action button
     */
    label: string;
    /**
     * Additional HTML attributes to the action button
     */
    attrs?: Record<string, string>;
    /**
     * Indicate if the action is pending, will show a loading indicator and disable the button
     */
    pending?: boolean;
    /**
     * Function to handle the action, this is executed on the server side.
     * Will automatically refresh the tabs after the action is resolved.
     */
    handle?: () => void | Promise<void>;
    /**
     * Treat the action as a link, will open the link in a new tab
     */
    src?: string;
}
type ModuleView = ModuleIframeView | ModuleLaunchView | ModuleVNodeView;
interface ModuleIframeTabLazyOptions {
    description?: string;
    onLoad?: () => Promise<void>;
}
interface ModuleBuiltinTab {
    name: string;
    icon?: string;
    title?: string;
    path?: string;
    category?: TabCategory;
    defaultOrder?: number;
    show?: () => MaybeRefOrGetter<any>;
    badge?: () => MaybeRefOrGetter<number | string | undefined>;
    onClick?: () => void;
}
type ModuleTabInfo = ModuleCustomTab | ModuleBuiltinTab;
type CategorizedTabs = [TabCategory, (ModuleCustomTab | ModuleBuiltinTab)[]][];

/**
 * Severity level of a notification, mirroring devframe's message levels.
 *
 * Determines the color/icon of the entry in the Vite DevTools **Messages** dock
 * and its toast.
 */
type NuxtDevtoolsNotifyLevel = DevToolsMessageLevel;
/**
 * A Nuxt-friendly subset of devframe's `DevframeMessageEntryInput`.
 *
 * This is the input accepted by the `devtools:notify` Nuxt hook, the `notify`
 * RPC function and the injected client's `notify()` — all of which forward to
 * the connected `ctx.messages` host so notifications flow through the single
 * devframe Messages system (persistent dock list + toast overlay).
 *
 * Tiers are expressed through the flags below:
 * - **Ephemeral** (toast-only feedback like "Copied!"): `notify: true` with an
 *   `autoDismiss` (toast lifetime) and `autoDelete` (entry lifetime) so it never
 *   builds up history in the Messages dock.
 * - **Persistent** (server-originated, leveled): omit `autoDelete` so the entry
 *   is kept in the Messages dock list.
 */
interface NuxtDevtoolsNotifyInput {
    /** Short title / summary of the message. */
    message: string;
    /** Severity level. Defaults to `'info'`. */
    level?: NuxtDevtoolsNotifyLevel;
    /** Optional detailed description or explanation. */
    description?: string;
    /** Optional tags/labels for filtering in the Messages dock. */
    labels?: string[];
    /** Optional grouping category (e.g. `'build'`, `'lint'`, `'runtime'`). */
    category?: string;
    /** Optional source file position (e.g. for a build/lint error). */
    filePosition?: DevToolsMessageFilePosition;
    /** Optional stack trace string. */
    stacktrace?: string;
    /** Whether this message should also appear as a transient toast. */
    notify?: boolean;
    /** Time in ms to auto-dismiss the toast (client-side). */
    autoDismiss?: number;
    /** Time in ms to auto-delete the entry from the persistent list (server-side). */
    autoDelete?: number;
}

interface HookInfo {
    name: string;
    start: number;
    end?: number;
    duration?: number;
    listeners: number;
    executions: number[];
}
interface PackageUpdateInfo {
    name: string;
    current: string;
    latest: string;
    needsUpdate: boolean;
}
type PackageManagerName = 'npm' | 'yarn' | 'pnpm' | 'bun';
type NpmCommandType = 'install' | 'uninstall' | 'update';
interface NpmCommandOptions {
    dev?: boolean;
}
interface AutoImportsWithMetadata {
    imports: Import[];
    metadata?: UnimportMeta;
    dirs: string[];
}
interface RouteInfo extends Pick<RouteRecordNormalized, 'name' | 'path' | 'meta' | 'props' | 'children'> {
    file?: string;
}
interface ServerRouteInfo {
    route: string;
    filepath: string;
    method?: string;
    type: 'api' | 'route' | 'runtime' | 'collection';
    routes?: ServerRouteInfo[];
}
type ServerRouteInputType = 'string' | 'number' | 'boolean' | 'file' | 'date' | 'time' | 'datetime-local';
interface ServerRouteInput {
    active: boolean;
    key: string;
    value: any;
    type?: ServerRouteInputType;
}
interface Payload {
    url: string;
    time: number;
    data?: Record<string, any>;
    state?: Record<string, any>;
    functions?: Record<string, any>;
}
interface ServerTaskInfo {
    name: string;
    handler: string;
    description: string;
    type: 'collection' | 'task';
    tasks?: ServerTaskInfo[];
}
interface ScannedNitroTasks {
    tasks: {
        [name: string]: {
            handler: string;
            description: string;
        };
    };
    scheduledTasks: {
        [cron: string]: string[];
    };
}
interface PluginInfoWithMetic {
    src: string;
    mode?: 'client' | 'server' | 'all';
    ssr?: boolean;
    metric?: PluginMetric;
}
interface PluginMetric {
    src: string;
    start: number;
    end: number;
    duration: number;
}
interface LoadingTimeMetric {
    ssrStart?: number;
    appInit?: number;
    appLoad?: number;
    pageStart?: number;
    pageEnd?: number;
    pluginInit?: number;
    hmrStart?: number;
    hmrEnd?: number;
}
interface BasicModuleInfo {
    entryPath?: string;
    meta?: {
        name?: string;
    };
}
interface InstalledModuleInfo {
    name?: string;
    isPackageModule: boolean;
    isUninstallable: boolean;
    info?: ModuleStaticInfo;
    entryPath?: string;
    timings?: Record<string, number | undefined>;
    meta?: {
        name?: string;
    };
}
interface ModuleStaticInfo {
    name: string;
    description: string;
    repo: string;
    npm: string;
    icon?: string;
    github: string;
    website: string;
    learn_more: string;
    category: string;
    type: ModuleType;
    stats: ModuleStats;
    maintainers: MaintainerInfo[];
    contributors: GitHubContributor[];
    compatibility: ModuleCompatibility;
}
interface ModuleCompatibility {
    nuxt: string;
    requires: {
        bridge?: boolean | 'optional';
    };
}
interface ModuleStats {
    downloads: number;
    stars: number;
    publishedAt: number;
    createdAt: number;
}
type CompatibilityStatus = 'working' | 'wip' | 'unknown' | 'not-working';
type ModuleType = 'community' | 'official' | '3rd-party';
interface MaintainerInfo {
    name: string;
    github: string;
    twitter?: string;
}
interface GitHubContributor {
    login: string;
    name?: string;
    avatar_url?: string;
}
interface VueInspectorClient {
    enabled: boolean;
    position: {
        x: number;
        y: number;
    };
    linkParams: {
        file: string;
        line: number;
        column: number;
    };
    enable: () => void;
    disable: () => void;
    toggleEnabled: () => void;
    openInEditor: (url: URL) => void;
    onUpdated: () => void;
}
type VueInspectorData = VueInspectorClient['linkParams'] & Partial<VueInspectorClient['position']>;
interface CodeSnippet {
    code: string;
    lang: string;
    name: string;
    docs?: string;
}
interface ComponentRelationship {
    id: string;
    deps: string[];
}
interface ComponentWithRelationships {
    component: Component;
    dependencies?: string[];
    dependents?: string[];
}

/** @deprecated Part of the removed `vscode` integration. */
type CodeServerType = 'ms-code-cli' | 'ms-code-server' | 'coder-code-server';
interface ModuleOptions {
    /**
     * Enable DevTools
     *
     * @default true
     */
    enabled?: boolean;
    /**
     * Custom tabs
     *
     * This is in static format, for dynamic injection, call `nuxt.hook('devtools:customTabs')` instead
     */
    customTabs?: ModuleCustomTab[];
    /** Code Server integration options. */
    codeServer?: CodeServerIntegrationOptions;
    /**
     * Legacy VS Code Server integration options.
     *
     * @deprecated Use `codeServer`. Legacy modes are no longer supported.
     */
    vscode?: VSCodeIntegrationOptions;
    /**
     * Enable Vue Component Inspector
     *
     * @default true
     */
    componentInspector?: boolean;
    /**
     * Enable the Vue DevTools integration, which registers Vue DevTools'
     * own dock entry next to the Nuxt one.
     *
     * @default true
     */
    vueDevTools?: boolean;
    /**
     * Enable the Vite Inspect integration.
     *
     * `vite-plugin-inspect` is an optional peer dependency. When it isn't
     * installed, DevTools shows an install launcher in its place (like Vite Plus
     * DevTools); once installed, the real Inspect view is mounted. Set this to
     * `false` to disable the integration (and its launcher) entirely.
     *
     * @default true
     */
    viteInspect?: boolean;
    /**
     * Enable the Data Inspector integration, which registers the live
     * `Nuxt Application` data source and mounts the Data Inspector panel.
     *
     * @default true
     */
    dataInspector?: boolean;
    /**
     * Disable the DevTools client authorization prompt, allowing any browser to
     * connect without approving it first.
     *
     * Defaults to `true` in sandboxed environments (StackBlitz, CodeSandbox).
     *
     * Note: disabling authorization lets any browser (including other devices, if
     * you expose the dev server to your LAN/WAN) connect to DevTools and access
     * your server and filesystem. Only disable it in trusted environments.
     */
    disableAuthorization?: boolean;
    /**
     * Experimental features
     */
    experimental?: {
        /**
         * Timeline tab
         * @deprecated Use `timeline.enable` instead
         */
        timeline?: boolean;
    };
    /**
     * Options for the timeline tab
     */
    timeline?: {
        /**
         * Enable timeline tab
         *
         * @default false
         */
        enabled?: boolean;
        /**
         * Track on function calls
         */
        functions?: {
            include?: (string | RegExp | ((item: Import) => boolean))[];
            /**
             * Include from specific modules
             *
             * @default ['#app', '@unhead/vue']
             */
            includeFrom?: string[];
            exclude?: (string | RegExp | ((item: Import) => boolean))[];
        };
    };
    /**
     * Options for assets tab
     */
    assets?: {
        /**
         * Allowed file extensions for assets tab to upload.
         * To security concern.
         *
         * Set to '*' to disbale this limitation entirely
         *
         * @default Common media and txt files
         */
        uploadExtensions?: '*' | string[];
    };
    /**
     * Enable anonymous telemetry, helping us improve Nuxt DevTools.
     *
     * By default it will respect global Nuxt telemetry settings.
     */
    telemetry?: boolean;
}
interface CodeServerIntegrationOptions {
    /**
     * Enable the Code Server integration.
     *
     * @default true
     */
    enabled?: boolean;
    /** Path or command name for Coder's `code-server` binary. */
    bin?: string;
    /** Workspace opened by Code Server. Defaults to the Nuxt root directory. */
    cwd?: string;
    /** Port for the Code Server process. Defaults to the plugin's free-port behavior. */
    serverPort?: number;
    /** Host for the Code Server process. Defaults to the plugin's loopback host. */
    host?: string;
    /** Additional safe arguments passed to `code-server`. */
    args?: string[];
    /** Additional safe environment variables passed to `code-server`. */
    env?: Record<string, string>;
    /** Suffix used to isolate the authenticated Code Server session cookie. */
    cookieSuffix?: string;
    /** Milliseconds to wait for Code Server to become ready. */
    startTimeout?: number;
}
/** @deprecated Use {@link CodeServerIntegrationOptions}. */
interface VSCodeIntegrationOptions {
    /**
     * Enable VS Code Server integration
     */
    enabled?: boolean;
    /**
     * Start VS Code Server on boot
     *
     * @default false
     */
    startOnBoot?: boolean;
    /**
     * Port to start VS Code Server
     *
     * @default 3080
     */
    port?: number;
    /**
     * Reuse existing server if available (same port)
     */
    reuseExistingServer?: boolean;
    /**
     * Determine whether to use code-server or vs code tunnel
     *
     * @default 'local-serve'
     */
    mode?: 'local-serve' | 'tunnel';
    /**
     * Options for VS Code tunnel
     */
    tunnel?: VSCodeTunnelOptions;
    /**
     * Determines which binary and arguments to use for VS Code.
     *
     * By default, uses the MS Code Server (ms-code-server).
     * Can alternatively use the open source Coder code-server (coder-code-server),
     * or the MS VS Code CLI (ms-code-cli)
     *  @default 'ms-code-server'
     */
    codeServer?: CodeServerType;
    /**
     * Host address to listen on. Unspecified by default.
     */
    host?: string;
}
/** @deprecated Tunnels are not supported by the Code Server integration. */
interface VSCodeTunnelOptions {
    /**
     * the machine name for port forwarding service
     *
     * default: device hostname
     */
    name?: string;
}
interface NuxtDevToolsOptions {
    behavior: {
        telemetry: boolean | null;
        openInEditor: string | undefined;
    };
    ui: {
        componentsGraphShowGlobalComponents: boolean;
        componentsGraphShowLayouts: boolean;
        componentsGraphShowNodeModules: boolean;
        componentsGraphShowPages: boolean;
        componentsGraphShowWorkspace: boolean;
        componentsView: 'list' | 'graph';
        hiddenTabCategories: string[];
        hiddenTabs: string[];
        pinnedTabs: string[];
        scale: number;
        showExperimentalFeatures: boolean;
        showHelpButtons: boolean;
    };
    serverRoutes: {
        selectedRoute: ServerRouteInfo | null;
        view: 'tree' | 'list';
        inputDefaults: Record<string, ServerRouteInput[]>;
        sendFrom: 'app' | 'devtools';
    };
    serverTasks: {
        enabled: boolean;
        selectedTask: ServerTaskInfo | null;
        view: 'tree' | 'list';
        inputDefaults: Record<string, ServerRouteInput[]>;
    };
    assets: {
        view: 'grid' | 'list';
    };
}

interface AnalyzeBuildMeta extends NuxtAnalyzeMeta {
    features: {
        bundleClient: boolean;
        bundleNitro: boolean;
        viteInspect: boolean;
    };
    size: {
        clientBundle?: number;
        nitroBundle?: number;
    };
}
interface AnalyzeBuildsInfo {
    isBuilding: boolean;
    /**
     * Unique id of the terminal session for the build currently in flight, or
     * `undefined` when idle. The client reveals this session and derives its
     * "Building…" state from it instead of an `onTerminalExit` broadcast.
     */
    activeSessionId?: string;
    builds: AnalyzeBuildMeta[];
}

/**
 * `nitropack` (Nitro v2) and `nitro` (Nitro v3) are both declared as *optional*
 * peer dependencies, so a consumer only ever has one installed (Nuxt 4 →
 * `nitropack`, Nuxt 5 → `nitro`). A missing peer resolves its `import type` to
 * `any`, which would collapse a naive `V2 | V3` union to `any`. Detect which
 * package actually resolved — `keyof any` matches every key, so probing an
 * impossible `'___INVALID'` key tells a real Nitro type from the `any`
 * fallback — and resolve to just that one. Mirrors `@nuxt/kit`'s own detection.
 */
type HasNitroV2 = 'options' extends keyof Nitro ? ('___INVALID' extends keyof Nitro ? false : true) : false;
type HasNitroV3 = 'options' extends keyof Nitro$1 ? ('___INVALID' extends keyof Nitro$1 ? false : true) : false;
type AnyNitro = HasNitroV2 extends true ? (HasNitroV3 extends true ? Nitro | Nitro$1 : Nitro) : Nitro$1;
type AnyStorageMounts = HasNitroV2 extends true ? (HasNitroV3 extends true ? StorageMounts | StorageMounts$1 : StorageMounts) : StorageMounts$1;

interface ServerFunctions {
    getServerConfig: () => NuxtOptions;
    getServerDebugContext: () => Promise<ServerDebugContext | undefined>;
    /**
     * @deprecated Replaced by the Data Inspector panel's live `Nuxt Application`
     * source. Kept as a compatibility shim (emits `NDT_DEP_0009`) for one
     * migration window and will be removed in a future major.
     */
    getServerData: () => Promise<NuxtServerData>;
    getServerRuntimeConfig: () => Record<string, any>;
    getModuleOptions: () => ModuleOptions;
    getComponents: () => Component[];
    getComponentsRelationships: () => Promise<ComponentRelationship[]>;
    getAutoImports: () => Promise<AutoImportsWithMetadata>;
    getServerPages: () => NuxtPage[];
    getCustomTabs: () => ModuleCustomTab[];
    getServerHooks: () => HookInfo[];
    getServerLayouts: () => NuxtLayout[];
    getServerRoutes: () => ServerRouteInfo[];
    getServerTasks: () => ScannedNitroTasks | null;
    getServerApp: () => NuxtApp | undefined;
    getOptions: <T extends keyof NuxtDevToolsOptions>(tab: T) => Promise<NuxtDevToolsOptions[T]>;
    updateOptions: <T extends keyof NuxtDevToolsOptions>(tab: T, settings: Partial<NuxtDevToolsOptions[T]>) => Promise<void>;
    clearOptions: () => Promise<void>;
    checkForUpdateFor: (name: string) => Promise<PackageUpdateInfo | undefined>;
    getNpmCommand: (command: NpmCommandType, packageName: string, options?: NpmCommandOptions) => Promise<string[] | undefined>;
    runNpmCommand: (command: NpmCommandType, packageName: string, options?: NpmCommandOptions) => Promise<{
        processId: string;
    } | undefined>;
    revealTerminal: (id: string) => Promise<boolean>;
    getStorageMounts: () => Promise<AnyStorageMounts>;
    getStorageKeys: (base?: string) => Promise<string[]>;
    getStorageItem: (key: string) => Promise<StorageValue>;
    setStorageItem: (key: string, value: StorageValue) => Promise<void>;
    removeStorageItem: (key: string) => Promise<void>;
    getAnalyzeBuildInfo: () => Promise<AnalyzeBuildsInfo>;
    generateAnalyzeBuildName: () => Promise<string>;
    startAnalyzeBuild: (name: string) => Promise<string>;
    clearAnalyzeBuilds: (names?: string[]) => Promise<void>;
    notify: (input: NuxtDevtoolsNotifyInput) => Promise<void>;
    telemetryEvent: (payload: object, immediate?: boolean) => void;
    customTabAction: (name: string, action: number) => Promise<boolean>;
    enablePages: () => Promise<void>;
    openInEditor: (filepath: string) => Promise<boolean>;
    restartNuxt: (hard?: boolean) => Promise<void>;
    installNuxtModule: (name: string, dry?: boolean, sessionId?: string) => Promise<InstallModuleReturn>;
    uninstallNuxtModule: (name: string, dry?: boolean, sessionId?: string) => Promise<InstallModuleReturn>;
    enableTimeline: (dry: boolean) => Promise<[string, string]>;
    requestForAuth: (info?: string, origin?: string) => Promise<void>;
    verifyAuthToken: () => Promise<boolean>;
}
interface ClientFunctions {
    refresh: (event: ClientUpdateEvent) => void;
    callHook: (hook: string, ...args: any[]) => Promise<void>;
    navigateTo: (path: string) => void;
    /**
     * Minimal server→client completion signal for generic package updates only
     * (`runNpmCommand`): the run RPC returns before the process exits, so this
     * lets `usePackageUpdate` and the restart prompt settle once it finishes. It
     * carries only `{ id, code }` and is not a terminal-data transport. Module
     * install/uninstall and analyze-build no longer rely on it — they clear their
     * UI from the awaited RPC / refreshed info instead.
     */
    onTerminalExit: (_: {
        id: string;
        code?: number;
    }) => void;
}
/**
 * @deprecated The payload of the deprecated {@link ServerFunctions.getServerData}
 * shim. Use the Data Inspector panel's live `Nuxt Application` source instead.
 */
interface NuxtServerData {
    nuxt: NuxtOptions;
    nitro?: AnyNitro['options'];
    vite: {
        server?: ResolvedConfig;
        client?: ResolvedConfig;
    };
}
type ClientUpdateEvent = keyof ServerFunctions;

/**
 * Legacy Nuxt DevTools RPC compatibility surface exposed on `nuxt.devtools.rpc`.
 *
 * For new integrations prefer {@link onDevtoolsReady}, where the connected
 * `ViteDevToolsNodeContext` gives you the full devframe `ctx.rpc`
 * (`register`/`invokeLocal`/`broadcast`/`sharedState`/…).
 */
interface NuxtDevtoolsRpc {
    /**
     * Broadcast proxy for calling client functions.
     * Supports `rpc.broadcast.refresh.asEvent(event)` for backward compatibility.
     */
    broadcast: {
        [K in keyof ClientFunctions]: ClientFunctions[K] & {
            asEvent: ClientFunctions[K];
        };
    };
    /**
     * Proxy for reading/writing server functions locally.
     */
    functions: ServerFunctions;
}
/**
 * @internal
 */
interface NuxtDevtoolsServerContext {
    nuxt: Nuxt;
    options: ModuleOptions;
    rpc: NuxtDevtoolsRpc;
    /**
     * The connected Vite DevTools kit context (`docks`/`terminals`/`messages`/
     * `commands`/`rpc`/`diagnostics`/…).
     *
     * This is the raw escape hatch and is `undefined` until the Vite DevTools
     * plugin connects. Prefer {@link onDevtoolsReady}, which hands you the
     * connected context.
     */
    devtoolsKit: ViteDevToolsNodeContext | undefined;
    /**
     * Hook to open file in editor
     */
    openInEditorHooks: ((filepath: string) => boolean | void | Promise<boolean | void>)[];
    /**
     * Invalidate client cache for a function and ask for re-fetching
     */
    refresh: (event: keyof ServerFunctions) => void;
    /**
     * Push a notification through the devframe Messages system (`ctx.messages`).
     *
     * The connected messages host surfaces it in the Vite DevTools **Messages**
     * dock and/or as a toast. Calls made before the kit connects are buffered and
     * replayed on connect. Used by the `devtools:notify` hook, the `notify` RPC
     * function and the curated built-in notification sources.
     */
    notify: (input: NuxtDevtoolsNotifyInput) => void;
    /**
     * @deprecated Use the Vite DevTools RPC registration instead:
     * `nuxt.devtools.rpc.register(defineRpcFunction(...))`. Kept working as a shim.
     */
    extendServerRpc: <ClientFunctions extends object = Record<string, unknown>, ServerFunctions extends object = Record<string, unknown>>(name: string, functions: ServerFunctions) => BirpcGroup<ClientFunctions, ServerFunctions>;
}
interface NuxtDevtoolsInfo {
    version: string;
    packagePath: string;
}
interface InstallModuleReturn {
    configOriginal: string;
    configGenerated: string;
    commands: string[];
    processId: string;
}
type ServerDebugModuleMutationRecord = (Omit<NuxtDebugModuleMutationRecord, 'module'> & {
    name: string;
});
interface ServerDebugContext {
    moduleMutationRecords: ServerDebugModuleMutationRecord[];
}

interface TerminalBase {
    id: string;
    name: string;
    description?: string;
    icon?: string;
}
type TerminalAction = 'restart' | 'terminate' | 'clear' | 'remove';
interface SubprocessOptions {
    command: string;
    args?: string[];
    cwd?: string;
    env?: Record<string, string | undefined>;
    nodeOptions?: SpawnOptions;
}
interface TerminalInfo extends TerminalBase {
    /**
     * Whether the terminal can be restarted.
     *
     * @deprecated Ignored since v4: legacy terminals are bridged onto the built-in
     * Terminals dock as read-only, output-only sessions, so Devframe shows no
     * restart control. Restart a `startSubprocess()`-owned process through its
     * returned handle instead. Will be removed in v5.
     */
    restartable?: boolean;
    /**
     * Whether the terminal can be terminated.
     *
     * @deprecated Ignored since v4 (see {@link TerminalInfo.restartable}). Will be
     * removed in v5.
     */
    terminatable?: boolean;
    /**
     * Whether the terminal is terminated
     */
    isTerminated?: boolean;
    /**
     * Content buffer
     */
    buffer?: string;
}
interface TerminalState extends TerminalInfo {
    /**
     * User action to restart the terminal, when not provided, this action will be disabled.
     *
     * @deprecated Ignored since v4: the bridge to the built-in Terminals dock
     * cannot attach action callbacks to an externally registered session. Will be
     * removed in v5.
     */
    onActionRestart?: () => Promise<void> | void;
    /**
     * User action to terminate the terminal, when not provided, this action will be disabled.
     *
     * @deprecated Ignored since v4 (see {@link TerminalState.onActionRestart}).
     * Will be removed in v5.
     */
    onActionTerminate?: () => Promise<void> | void;
}

declare module '@nuxt/schema' {
    interface NuxtHooks {
        /**
         * Called before devtools starts. Useful to detect if devtools is enabled.
         */
        'devtools:before': () => void;
        /**
         * Called after devtools is initialized.
         */
        'devtools:initialized': (info: NuxtDevtoolsInfo) => void;
        /**
         * Called once the Vite DevTools kit has connected, with the connected
         * `ViteDevToolsNodeContext`.
         *
         * This is the recommended place to do all DevTools integration
         * (registering docks, terminals, messages, commands, RPC functions,
         * diagnostics, …): the kit is guaranteed to be available here, so you don't
         * need the connect-safe accessors on `nuxt.devtools`.
         */
        'devtools:ready': (ctx: ViteDevToolsNodeContext) => void | Promise<void>;
        /**
         * Push a notification through the devframe Messages system.
         *
         * Forwarded to the connected `ctx.messages` host, so it surfaces in the
         * Vite DevTools **Messages** dock (persistent, when leveled) and/or as a
         * transient toast (when `notify` is set). Calls made before the kit connects
         * are buffered and replayed once it does.
         *
         * @example
         * ```ts
         * nuxt.callHook('devtools:notify', { message: 'Build failed', level: 'error' })
         * ```
         */
        'devtools:notify': (input: NuxtDevtoolsNotifyInput) => void;
        /**
         * Hooks to extend devtools tabs.
         */
        'devtools:customTabs': (tabs: ModuleCustomTab[]) => void;
        /**
         * Retrigger update for custom tabs, `devtools:customTabs` will be called again.
         */
        'devtools:customTabs:refresh': () => void;
        /**
         * Register a terminal whose process is owned by the caller (module).
         *
         * The registered session is surfaced **read-only** in the built-in Vite
         * DevTools **Terminals** dock; stream output into it via
         * `devtools:terminal:write`.
         */
        'devtools:terminal:register': (terminal: TerminalState) => void;
        /**
         * Write to a terminal.
         *
         * Returns true if terminal is found.
         */
        'devtools:terminal:write': (_: {
            id: string;
            data: string;
        }) => void;
        /**
         * Remove a terminal from devtools.
         *
         * Returns true if terminal is found and deleted.
         */
        'devtools:terminal:remove': (_: {
            id: string;
        }) => void;
        /**
         * Mark a terminal as terminated.
         */
        'devtools:terminal:exit': (_: {
            id: string;
            code?: number;
        }) => void;
    }
}

export type { ServerDebugModuleMutationRecord as $, AnalyzeBuildMeta as A, BasicModuleInfo as B, CategorizedTabs as C, ModuleVNodeView as D, ModuleView as E, NpmCommandOptions as F, GitHubContributor as G, HookInfo as H, InstallModuleReturn as I, NpmCommandType as J, NuxtDevToolsOptions as K, LoadingTimeMetric as L, ModuleCustomTab as M, NuxtDevtoolsServerContext as N, NuxtDevtoolsNotifyLevel as O, PluginMetric as P, NuxtDevtoolsRpc as Q, NuxtServerData as R, SubprocessOptions as S, TerminalState as T, PackageManagerName as U, PackageUpdateInfo as V, Payload as W, PluginInfoWithMetic as X, RouteInfo as Y, ScannedNitroTasks as Z, ServerDebugContext as _, NuxtDevtoolsInfo as a, ServerRouteInfo as a0, ServerRouteInput as a1, ServerRouteInputType as a2, ServerTaskInfo as a3, TabCategory as a4, TerminalAction as a5, TerminalBase as a6, TerminalInfo as a7, VSCodeIntegrationOptions as a8, VSCodeTunnelOptions as a9, VueInspectorClient as aa, VueInspectorData as ab, ServerFunctions as b, NuxtDevtoolsNotifyInput as c, AnalyzeBuildsInfo as d, AutoImportsWithMetadata as e, ClientFunctions as f, ClientUpdateEvent as g, CodeServerIntegrationOptions as h, CodeServerType as i, CodeSnippet as j, CompatibilityStatus as k, ComponentRelationship as l, ComponentWithRelationships as m, InstalledModuleInfo as n, MaintainerInfo as o, ModuleBuiltinTab as p, ModuleCompatibility as q, ModuleIframeTabLazyOptions as r, ModuleIframeView as s, ModuleLaunchAction as t, ModuleLaunchView as u, ModuleOptions as v, ModuleStaticInfo as w, ModuleStats as x, ModuleTabInfo as y, ModuleType as z };
