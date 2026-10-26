import { H as HookInfo, P as PluginMetric, L as LoadingTimeMetric, b as ServerFunctions, c as NuxtDevtoolsNotifyInput } from './shared/devtools-kit.CtfaEBN3.mjs';
export { A as AnalyzeBuildMeta, d as AnalyzeBuildsInfo, e as AutoImportsWithMetadata, B as BasicModuleInfo, C as CategorizedTabs, f as ClientFunctions, g as ClientUpdateEvent, h as CodeServerIntegrationOptions, i as CodeServerType, j as CodeSnippet, k as CompatibilityStatus, l as ComponentRelationship, m as ComponentWithRelationships, G as GitHubContributor, I as InstallModuleReturn, n as InstalledModuleInfo, o as MaintainerInfo, p as ModuleBuiltinTab, q as ModuleCompatibility, M as ModuleCustomTab, r as ModuleIframeTabLazyOptions, s as ModuleIframeView, t as ModuleLaunchAction, u as ModuleLaunchView, v as ModuleOptions, w as ModuleStaticInfo, x as ModuleStats, y as ModuleTabInfo, z as ModuleType, D as ModuleVNodeView, E as ModuleView, F as NpmCommandOptions, J as NpmCommandType, K as NuxtDevToolsOptions, a as NuxtDevtoolsInfo, O as NuxtDevtoolsNotifyLevel, Q as NuxtDevtoolsRpc, N as NuxtDevtoolsServerContext, R as NuxtServerData, U as PackageManagerName, V as PackageUpdateInfo, W as Payload, X as PluginInfoWithMetic, Y as RouteInfo, Z as ScannedNitroTasks, _ as ServerDebugContext, $ as ServerDebugModuleMutationRecord, a0 as ServerRouteInfo, a1 as ServerRouteInput, a2 as ServerRouteInputType, a3 as ServerTaskInfo, S as SubprocessOptions, a4 as TabCategory, a5 as TerminalAction, a6 as TerminalBase, a7 as TerminalInfo, T as TerminalState, a8 as VSCodeIntegrationOptions, a9 as VSCodeTunnelOptions, aa as VueInspectorClient, ab as VueInspectorData } from './shared/devtools-kit.CtfaEBN3.mjs';
import { DevToolsRpcClient } from '@vitejs/devtools-kit/client';
import { Hookable } from 'hookable';
import { NuxtApp } from 'nuxt/app';
import { AppConfig } from 'nuxt/schema';
import { $Fetch } from 'ofetch';
import { BuiltinLanguage } from 'shiki';
import { Ref } from 'vue';
import { StackFrame } from 'error-stack-parser-es';
import '@vitejs/devtools-kit';
import 'birpc';
import 'unimport';
import 'vue-router';
import 'unstorage';
import 'vite';
import '@nuxt/schema';
import 'nitro/types';
import 'nitropack';
import 'node:child_process';

interface TimelineEventFunction {
    type: 'function';
    start: number;
    end?: number;
    name: string;
    args?: any[];
    result?: any;
    stacktrace?: StackFrame[];
    isPromise?: boolean;
}
interface TimelineServerState {
    timeSsrStart?: number;
}
interface TimelineEventRoute {
    type: 'route';
    start: number;
    end?: number;
    from: string;
    to: string;
}
interface TimelineOptions {
    enabled: boolean;
    stacktrace: boolean;
    arguments: boolean;
}
type TimelineEvent = TimelineEventFunction | TimelineEventRoute;
interface TimelineMetrics {
    events: TimelineEvent[];
    nonLiteralSymbol: symbol;
    options: TimelineOptions;
}
interface TimelineEventNormalized<T> {
    event: T;
    segment: TimelineEventsSegment;
    relativeStart: number;
    relativeWidth: number;
    layer: number;
}
interface TimelineEventsSegment {
    start: number;
    end: number;
    events: TimelineEvent[];
    functions: TimelineEventNormalized<TimelineEventFunction>[];
    route?: TimelineEventNormalized<TimelineEventRoute>;
    duration: number;
    previousGap?: number;
}

