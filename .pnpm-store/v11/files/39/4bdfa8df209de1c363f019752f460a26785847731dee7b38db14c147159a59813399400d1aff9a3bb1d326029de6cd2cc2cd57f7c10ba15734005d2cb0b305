import { RpcDefinitionsToFunctions } from "devframe/rpc";
//#region src/node/rpc/index.d.ts
/**
 * The message-feed RPC functions registered by the plugin: thin, typed
 * wrappers over the hub's `ctx.messages` host. Namespaced
 * `devframes:plugin:messages:*` per the plugin convention.
 */
export declare const serverFunctions: readonly [import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:messages:list", "query", [since?: number | null | undefined], Promise<import("@devframes/hub/types").DevframeMessagesListDelta>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:messages:add", "action", [input: import("@devframes/hub/types").DevframeMessageEntryInput], Promise<import("@devframes/hub/types").DevframeMessageEntry | null>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:messages:update", "action", [id: string, patch: Partial<import("@devframes/hub/types").DevframeMessageEntryInput>], Promise<import("@devframes/hub/types").DevframeMessageEntry | null>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:messages:remove", "action", [id: string], Promise<void>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:messages:clear", "action", [], Promise<void>, undefined, undefined, import("devframe").DevframeNodeContext>];
declare module 'devframe' {
  interface DevframeRpcServerFunctions extends RpcDefinitionsToFunctions<typeof serverFunctions> {}
}
//#endregion