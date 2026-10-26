import { serverFunctions } from "./rpc/index.mjs";
import { DevframeNodeContext } from "devframe";
//#region src/node/setup.d.ts
/**
 * Register the message-feed RPC functions on a devframe node context.
 * Called from the definition's `setup(ctx)` and reusable by host adapters
 * that wire their own context.
 *
 * The plugin reads the feed from the hub-attached `ctx.messages` host. On a
 * plain (non-hub) context it warns once and keeps the RPC surface registered
 * as no-ops, so the panel still renders, with an empty feed.
 */
export declare function setupMessages(ctx: DevframeNodeContext): void;
//#endregion
export { serverFunctions };