interface NuxtDevtoolsClientHooks {
    /**
     * When the DevTools navigates, used for persisting the current tab
     */
    'devtools:navigate': (path: string) => void;
    /**
     * Event emitted when the component inspector is clicked
     */
    'host:inspector:click': (path: string) => void;
    /**
     * Event to close the component inspector
     */
    'host:inspector:close': () => void;
    /**
     * Triggers reactivity manually, since Vue won't be reactive across frames)
     */
    'host:update:reactivity': () => void;
    /**
     * Host action to control the DevTools navigation
     */
    'host:action:navigate': (path: string) => void;
    /**
     * Host action to reload the DevTools
     */
    'host:action:reload': () => void;
}
/**
 * Host client from the App
 */
interface NuxtDevtoolsHostClient {
    nuxt: NuxtApp;
    hooks: Hookable<NuxtDevtoolsClientHooks>;
    inspector?: {
        enable: () => void;
        disable: () => void;
        toggle: () => void;
        isEnabled: Ref<boolean>;
        isAvailable: Ref<boolean>;
    };
    devtools: {
        close: () => void;
        open: () => void;
        toggle: () => void;
        reload: () => void;
        navigate: (path: string) => void;
    };
    app: {
        reload: () => void;
        navigate: (path: string, hard?: boolean) => void;
        appConfig: AppConfig;
        colorMode: Ref<'dark' | 'light'>;
        $fetch: $Fetch;
    };
    metrics: {
        clientHooks: () => HookInfo[];
        clientPlugins: () => PluginMetric[] | undefined;
        clientTimeline: () => TimelineMetrics | undefined;
        loading: () => LoadingTimeMetric;
    };
    /**
     * A counter to trigger reactivity updates
     */
    revision: Ref<number>;
}
interface CodeHighlightOptions {
    grammarContextCode?: string;
}
type AsyncServerFunctions = {
    [K in keyof ServerFunctions]: (...args: Parameters<ServerFunctions[K]>) => Promise<Awaited<ReturnType<ServerFunctions[K]>>>;
};
interface NuxtDevtoolsClient {
    rpc: AsyncServerFunctions;
    renderCodeHighlight: (code: string, lang?: BuiltinLanguage, options?: CodeHighlightOptions) => {
        code: string;
        supported: boolean;
    };
    renderMarkdown: (markdown: string) => string;
    colorMode: string;
    /**
     * Push a notification through the devframe Messages system.
     *
     * Routes to the shared Messages host, so it surfaces in the Vite DevTools
     * **Messages** dock and/or as a transient toast (when `notify` is set — the
     * default). Use this from custom tabs / in-client code to give feedback
     * through the same notification system as the rest of DevTools.
     */
    notify: (input: NuxtDevtoolsNotifyInput) => Promise<void>;
    /**
     * The connected Vite DevTools RPC client, mirroring `nuxt.devtools.devtoolsKit`
     * on the server. Use it for full devframe-native access — register client RPC
     * functions (`devtoolsKit.client.register(...)`), call server functions
     * (`devtoolsKit.call(...)`), shared state, streaming, scoping, etc.
     *
     * `undefined` until the DevTools client has connected.
     */
    devtoolsKit: DevToolsRpcClient | undefined;
    /**
     * @deprecated Use `onDevtoolsReady((kit) => kit.client.register(...))` from
     * `@nuxt/devtools-kit/iframe-client`, or the exposed `devtoolsKit` client
     * directly (`client.devtools.devtoolsKit?.client.register(...)`).
     */
    extendClientRpc: <ServerFunctions extends object = Record<string, unknown>, ClientFunctions extends object = Record<string, unknown>>(name: string, functions: ClientFunctions) => ServerFunctions;
}
interface NuxtDevtoolsIframeClient {
    host: NuxtDevtoolsHostClient;
    devtools: NuxtDevtoolsClient;
}

export { HookInfo, LoadingTimeMetric, NuxtDevtoolsNotifyInput, PluginMetric, ServerFunctions };
export type { AsyncServerFunctions, CodeHighlightOptions, NuxtDevtoolsClient, NuxtDevtoolsClientHooks, NuxtDevtoolsHostClient, NuxtDevtoolsIframeClient, TimelineEvent, TimelineEventFunction, TimelineEventNormalized, TimelineEventRoute, TimelineEventsSegment, TimelineMetrics, TimelineOptions, TimelineServerState };
