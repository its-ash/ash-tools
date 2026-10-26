import { n as createResolveMetadataRpc, r as resolveMetadata, t as ResolveMetadataOptions } from "../../resolve-metadata-CpI-ucF6.mjs";
import { RpcDefinitionsToFunctions } from "devframe/rpc";
//#region src/node/rpc/index.d.ts
export declare const serverFunctions: readonly [import("devframe/rpc").RpcFunctionDefinitionWithSchemas<"devframes:plugin:og:resolve-metadata", "query", readonly [import("devframe/utils/simple-schema").SimpleSchema<{
  url?: string | undefined;
}, {
  url?: string | undefined;
}>], import("devframe/utils/simple-schema").SimpleSchema<{
  requestedUrl: string;
  url: string;
  status: number;
  fetchedAt: number;
  tags: {
    tag: "html" | "link" | "meta" | "title";
    name: string;
    value: string;
  }[];
}, {
  requestedUrl: string;
  url: string;
  status: number;
  fetchedAt: number;
  tags: {
    tag: "html" | "link" | "meta" | "title";
    name: string;
    value: string;
  }[];
}>, import("devframe").DevframeNodeContext>];
declare module 'devframe' {
  interface DevframeRpcServerFunctions extends RpcDefinitionsToFunctions<typeof serverFunctions> {}
}
//#endregion
export { type ResolveMetadataOptions, createResolveMetadataRpc, resolveMetadata };