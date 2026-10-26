import { defineRpcFunction } from "devframe";
import { defineDiagnostics } from "devframe/utils/nostics";
//#region src/node/diagnostics.ts
/**
* Structured diagnostics for the code-server plugin. Uses the plugin's own
* `DP_CODE_SERVER_` prefix per the built-in plugin convention, keeping it
* collision-free with devframe core (`DF`) and the hub (`DF8xxx`).
*/
const diagnostics = defineDiagnostics({
	docsBase: "https://devfra.me/errors",
	codes: {
		DP_CODE_SERVER_0001: {
			why: (p) => `No editor binary found (could not run "${p.bin} --version")`,
			fix: "Install Coder code-server (`curl -fsSL https://code-server.dev/install.sh | sh`) or the Microsoft `code` CLI, or set the `bin` option to its path. See https://coder.com/docs/code-server/latest/install"
		},
		DP_CODE_SERVER_0002: {
			why: (p) => `the editor did not become ready on port ${p.port} within ${p.timeout}ms`,
			fix: "Check the editor logs for startup errors, raise `startTimeout`, or free the port."
		},
		DP_CODE_SERVER_0003: { why: (p) => `Failed to spawn the editor ("${p.bin}"): ${p.reason}` },
		DP_CODE_SERVER_0004: {
			why: "code-server supervisor is not initialised on this context",
			fix: "Call setupCodeServer(ctx) (or use createCodeServerDevframe) before invoking the code-server RPCs."
		},
		DP_CODE_SERVER_0005: {
			why: (p) => `the editor process exited unexpectedly with code ${p.code}`,
			fix: "Inspect the captured output in the launcher and re-launch."
		},
		DP_CODE_SERVER_0006: {
			why: (p) => `the tunnel did not open within ${p.timeout}ms`,
			fix: "Check the tunnel logs, ensure the `code` CLI is signed in, or raise `startTimeout`."
		}
	}
});
//#endregion
//#region src/node/context.ts
const supervisors = /* @__PURE__ */ new WeakMap();
function setCodeServerSupervisor(ctx, supervisor) {
	supervisors.set(ctx, supervisor);
}
function getCodeServerSupervisor(ctx) {
	const supervisor = supervisors.get(ctx);
	if (!supervisor) throw diagnostics.DP_CODE_SERVER_0004({});
	return supervisor;
}
//#endregion
//#region src/node/rpc/functions/detect.ts
const detect = defineRpcFunction({
	name: "devframes:plugin:code-server:detect",
	type: "query",
	jsonSerializable: true,
	agent: {
		description: "Check whether the code-server binary is installed locally, returning its version when found.",
		safety: "read"
	},
	setup: (ctx) => ({ handler: () => getCodeServerSupervisor(ctx).detect() })
});
//#endregion
//#region src/node/rpc/functions/start.ts
const start = defineRpcFunction({
	name: "devframes:plugin:code-server:start",
	type: "action",
	jsonSerializable: true,
	agent: {
		description: "Launch the code-server editor (if not already running) and wait until it is ready. Optionally open a specific workspace folder.",
		safety: "action"
	},
	setup: (ctx) => ({ handler: (req = {}) => getCodeServerSupervisor(ctx).start(req) })
});
//#endregion
//#region src/node/rpc/index.ts
const serverFunctions = [
	detect,
	defineRpcFunction({
		name: "devframes:plugin:code-server:status",
		type: "query",
		jsonSerializable: true,
		setup: (ctx) => ({ handler: () => getCodeServerSupervisor(ctx).status() })
	}),
	start,
	defineRpcFunction({
		name: "devframes:plugin:code-server:stop",
		type: "action",
		jsonSerializable: true,
		agent: {
			description: "Stop the running code-server editor process.",
			safety: "action"
		},
		setup: (ctx) => ({ handler: () => getCodeServerSupervisor(ctx).stop() })
	})
];
//#endregion
export { diagnostics as i, getCodeServerSupervisor as n, setCodeServerSupervisor as r, serverFunctions as t };
