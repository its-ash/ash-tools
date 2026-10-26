import { CodeServerDetection, CodeServerSharedState, CodeServerStartRequest, CodeServerStatusResult } from "../types.mjs";
import "../index.mjs";
import { RpcDefinitionsToFunctions } from "devframe/rpc";
//#region src/node/rpc/index.d.ts
export declare const serverFunctions: readonly [import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:code-server:detect", "query", [], Promise<CodeServerDetection>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:code-server:status", "query", [], CodeServerStatusResult, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:code-server:start", "action", [req?: CodeServerStartRequest | undefined], Promise<CodeServerStatusResult>, undefined, undefined, import("devframe").DevframeNodeContext>, import("devframe/rpc").RpcFunctionDefinitionWithoutSchemas<"devframes:plugin:code-server:stop", "action", [], CodeServerStatusResult, undefined, undefined, import("devframe").DevframeNodeContext>];
declare module 'devframe' {
  interface DevframeRpcServerFunctions extends RpcDefinitionsToFunctions<typeof serverFunctions> {}
  interface DevframeRpcSharedStates {
    'devframes:plugin:code-server:state': CodeServerSharedState;
  }
}
//#endregion