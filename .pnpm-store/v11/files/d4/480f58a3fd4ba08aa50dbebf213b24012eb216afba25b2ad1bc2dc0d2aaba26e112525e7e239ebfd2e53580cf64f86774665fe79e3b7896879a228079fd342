import { DEFAULT_CODE_SERVER_PORT, PLUGIN_ID, STATE_KEY, TERMINAL_SESSION_ICON, TERMINAL_SESSION_TITLE, getCookieSessionName } from "./constants.mjs";
import { i as diagnostics, n as getCodeServerSupervisor, r as setCodeServerSupervisor, t as serverFunctions } from "../rpc-Df9IRgPd.mjs";
import { execSync, spawn } from "node:child_process";
import { createHash, randomBytes } from "node:crypto";
import { request } from "node:http";
import { hostname } from "node:os";
import process from "node:process";
import { getPort } from "devframe/utils/get-port";
//#region src/node/backends.ts
const sha256 = (value) => createHash("sha256").update(value).digest("hex");
/** Coder's open-source `code-server`, with password auth + session cookie handoff. */
const codeServerProfile = {
	kind: "code-server",
	backend: "code-server",
	defaultBin: "code-server",
	buildArgs: (c) => [
		"--auth",
		"password",
		"--bind-addr",
		`${c.host}:${c.port}`,
		"--disable-telemetry",
		"--disable-update-check",
		...c.extraArgs,
		c.folder
	],
	buildEnv: (c, base) => {
		const env = { ...base };
		delete env.PASSWORD;
		env.HASHED_PASSWORD = sha256(c.secret);
		return env;
	},
	healthPath: "/healthz",
	matchPort: (line) => {
		const m = line.match(/HTTP server listening on https?:\/\/(?:[^:]+|\[[^\]]+\]):(\d+)/);
		return m ? Number.parseInt(m[1], 10) : void 0;
	},
	connect: (c) => ({
		path: "/",
		cookie: {
			name: c.cookieName,
			value: sha256(c.secret)
		}
	}),
	connectReused: () => ({ path: "/" })
};
/** Microsoft's `code serve-web`, with a connection token passed via `?tkn=`. */
const serveWebProfile = {
	kind: "serve-web",
	backend: "ms-code-serve-web",
	defaultBin: "code",
	buildArgs: (c) => [
		"serve-web",
		"--accept-server-license-terms",
		"--host",
		c.host,
		"--port",
		String(c.port),
		"--connection-token",
		c.secret,
		...c.extraArgs
	],
	buildEnv: (_c, base) => ({ ...base }),
	/** serve-web has no health endpoint; the root responds (200/redirect) once up. */
	healthPath: "/",
	matchPort: (line) => {
		const m = line.match(/available at https?:\/\/[^:]+:(\d+)/i);
		return m ? Number.parseInt(m[1], 10) : void 0;
	},
	connect: (c) => ({ path: `/?tkn=${encodeURIComponent(c.secret)}&folder=${encodeURIComponent(c.folder)}` }),
	connectReused: () => ({ path: "/" })
};
const TUNNEL_URL_RE = /(https:\/\/vscode\.dev\/tunnel\/\S+)/;
const DEVICE_LOGIN_RE = /log in(?:to)? (https?:\/\/\S+) and use code ([A-Z0-9-]+)/i;
/** Microsoft's `code tunnel`, embedding the hosted `vscode.dev` editor. */
const tunnelProfile = {
	kind: "tunnel",
	backend: "ms-code-serve-web",
	defaultBin: "code",
	buildArgs: (c) => [
		"tunnel",
		"--accept-server-license-terms",
		"--name",
		c.tunnelName,
		...c.extraArgs
	],
	buildEnv: (_c, base) => ({ ...base }),
	healthPath: null,
	matchReadyUrl: (line) => {
		const m = line.match(TUNNEL_URL_RE);
		return m ? m[1].replace(/[)\].,]+$/, "") : void 0;
	},
	matchLogin: (line) => {
		const m = line.match(DEVICE_LOGIN_RE);
		return m ? {
			url: m[1],
			code: m[2]
		} : void 0;
	},
	connect: (c) => ({ url: c.readyUrl ?? `https://vscode.dev/tunnel/${c.tunnelName}${c.folder}` }),
	connectReused: () => ({})
};
const LOCAL_PROFILES = {
	"code-server": codeServerProfile,
	"ms-code-serve-web": serveWebProfile
};
/** The ordered auto-detection candidates for `mode: 'local'` with no explicit backend. */
const AUTO_DETECT_ORDER = ["code-server", "ms-code-serve-web"];
/** Resolve the launch profile for a mode + backend. */
function resolveProfile(mode, backend) {
	return mode === "tunnel" ? tunnelProfile : LOCAL_PROFILES[backend];
}
//#endregion
//#region src/node/detect.ts
/**
* Probe the host for a usable code-server binary by running
* `<bin> --version`. Resolves to `installed: false` when the binary is
* missing (ENOENT), errors, or exits non-zero (never throws), so the
* launcher can fall back to install instructions.
*
* `code-server --version` prints e.g. `4.96.4 abc123 with Code 1.96.4`; the
* first semver-looking token is taken as the version. Matching a semver
* pattern (rather than the leading whitespace token) keeps cold-start i18n
* noise (e.g. an `i18next: …` initialization line printed before the version)
* from leaking into the reported version.
*/
function detectCodeServer(bin = "code-server", timeoutMs = 5e3) {
	return new Promise((resolve) => {
		let settled = false;
		let stdout = "";
		const finish = (result) => {
			if (settled) return;
			settled = true;
			resolve(result);
		};
		let child;
		try {
			child = spawn(bin, ["--version"], {
				windowsHide: true,
				stdio: [
					"ignore",
					"pipe",
					"ignore"
				],
				shell: process.platform === "win32" && bin.endsWith(".cmd")
			});
		} catch {
			finish({
				installed: false,
				bin
			});
			return;
		}
		const timer = setTimeout(() => {
			child.kill();
			finish({
				installed: false,
				bin
			});
		}, timeoutMs);
		timer.unref?.();
		child.stdout?.on("data", (chunk) => {
			stdout += chunk.toString("utf8");
		});
		child.on("error", () => {
			clearTimeout(timer);
			finish({
				installed: false,
				bin
			});
		});
		child.on("exit", (code) => {
			clearTimeout(timer);
			if (code === 0) {
				const version = stdout.match(/\d+\.\d+\.\d\S*/)?.[0] ?? (stdout.trim().split(/\s+/)[0] || void 0);
				finish({
					installed: true,
					version,
					bin
				});
			} else finish({
				installed: false,
				bin
			});
		});
	});
}
//#endregion
//#region src/node/supervisor.ts
/** Recent output lines retained for surfacing startup failures. */
const LOG_BUFFER_LINES = 200;
/** Interval between readiness probes. */
const PROBE_INTERVAL = 250;
/**
* Owns the lifecycle of a single editor child process. Resolves a launch
* {@link CodeServerProfile} (Coder `code-server`, Microsoft `code serve-web`,
* or `code tunnel`), detects the binary, launches it with freshly generated
* auth material, waits for readiness, and mirrors a secret-free status into
* shared state. The connect descriptor (session cookie / connection token /
* tunnel URL) is handed back only through `start()` / `status()` so the
* already-authorized client can open the editor without a login page.
*
* Depends only on the core devframe context (shared state), not on the hub.
*/
var CodeServerSupervisor = class {
	ctx;
	mode;
	explicitBackend;
	explicitBin;
	workspace;
	host;
	forcedPort;
	extraArgs;
	extraEnv;
	cookieSuffix;
	cookieName;
	startTimeout;
	reuseExistingServer;
	tunnelName;
	/** Resolved after the first detection. */
	backend;
	bin;
	profile;
	state;
	detection;
	server = { status: "stopped" };
	proc;
	/** Context of the live launch, used to compute the client connect descriptor. */
	launchCtx;
	/** Whether the running server was adopted (reused) rather than launched. */
	adopted = false;
	/** Captured `vscode.dev` URL for a running tunnel. */
	readyUrl;
	logBuffer = [];
	exitHandler;
	/** Stable id of the hub terminal session, reused across start/stop. */
	sessionId;
	/** The live hub terminal session when launched through `ctx.terminals`. */
	session;
	constructor(ctx, options = {}) {
		this.ctx = ctx;
		this.mode = options.mode ?? "local";
		this.explicitBackend = options.backend;
		this.explicitBin = options.bin;
		this.workspace = options.cwd ?? ctx.cwd;
		this.host = options.host ?? "127.0.0.1";
		this.forcedPort = options.serverPort;
		this.extraArgs = options.args ?? [];
		this.extraEnv = options.env ?? {};
		this.cookieSuffix = options.cookieSuffix;
		this.cookieName = getCookieSessionName(options.cookieSuffix);
		this.startTimeout = options.startTimeout ?? 3e4;
		this.reuseExistingServer = options.reuseExistingServer ?? false;
		this.tunnelName = options.tunnel?.name || hostname().split(".").join("") || "devframe";
		this.sessionId = options.cookieSuffix ? `${PLUGIN_ID}:${options.cookieSuffix}` : PLUGIN_ID;
		this.backend = this.explicitBackend ?? "code-server";
		this.profile = resolveProfile(this.mode, this.backend);
		this.bin = this.explicitBin ?? this.profile.defaultBin;
		this.detection = {
			checked: false,
			installed: false,
			bin: this.bin,
			backend: this.backend,
			mode: this.mode
		};
	}
	/** Resolve shared state, register process-exit cleanup, run first detection. */
	async init() {
		if (this.state) return;
		this.state = await this.ctx.rpc.sharedState.get(STATE_KEY, { initialValue: {
			detection: this.detection,
			server: this.server
		} });
		await this.detect();
	}
	/**
	* Probe for a usable editor binary and publish the result. Resolves the
	* backend + binary when the caller left them implicit: tunnel mode always
	* uses `code`; an explicit backend or `bin` is honored as-is; otherwise the
	* plugin tries each {@link AUTO_DETECT_ORDER} candidate and keeps the first
	* that is installed.
	*/
	async detect() {
		let installed = false;
		let version;
		if (this.mode === "tunnel" || this.explicitBackend || this.explicitBin) {
			const result = await detectCodeServer(this.bin);
			installed = result.installed;
			version = result.version;
		} else {
			let resolved;
			for (const backend of AUTO_DETECT_ORDER) {
				const bin = resolveProfile("local", backend).defaultBin;
				const result = await detectCodeServer(bin);
				if (result.installed) {
					resolved = {
						backend,
						bin,
						version: result.version
					};
					break;
				}
			}
			if (resolved) {
				this.setResolved(resolved.backend, resolved.bin);
				installed = true;
				version = resolved.version;
			}
		}
		this.detection = {
			checked: true,
			installed,
			version,
			bin: this.bin,
			backend: this.backend,
			mode: this.mode
		};
		this.publish();
		return this.detection;
	}
	/** Current status (+ connect info when running) for the launcher UI. */
	status() {
		return {
			detection: { ...this.detection },
			server: this.serverInfo(),
			connect: this.connectInfo()
		};
	}
	/**
	* The server info projected to clients and shared state, tagged with the
	* live hub terminal session id when one exists so the launcher can offer a
	* "view in terminal" jump. Standalone runtimes hold no session and leave it
	* undefined.
	*/
	serverInfo() {
		return this.session ? {
			...this.server,
			terminalSessionId: this.sessionId
		} : { ...this.server };
	}
	/**
	* Launch the editor (if not already up) and resolve once it is reachable.
	* Idempotent while starting/running; returns the live status instead of
	* spawning a second process. In tunnel mode it resolves as soon as either
	* the `vscode.dev` URL or a device-login prompt is seen, so the action never
	* blocks on interactive authentication.
	*/
	async start(req = {}) {
		if (this.server.status === "running" || this.server.status === "starting") return this.status();
		if (!this.detection.checked) await this.detect();
		if (!this.detection.installed) throw diagnostics.DP_CODE_SERVER_0001({ bin: this.bin });
		const folder = req.folder ?? this.workspace;
		const isLocal = this.profile.kind !== "tunnel";
		if (isLocal && this.reuseExistingServer) {
			const adopted = await this.tryAdopt(folder);
			if (adopted) return adopted;
		}
		const secret = randomBytes(32).toString("hex");
		const initialPort = await this.resolveInitialPort(isLocal);
		const ctx = this.baseCtx(folder, initialPort, secret);
		this.launchCtx = ctx;
		this.adopted = false;
		this.readyUrl = void 0;
		const args = this.profile.buildArgs(ctx);
		const env = this.buildLaunchEnv(ctx);
		this.logBuffer = [];
		const signals = {
			isLocal,
			actualPort: initialPort
		};
		const portPromise = isLocal && initialPort === 0 ? new Promise((resolve, reject) => {
			signals.resolvePort = resolve;
			signals.rejectPort = reject;
		}) : Promise.resolve(initialPort);
		const readyUrlPromise = new Promise((resolve) => {
			signals.resolveReadyUrl = resolve;
		});
		const loginPromise = new Promise((resolve) => {
			signals.resolveLogin = resolve;
		});
		const child = await this.launchProcess(args, env, folder);
		this.registerCleanup();
		this.proc = child;
		this.server = {
			status: "starting",
			port: isLocal ? signals.actualPort || void 0 : void 0,
			pid: child.pid,
			startedAt: Date.now()
		};
		this.publish();
		child.stdout?.on("data", (chunk) => this.consumeOutput(chunk, child, signals));
		child.stderr?.on("data", (chunk) => this.consumeOutput(chunk, child, signals));
		child.on("error", (error) => this.handleChildError(child, error, signals));
		child.on("exit", (code) => this.handleChildExit(child, code, signals));
		try {
			if (!isLocal) return await this.awaitTunnel(child, readyUrlPromise, loginPromise);
			return await this.finalizeLocalStart(child, portPromise);
		} catch (error) {
			const reason = error instanceof Error ? error.message : String(error);
			return this.handleStartFailure(child, reason, isLocal, signals.actualPort);
		}
	}
	/** Adopt an already-running local server on the target port, if one answers. */
	async tryAdopt(folder) {
		const target = this.forcedPort && this.forcedPort !== 0 ? this.forcedPort : DEFAULT_CODE_SERVER_PORT;
		if (!this.profile.healthPath || !await probeHealth(target, this.profile.healthPath)) return void 0;
		this.adopted = true;
		this.launchCtx = this.baseCtx(folder, target, "");
		this.server = {
			status: "running",
			port: target,
			startedAt: Date.now()
		};
		this.publish();
		return this.status();
	}
	/**
	* Resolve the port to launch on: 0 for tunnels/dynamic launches, otherwise a
	* free port starting from the forced port (if any) or the default -
	* verified free before it's ever handed to the editor binary's `--port`.
	*/
	async resolveInitialPort(isLocal) {
		if (!isLocal || this.forcedPort === 0) return 0;
		return getPort({
			host: "127.0.0.1",
			port: this.forcedPort ?? 8080
		});
	}
	/** The child's environment: inherited env, caller overrides, then profile shaping. */
	buildLaunchEnv(ctx) {
		const baseEnv = {};
		for (const [k, v] of Object.entries(process.env)) if (v !== void 0) baseEnv[k] = v;
		Object.assign(baseEnv, this.extraEnv);
		return this.profile.buildEnv(ctx, baseEnv);
	}
	/** Buffer output and latch the port / login / ready-URL signals from each line. */
	consumeOutput(chunk, child, s) {
		const text = chunk.toString("utf8");
		this.appendLog(text);
		for (const line of text.split("\n")) {
			this.latchPort(line, s);
			this.latchLogin(line, child, s);
			this.latchReadyUrl(line, child, s);
		}
	}
	latchPort(line, s) {
		if (!s.isLocal || s.actualPort !== 0 || !this.profile.matchPort) return;
		const port = this.profile.matchPort(line);
		if (!port) return;
		s.actualPort = port;
		this.server.port = port;
		this.publish();
		s.resolvePort?.(port);
		s.resolvePort = void 0;
	}
	latchLogin(line, child, s) {
		const login = this.profile.matchLogin?.(line);
		if (!login || this.proc !== child) return;
		this.server = {
			...this.server,
			login
		};
		this.publish();
		s.resolveLogin?.(login);
	}
	latchReadyUrl(line, child, s) {
		const url = this.profile.matchReadyUrl?.(line);
		if (!url || this.proc !== child) return;
		this.readyUrl = url;
		s.resolveReadyUrl?.(url);
	}
	handleChildError(child, error, s) {
		if (this.proc !== child) return;
		this.server = {
			status: "error",
			error: error.message
		};
		this.reset();
		s.rejectPort?.(error);
		this.publish();
	}
	handleChildExit(child, code, s) {
		if (this.proc !== child) return;
		const exitCode = code ?? 0;
		const crashed = this.server.status !== "stopped" && exitCode !== 0;
		this.reset();
		this.server = crashed ? {
			status: "error",
			error: this.lastLog() || `${this.bin} exited with code ${exitCode}`
		} : { status: "stopped" };
		if (crashed) diagnostics.DP_CODE_SERVER_0005({ code: exitCode }, { method: "warn" });
		this.reflectHub(crashed ? "error" : "stopped");
		this.session = void 0;
		s.rejectPort?.(/* @__PURE__ */ new Error(`${this.bin} exited before binding (code ${exitCode})`));
		this.publish();
	}
	/** Wait for the dynamic port then readiness, and flip to `running`. */
	async finalizeLocalStart(child, portPromise) {
		const port = await Promise.race([portPromise, new Promise((_, reject) => setTimeout(() => reject(/* @__PURE__ */ new Error("timeout waiting for dynamic port allocation")), this.startTimeout))]);
		if (!await this.waitForReady(port, this.profile.healthPath)) throw new Error(this.lastLog() || "startup timed out");
		if (this.proc !== child) return this.status();
		this.server = {
			status: "running",
			port,
			pid: child.pid,
			startedAt: this.server.startedAt
		};
		this.publish();
		return this.status();
	}
	handleStartFailure(child, reason, isLocal, actualPort) {
		if (this.proc === child) {
			this.terminate(child);
			this.reflectHub("error");
			this.session = void 0;
			this.server = {
				status: "error",
				error: reason
			};
			this.reset();
			this.publish();
		}
		if (!isLocal) throw diagnostics.DP_CODE_SERVER_0006({ timeout: this.startTimeout });
		throw diagnostics.DP_CODE_SERVER_0002({
			port: actualPort,
			timeout: this.startTimeout
		});
	}
	/** Stop the editor process and reset to `stopped`. */
	stop() {
		const child = this.proc;
		const wasAdopted = this.adopted;
		this.reset();
		this.server = { status: "stopped" };
		if (child && !wasAdopted) this.terminate(child);
		this.reflectHub("stopped");
		this.session = void 0;
		this.publish();
		return this.status();
	}
	/** Kill the process on host shutdown / test teardown. */
	dispose() {
		if (this.proc && !this.adopted) this.terminate(this.proc);
		this.reset();
		this.session = void 0;
		if (this.exitHandler) {
			process.off("exit", this.exitHandler);
			this.exitHandler = void 0;
		}
	}
	/** Resolved backend for tests / callers. */
	get resolvedBackend() {
		return this.backend;
	}
	setResolved(backend, bin) {
		this.backend = backend;
		this.bin = bin;
		this.profile = resolveProfile(this.mode, backend);
	}
	baseCtx(folder, port, secret) {
		return {
			host: this.host,
			port,
			folder,
			secret,
			cookieName: this.cookieName,
			extraArgs: this.extraArgs,
			tunnelName: this.tunnelName
		};
	}
	/** Clear per-launch process state (keeps `server`/`detection`). */
	reset() {
		this.proc = void 0;
		this.launchCtx = void 0;
		this.adopted = false;
		this.readyUrl = void 0;
	}
	/**
	* Resolve start() for a tunnel: succeed as soon as the `vscode.dev` URL
	* appears (→ running) or a device-login prompt is seen (→ starting, so the
	* user can authenticate while the log stream continues to a running URL).
	*/
	async awaitTunnel(child, readyUrlPromise, loginPromise) {
		const timeout = new Promise((_, reject) => setTimeout(() => reject(/* @__PURE__ */ new Error("timed out waiting for the tunnel")), this.startTimeout));
		const outcome = await Promise.race([
			readyUrlPromise.then(() => "ready"),
			loginPromise.then(() => "login"),
			timeout
		]);
		if (this.proc !== child) return this.status();
		if (outcome === "ready") {
			this.server = {
				status: "running",
				pid: child.pid,
				startedAt: this.server.startedAt
			};
			this.publish();
			return this.status();
		}
		readyUrlPromise.then(() => {
			if (this.proc === child && this.server.status !== "running") {
				this.server = {
					status: "running",
					pid: child.pid,
					startedAt: this.server.startedAt,
					login: void 0
				};
				this.publish();
			}
		});
		return this.status();
	}
	connectInfo() {
		if (this.server.status !== "running" || !this.launchCtx) return void 0;
		if (this.adopted) return this.profile.connectReused({ port: this.launchCtx.port });
		return this.profile.connect({
			...this.launchCtx,
			port: this.server.port ?? this.launchCtx.port,
			readyUrl: this.readyUrl
		});
	}
	terminate(child) {
		if (this.session) {
			this.session.terminate().catch(() => {});
			return;
		}
		try {
			if (process.platform === "win32" && this.bin.endsWith(".cmd") && child.pid) execSync(`taskkill /pid ${child.pid} /t /f`, { stdio: "ignore" });
			else child.kill("SIGTERM");
		} catch {}
	}
	/**
	* Resolve the hub's terminals subsystem when this devframe is mounted in a
	* hub. `ctx.terminals` only exists on a `DevframeHubContext`, so it is
	* duck-typed: standalone runtimes (CLI / Vite / build) have no such property
	* and fall back to a direct child process.
	*/
	resolveHubTerminals() {
		const terminals = this.ctx.terminals;
		return terminals && typeof terminals.startChildProcess === "function" ? terminals : void 0;
	}
	/** Update the mirrored hub terminal session's status, when one exists. */
	reflectHub(status) {
		const hub = this.resolveHubTerminals();
		if (hub?.sessions.has(this.sessionId)) hub.update({
			id: this.sessionId,
			status
		});
	}
	/**
	* Launch the editor binary. In a hub, spawn it through `ctx.terminals` so it
	* shows up as a read-only terminal session (proper icon + name) whose output
	* the hub streams to its terminals panel; standalone, spawn it directly.
	* Either way, return the underlying {@link ChildProcess} so the shared
	* readiness / port / log wiring in `start()` is identical.
	*/
	async launchProcess(args, env, folder) {
		const hub = this.resolveHubTerminals();
		return hub ? this.launchViaHub(hub, args, env, folder) : this.launchDirect(args, env, folder);
	}
	/** Spawn through the hub's terminals subsystem so it shows as a terminal session. */
	async launchViaHub(hub, args, env, folder) {
		const stale = hub.sessions.get(this.sessionId);
		if (stale) hub.remove?.(stale);
		try {
			const session = await hub.startChildProcess({
				command: this.bin,
				args,
				cwd: folder,
				/**
				* The hub merges `process.env`, so neutralize PASSWORD explicitly
				* (code-server authenticates via HASHED_PASSWORD).
				*/
				env: {
					...env,
					PASSWORD: ""
				}
			}, {
				id: this.sessionId,
				title: TERMINAL_SESSION_TITLE,
				description: folder,
				icon: TERMINAL_SESSION_ICON,
				/**
				* Restart re-runs the supervisor's start flow (fresh port + secret),
				* not this raw process, so hide the panel's generic restart.
				*/
				restartable: false
			});
			const child = session.getChildProcess();
			if (!child) throw new Error("editor process handle was unavailable");
			this.session = session;
			return child;
		} catch (error) {
			const reason = error instanceof Error ? error.message : String(error);
			const orphan = hub.sessions.get(this.sessionId);
			if (orphan) hub.remove?.(orphan);
			this.session = void 0;
			this.server = {
				status: "error",
				error: reason
			};
			this.publish();
			throw diagnostics.DP_CODE_SERVER_0003({
				bin: this.bin,
				reason
			});
		}
	}
	/** Spawn the editor binary directly (standalone runtimes with no hub). */
	launchDirect(args, env, folder) {
		try {
			return spawn(this.bin, args, {
				cwd: folder,
				env,
				windowsHide: true,
				stdio: [
					"ignore",
					"pipe",
					"pipe"
				],
				shell: process.platform === "win32" && this.bin.endsWith(".cmd")
			});
		} catch (error) {
			const reason = error instanceof Error ? error.message : String(error);
			this.server = {
				status: "error",
				error: reason
			};
			this.publish();
			throw diagnostics.DP_CODE_SERVER_0003({
				bin: this.bin,
				reason
			});
		}
	}
	appendLog(text) {
		for (const line of text.split("\n")) {
			if (line.length === 0) continue;
			this.logBuffer.push(line);
		}
		if (this.logBuffer.length > LOG_BUFFER_LINES) this.logBuffer.splice(0, this.logBuffer.length - LOG_BUFFER_LINES);
	}
	lastLog() {
		return this.logBuffer.length ? this.logBuffer[this.logBuffer.length - 1] : void 0;
	}
	publish() {
		this.state?.mutate((draft) => {
			draft.detection = { ...this.detection };
			draft.server = this.serverInfo();
		});
	}
	/**
	* Poll the server's readiness path until it responds or the timeout elapses.
	* Returns false if the process exits first.
	*/
	async waitForReady(port, path) {
		const deadline = Date.now() + this.startTimeout;
		while (Date.now() < deadline) {
			if (!this.proc) return false;
			if (await probeHealth(port, path)) return true;
			await delay(PROBE_INTERVAL);
		}
		return false;
	}
	registerCleanup() {
		if (this.exitHandler) return;
		this.exitHandler = () => this.dispose();
		process.once("exit", this.exitHandler);
	}
};
function delay(ms) {
	return new Promise((resolve) => {
		setTimeout(resolve, ms).unref?.();
	});
}
function probeHealth(port, path) {
	return new Promise((resolve) => {
		const req = request({
			host: "127.0.0.1",
			port,
			path,
			method: "GET",
			timeout: 1500
		}, (res) => {
			res.resume();
			resolve((res.statusCode ?? 0) >= 200 && (res.statusCode ?? 0) < 500);
		});
		req.on("error", () => resolve(false));
		req.on("timeout", () => {
			req.destroy();
			resolve(false);
		});
		req.end();
	});
}
//#endregion
//#region src/node/setup.ts
/**
* Wire the code-server subsystem onto a devframe node context: create the
* {@link CodeServerSupervisor}, run the initial binary detection, publish
* status into shared state, and register the control RPC functions. Returns
* the supervisor so callers can launch/stop or dispose it on shutdown.
*
* Works in any devframe runtime (CLI, Vite, embedded, build), since it only relies
* on the core `ctx.rpc` shared-state surface, not on the hub.
*/
async function setupCodeServer(ctx, options = {}) {
	const supervisor = new CodeServerSupervisor(ctx, options);
	setCodeServerSupervisor(ctx, supervisor);
	await supervisor.init();
	for (const fn of serverFunctions) ctx.rpc.register(fn);
	if (options.startOnBoot) supervisor.start().catch(() => {});
	return supervisor;
}
//#endregion
export { CodeServerSupervisor, detectCodeServer, diagnostics, getCodeServerSupervisor, resolveProfile, setCodeServerSupervisor, setupCodeServer };
