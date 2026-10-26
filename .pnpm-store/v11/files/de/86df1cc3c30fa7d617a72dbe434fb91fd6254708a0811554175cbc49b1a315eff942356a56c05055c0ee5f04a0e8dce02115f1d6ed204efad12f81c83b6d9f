import { DevToolsRpcClient } from '@vitejs/devtools-kit/client';
import { Ref } from 'vue';
import '../shared/devtools-kit.CtfaEBN3.mjs';
import { NuxtDevtoolsIframeClient } from '../types.mjs';
import '@vitejs/devtools-kit';
import 'birpc';
import 'nuxt/schema';
import 'unimport';
import 'vue-router';
import 'unstorage';
import 'vite';
import '@nuxt/schema';
import 'nitro/types';
import 'nitropack';
import 'node:child_process';
import 'hookable';
import 'nuxt/app';
import 'ofetch';
import 'shiki';
import 'error-stack-parser-es';

declare function onDevtoolsClientConnected(fn: (client: NuxtDevtoolsIframeClient) => void): (() => void) | undefined;
/**
 * Run a callback once the Vite DevTools client is ready, receiving the connected
 * `DevToolsRpcClient` — the client-side mirror of the server's
 * `onDevtoolsReady((ctx) => …)`.
 *
 * This is the recommended way to do client-side DevTools integration (register
 * client RPC functions, call server functions, shared state, streaming, …):
 *
 * ```ts
 * import { onDevtoolsReady } from '@nuxt/devtools-kit/iframe-client'
 *
 * onDevtoolsReady((kit) => {
 *   kit.client.register({ name: 'my-module:on-update', type: 'event', handler })
 * })
 * ```
 */
declare function onDevtoolsReady(fn: (kit: DevToolsRpcClient) => void): (() => void) | undefined;
declare function useDevtoolsClient(): Ref<NuxtDevtoolsIframeClient | undefined, NuxtDevtoolsIframeClient | undefined>;

export { onDevtoolsClientConnected, onDevtoolsReady, useDevtoolsClient };
