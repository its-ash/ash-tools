import { defineRpcFunction } from "devframe";
import { defineDiagnostics } from "devframe/utils/nostics";
import { s } from "devframe/utils/simple-schema";
//#region src/node/diagnostics.ts
/**
* Structured diagnostics for the terminals plugin. Uses the plugin's own
* `DP_TERMINALS_` prefix per the built-in plugin convention, keeping it
* collision-free with devframe core (`DF`) and the hub (`DF8xxx`).
*/
const diagnostics = defineDiagnostics({
	docsBase: "https://devfra.me/errors",
	codes: {
		DP_TERMINALS_0001: {
			why: (p) => `Terminal session "${p.id}" does not exist`,
			fix: "Spawn a session first, or refresh the session list."
		},
		DP_TERMINALS_0002: {
			why: (p) => `Spawning the arbitrary command "${p.command}" is not allowed`,
			fix: "Add it to `presets`, or pass `allowArbitraryCommands: true` to createTerminalsDevframe()."
		},
		DP_TERMINALS_0003: {
			why: (p) => `Cannot write to read-only terminal session "${p.id}"`,
			fix: "Spawn the session with `mode: \"interactive\"` to accept input."
		},
		DP_TERMINALS_0004: { why: (p) => `Failed to spawn "${p.command}": ${p.reason}` },
		DP_TERMINALS_0005: {
			why: "Native PTY bindings (zigpty) are unavailable; interactive sessions fall back to pipe-based terminal emulation. Full-screen TUIs may not render correctly.",
			fix: "Check that this platform is covered by zigpty's prebuilds (Linux/macOS/Windows, x64/arm64, glibc/musl) and that the dependency installed intact."
		},
		DP_TERMINALS_0006: { why: (p) => `Unknown terminal preset "${p.id}"` },
		DP_TERMINALS_0007: {
			why: "Terminals manager is not initialised on this context",
			fix: "Call setupTerminals(ctx) (or use createTerminalsDevframe) before invoking terminal RPCs."
		}
	}
});
//#endregion
//#region src/node/context.ts
const managers = /* @__PURE__ */ new WeakMap();
function setTerminalManager(ctx, manager) {
	managers.set(ctx, manager);
}
function getTerminalManager(ctx) {
	const manager = managers.get(ctx);
	if (!manager) throw diagnostics.DP_TERMINALS_0007({});
	return manager;
}
//#endregion
//#region src/node/rpc/functions/clear-exited.ts
const clearExited = defineRpcFunction({
	name: "devframes:plugin:terminals:clear-exited",
	type: "action",
	jsonSerializable: true,
	args: [],
	returns: s.void(),
	agent: {
		description: "Discard every stopped (exited or errored) terminal session at once. Running sessions are left untouched.",
		safety: "destructive"
	},
	setup: (ctx) => ({ handler: () => {
		getTerminalManager(ctx).clearExited();
	} })
});
//#endregion
//#region src/node/rpc/schemas.ts
const terminalModeSchema = s.picklist(["interactive", "readonly"]);
const spawnRequestSchema = s.object({
	presetId: s.optional(s.string()),
	command: s.optional(s.string()),
	args: s.optional(s.array(s.string())),
	cwd: s.optional(s.string()),
	mode: s.optional(terminalModeSchema),
	title: s.optional(s.string()),
	cols: s.optional(s.number()),
	rows: s.optional(s.number()),
	env: s.optional(s.record(s.string(), s.string()))
});
const sessionInfoSchema = s.object({
	id: s.string(),
	title: s.string(),
	processName: s.optional(s.string()),
	customTitle: s.optional(s.string()),
	mode: terminalModeSchema,
	status: s.picklist([
		"running",
		"exited",
		"error"
	]),
	backend: s.picklist(["pty", "pipe"]),
	command: s.string(),
	args: s.array(s.string()),
	cwd: s.string(),
	cols: s.number(),
	rows: s.number(),
	pid: s.optional(s.number()),
	exitCode: s.optional(s.number()),
	icon: s.optional(s.string()),
	channel: s.optional(s.string()),
	presetId: s.optional(s.string()),
	createdAt: s.number()
});
const presetSchema = s.object({
	id: s.string(),
	title: s.string(),
	command: s.string(),
	args: s.array(s.string()),
	mode: terminalModeSchema,
	icon: s.optional(s.string())
});
//#endregion
//#region src/node/rpc/functions/list.ts
const list = defineRpcFunction({
	name: "devframes:plugin:terminals:list",
	type: "query",
	jsonSerializable: true,
	args: [],
	returns: s.array(sessionInfoSchema),
	agent: {
		description: "List the current terminal sessions with their status, mode, and command.",
		safety: "read"
	},
	setup: (ctx) => ({ handler: () => getTerminalManager(ctx).list() })
});
//#endregion
//#region src/node/rpc/functions/presets.ts
const presets = defineRpcFunction({
	name: "devframes:plugin:terminals:presets",
	type: "query",
	jsonSerializable: true,
	args: [],
	returns: s.array(presetSchema),
	setup: (ctx) => ({ handler: () => getTerminalManager(ctx).getPresets().map((p) => ({
		id: p.id,
		title: p.title,
		command: p.command,
		args: p.args ?? [],
		mode: p.mode ?? "readonly",
		icon: p.icon
	})) })
});
//#endregion
//#region src/node/rpc/functions/remove.ts
const remove = defineRpcFunction({
	name: "devframes:plugin:terminals:remove",
	type: "action",
	jsonSerializable: true,
	args: [s.object({ id: s.string() })],
	returns: s.void(),
	agent: {
		description: "Kill a terminal session and discard it (process, stream, and scrollback).",
		safety: "destructive"
	},
	setup: (ctx) => ({ handler: ({ id }) => {
		getTerminalManager(ctx).remove(id);
	} })
});
//#endregion
//#region src/node/rpc/functions/rename.ts
const rename = defineRpcFunction({
	name: "devframes:plugin:terminals:rename",
	type: "action",
	jsonSerializable: true,
	args: [s.object({
		id: s.string(),
		title: s.string()
	})],
	returns: s.void(),
	setup: (ctx) => ({ handler: ({ id, title }) => {
		getTerminalManager(ctx).rename(id, title);
	} })
});
//#endregion
//#region src/node/rpc/functions/resize.ts
const resize = defineRpcFunction({
	name: "devframes:plugin:terminals:resize",
	type: "action",
	jsonSerializable: true,
	args: [s.object({
		id: s.string(),
		cols: s.number(),
		rows: s.number()
	})],
	returns: s.void(),
	setup: (ctx) => ({ handler: ({ id, cols, rows }) => {
		getTerminalManager(ctx).resize(id, cols, rows);
	} })
});
//#endregion
//#region src/node/rpc/functions/restart.ts
const restart = defineRpcFunction({
	name: "devframes:plugin:terminals:restart",
	type: "action",
	jsonSerializable: true,
	args: [s.object({ id: s.string() })],
	returns: sessionInfoSchema,
	setup: (ctx) => ({ handler: ({ id }) => getTerminalManager(ctx).restart(id) })
});
//#endregion
//#region src/node/rpc/functions/spawn.ts
const spawn = defineRpcFunction({
	name: "devframes:plugin:terminals:spawn",
	type: "action",
	jsonSerializable: true,
	args: [spawnRequestSchema],
	returns: sessionInfoSchema,
	agent: {
		description: "Spawn a new terminal session. Pass a preset id, or a command + mode. Interactive sessions accept input; readonly sessions only stream output.",
		safety: "action"
	},
	setup: (ctx) => ({ handler: (req) => getTerminalManager(ctx).spawn(req ?? {}) })
});
//#endregion
//#region src/node/rpc/functions/terminate.ts
const terminate = defineRpcFunction({
	name: "devframes:plugin:terminals:terminate",
	type: "action",
	jsonSerializable: true,
	args: [s.object({ id: s.string() })],
	returns: s.void(),
	agent: {
		description: "Terminate a terminal session's running process. The session and its scrollback are kept; use restart to run it again.",
		safety: "destructive"
	},
	setup: (ctx) => ({ handler: ({ id }) => {
		getTerminalManager(ctx).terminate(id);
	} })
});
//#endregion
//#region src/node/rpc/index.ts
const serverFunctions = [
	list,
	presets,
	spawn,
	defineRpcFunction({
		name: "devframes:plugin:terminals:write",
		type: "action",
		jsonSerializable: true,
		args: [s.object({
			id: s.string(),
			data: s.string()
		})],
		returns: s.void(),
		setup: (ctx) => ({ handler: ({ id, data }) => {
			getTerminalManager(ctx).write(id, data);
		} })
	}),
	resize,
	terminate,
	restart,
	rename,
	remove,
	clearExited
];
//#endregion
export { diagnostics as i, getTerminalManager as n, setTerminalManager as r, serverFunctions as t };
