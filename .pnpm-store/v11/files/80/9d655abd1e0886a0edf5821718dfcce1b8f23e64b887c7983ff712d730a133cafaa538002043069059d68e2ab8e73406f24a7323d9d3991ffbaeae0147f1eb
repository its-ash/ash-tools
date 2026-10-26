import { PLUGIN_ID } from "./node/constants.mjs";
import { n as getMessagesHost, t as serverFunctions } from "./rpc-SrdqlzWN.mjs";
import { defineDiagnostics } from "devframe/utils/nostics";
//#region src/node/diagnostics.ts
/**
* Structured diagnostics for `@devframes/plugin-messages`. Node-side only.
* Codes use the plugin-private `DP_MESSAGES_` band (see the built-in
* plugins planning index) so they never collide with devframe core
* (`DF00xx`) or `@devframes/hub` (`DF80xx`).
*/
const diagnostics = defineDiagnostics({
	docsBase: "https://devfra.me/errors",
	codes: { DP_MESSAGES_0001: {
		why: (p) => `"${p.id}" is mounted on a context without a hub messages host (\`ctx.messages\`); its RPC surface stays registered but no-ops, so the panel will show an empty feed.`,
		fix: "Mount this devframe through a hub host (`@devframes/hub`'s `initHub`, or `createHubContext` + `ctx.install`) to get a live message feed."
	} }
});
//#endregion
//#region src/node/setup.ts
/**
* Register the message-feed RPC functions on a devframe node context.
* Called from the definition's `setup(ctx)` and reusable by host adapters
* that wire their own context.
*
* The plugin reads the feed from the hub-attached `ctx.messages` host. On a
* plain (non-hub) context it warns once and keeps the RPC surface registered
* as no-ops, so the panel still renders, with an empty feed.
*/
function setupMessages(ctx) {
	if (!getMessagesHost(ctx)) diagnostics.DP_MESSAGES_0001({ id: PLUGIN_ID });
	for (const fn of serverFunctions) ctx.rpc.register(fn);
}
//#endregion
export { setupMessages as t };
