import { defineRpcFunction } from "devframe";
//#region src/node/rpc/functions/_define.ts
/**
* Read the hub-attached messages host off a node context, if present. The
* plugin talks to `ctx.messages` structurally so its shipped code carries no
* runtime dependency on `@devframes/hub`, so hosts other than the hub can
* satisfy the same surface.
*/
function getMessagesHost(ctx) {
	return ctx.messages;
}
//#endregion
//#region src/node/rpc/functions/add.ts
/**
* Add a message entry from a browser client. The origin is force-stamped
* `from: 'browser'` so client-originated entries can't impersonate the
* server. Returns the stored entry (with generated id/timestamp), or `null`
* when no messages host is attached.
*/
const messagesAdd = defineRpcFunction({
	name: "devframes:plugin:messages:add",
	type: "action",
	jsonSerializable: true,
	setup: (ctx) => ({ handler: async (input) => {
		const host = getMessagesHost(ctx);
		if (!host) return null;
		return (await host.add({
			...input,
			from: "browser"
		})).entry;
	} })
});
//#endregion
//#region src/node/rpc/functions/clear.ts
/** Clear the whole message feed. */
const messagesClear = defineRpcFunction({
	name: "devframes:plugin:messages:clear",
	type: "action",
	jsonSerializable: true,
	setup: (ctx) => ({ handler: async () => {
		await getMessagesHost(ctx)?.clear();
	} })
});
//#endregion
//#region src/node/rpc/functions/list.ts
/**
* Read the message list incrementally. Pass the `version` from the previous
* result as `since` to receive only what changed; omit it for the initial
* full snapshot. Apply `removedIds` before upserting `entries`.
*
* `snapshot: true` bakes the full list into static builds, so the panel
* renders the last captured feed without a live server.
*/
const messagesList = defineRpcFunction({
	name: "devframes:plugin:messages:list",
	type: "query",
	jsonSerializable: true,
	snapshot: true,
	agent: {
		title: "List messages",
		description: "List the hub message feed: diagnostics, notifications, and tool output entries with level, category, labels, and positions."
	},
	setup: (ctx) => ({ handler: async (since) => {
		const host = getMessagesHost(ctx);
		if (!host) return {
			entries: [],
			removedIds: [],
			version: 0,
			full: true
		};
		return host.listSince(since);
	} })
});
//#endregion
//#region src/node/rpc/functions/remove.ts
/** Remove (dismiss) a single message entry by id. */
const messagesRemove = defineRpcFunction({
	name: "devframes:plugin:messages:remove",
	type: "action",
	jsonSerializable: true,
	setup: (ctx) => ({ handler: async (id) => {
		await getMessagesHost(ctx)?.remove(id);
	} })
});
//#endregion
//#region src/node/rpc/index.ts
/**
* The message-feed RPC functions registered by the plugin: thin, typed
* wrappers over the hub's `ctx.messages` host. Namespaced
* `devframes:plugin:messages:*` per the plugin convention.
*/
const serverFunctions = [
	messagesList,
	messagesAdd,
	defineRpcFunction({
		name: "devframes:plugin:messages:update",
		type: "action",
		jsonSerializable: true,
		setup: (ctx) => ({ handler: async (id, patch) => {
			const host = getMessagesHost(ctx);
			if (!host) return null;
			return await host.update(id, patch) ?? null;
		} })
	}),
	messagesRemove,
	messagesClear
];
//#endregion
export { getMessagesHost as n, serverFunctions as t };
