import { Server } from "@modelcontextprotocol/server";
import { diagnostics, listLiveDevframeInstances, probeDevframeOrigin } from "devframe/internal";
import { toAgentToolName } from "devframe/utils/agent-tool-name";
import { Client, StreamableHTTPClientTransport } from "@modelcontextprotocol/client";
import { StdioServerTransport } from "@modelcontextprotocol/server/stdio";
import { Diagnostic } from "devframe/utils/nostics";
import { joinURL, withLeadingSlash, withTrailingSlash } from "devframe/utils/url";
//#region src/connect/index.ts
/**
* Resolve the per-record bearer from the {@link ConnectServerOptions.authToken}
* option. Exported for focused tests of the credential resolution.
*/
function resolveAuthToken(authToken, record) {
	return typeof authToken === "function" ? authToken(record) : authToken;
}
/**
* Build the request headers the connector sends to one instance's MCP route:
* the instance's own (loopback) `origin` so the route's origin gate accepts
* this native client, plus `Authorization: Bearer <token>` when a bearer is
* configured. The bearer appears **only** here, never in the connection URL,
* the registry records, or the indexed results. Exported for focused tests.
*/
function buildInstanceRequestHeaders(url, token) {
	const headers = { origin: new URL(url).origin };
	if (token) headers.authorization = `Bearer ${token}`;
	return headers;
}
const INDEX_TOOL = toAgentToolName("devframe:connect:list-instances");
const CALL_TOOL = toAgentToolName("devframe:connect:call-tool");
const MCP_DISABLED_HINT = "This instance runs without an MCP route. Restart it with the --mcp flag to expose its tools, then list instances again.";
const GATEWAY_TOOLS = [{
	name: INDEX_TOOL,
	title: "Discover running devframes",
	description: "Discover every running devframe dev server on this machine and list each one's MCP tools. Call this FIRST, before assuming which devtools are available; the result names the instance (id, project root, origin) and the port to pass to the call tool. Safe to call freely.",
	inputSchema: {
		type: "object",
		properties: {}
	},
	annotations: {
		readOnlyHint: true,
		destructiveHint: false
	}
}, {
	name: CALL_TOOL,
	title: "Call a devframe tool",
	description: "Invoke one MCP tool on one running devframe instance discovered via the list-instances tool. Pass the instance's port, the tool name, and the tool's arguments object.",
	inputSchema: {
		type: "object",
		properties: {
			port: {
				type: "number",
				description: "The instance's port, from the list-instances tool."
			},
			tool: {
				type: "string",
				description: "Tool name, from the instance's tool list."
			},
			args: {
				type: "object",
				description: "Arguments object for the tool. Omit for zero-argument tools."
			}
		},
		required: ["port", "tool"],
		additionalProperties: false
	}
}];
/**
* Start the devframe MCP connector on stdio: a thin discovery + proxy server
* in the shape Vercel's next-devtools-mcp (https://github.com/vercel/next-devtools-mcp)
* validated; credit is due there for the architecture this connector follows.
* It exposes two gateway tools:
* `devframe_connect_list-instances` (discover running devframe instances via
* the instance registry and list each one's MCP tools) and
* `devframe_connect_call-tool` (invoke one tool on one instance over its
* Streamable-HTTP endpoint), and holds no domain knowledge of its own.
*/
async function startConnectServer(options = {}) {
	const server = new Server({
		name: "devframe-connect",
		version: "0.0.0"
	}, { capabilities: { tools: {} } });
	server.setRequestHandler("tools/list", async () => ({ tools: GATEWAY_TOOLS }));
	server.setRequestHandler("tools/call", async (request) => {
		const { name, arguments: args } = request.params;
		try {
			if (name === INDEX_TOOL) return textResult(await index(options));
			if (name === CALL_TOOL) return textResult(await call(options, args ?? {}));
			return errorResult({
				message: `unknown tool "${name}"`,
				fix: `Call ${INDEX_TOOL} or ${CALL_TOOL}.`
			});
		} catch (error) {
			return errorResult(toErrorPayload(error));
		}
	});
	const transport = new StdioServerTransport();
	await server.connect(transport);
	return { stop: async () => {
		await server.close();
	} };
}
/** Discover instances: registry (prune-on-read) + explicit port probes. */
async function index(options) {
	const { live } = await listLiveDevframeInstances({
		instancesDir: options.instancesDir,
		timeoutMs: options.timeoutMs
	});
	const records = [...live];
	for (const port of options.ports ?? []) {
		if (records.some((r) => r.port === port)) continue;
		const probed = await probePort(port, options.base, options.timeoutMs);
		if (probed) records.push(probed);
	}
	const instances = await Promise.all(records.map(async (record) => {
		const { mcp, ...rest } = record;
		const entry = {
			...rest,
			mcp: null
		};
		if (!mcp) {
			entry.hint = MCP_DISABLED_HINT;
			return entry;
		}
		const url = `${record.origin}${mcp.path}`;
		try {
			entry.mcp = {
				url,
				tools: await listInstanceTools(url, resolveAuthToken(options.authToken, record))
			};
		} catch (error) {
			entry.mcp = {
				url,
				error: error instanceof Error ? error.message : String(error)
			};
		}
		return entry;
	}));
	return {
		instances,
		...instances.length === 0 ? { hint: "No running devframe instances found. Start a devframe dev server (with --mcp for tools), or pass --port <n> to devframe connect if the instance predates the registry (plus --base <path> when it is mounted below the root, e.g. --base /__devtools/)." } : {}
	};
}
/**
* Probe an explicit port for a devframe serving `__connection.json` under
* `base` (default `/`), reusing the registry's origin-candidate probe (a
* `localhost`-bound server may listen on either address family). The
* advertised MCP path is relative to that base, as in the registry records
* the instance shell writes. Exported for focused tests.
*/
async function probePort(port, base = "/", timeoutMs) {
	const basePath = withTrailingSlash(withLeadingSlash(base));
	const probed = await probeDevframeOrigin(`http://localhost:${port}`, basePath, timeoutMs);
	if (!probed) return null;
	const mcpPath = probed.meta.mcp ? joinURL(basePath, probed.meta.mcp.path) : null;
	return {
		pid: -1,
		port,
		origin: probed.origin,
		basePath,
		id: `port-${port}`,
		rootDir: "",
		mcp: mcpPath ? { path: mcpPath } : null,
		startedAt: 0
	};
}
async function listInstanceTools(url, token) {
	return withInstanceClient(url, token, async (client) => (await client.listTools()).tools);
}
async function call(options, args) {
	if (typeof args.port !== "number" || typeof args.tool !== "string") throw diagnostics.DF0049();
	const { live } = await listLiveDevframeInstances({
		instancesDir: options.instancesDir,
		timeoutMs: options.timeoutMs
	});
	const record = live.find((record) => record.port === args.port && record.mcp) ?? await probePort(args.port, options.base, options.timeoutMs);
	if (!record) throw diagnostics.DF0050({ port: args.port });
	if (!record.mcp) throw diagnostics.DF0051({ port: args.port });
	return withInstanceClient(`${record.origin}${record.mcp.path}`, resolveAuthToken(options.authToken, record), async (client) => {
		const result = await client.callTool({
			name: args.tool,
			arguments: args.args ?? {}
		});
		return {
			instance: {
				id: record.id,
				port: record.port
			},
			tool: args.tool,
			isError: result.isError ?? false,
			content: result.content,
			...result.structuredContent ? { structuredContent: result.structuredContent } : {}
		};
	});
}
async function withInstanceClient(url, token, fn) {
	const transport = new StreamableHTTPClientTransport(new URL(url), { requestInit: { headers: buildInstanceRequestHeaders(url, token) } });
	const client = new Client({
		name: "devframe-connect",
		version: "0.0.0"
	}, { versionNegotiation: { mode: "auto" } });
	await client.connect(transport);
	try {
		return await fn(client);
	} finally {
		await client.close().catch(() => {});
	}
}
function textResult(value) {
	return { content: [{
		type: "text",
		text: JSON.stringify(value, null, 2)
	}] };
}
/**
* Project a thrown value into the connector's structured error payload. A
* nostics `Diagnostic` carries its code, `fix`, and docs URL across so the
* calling agent gets the actionable next step.
*/
function toErrorPayload(error) {
	if (error instanceof Diagnostic) return {
		code: error.code,
		message: error.message,
		...error.fix ? { fix: error.fix } : {},
		...error.docs ? { docs: error.docs } : {}
	};
	return {
		message: error instanceof Error ? error.message : String(error),
		...error && typeof error === "object" && "fix" in error && typeof error.fix === "string" ? { fix: error.fix } : {}
	};
}
function errorResult(error) {
	return {
		isError: true,
		content: [{
			type: "text",
			text: JSON.stringify({ error }, null, 2)
		}]
	};
}
//#endregion
export { buildInstanceRequestHeaders, probePort, resolveAuthToken, startConnectServer };
