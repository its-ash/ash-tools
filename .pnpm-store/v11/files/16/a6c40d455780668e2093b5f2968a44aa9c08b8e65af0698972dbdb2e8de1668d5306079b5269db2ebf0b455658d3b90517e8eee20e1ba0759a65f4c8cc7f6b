import { HUB_TERMINAL_STREAM_CHANNEL, PRESETS_STATE_KEY, SESSIONS_STATE_KEY, TERMINAL_STREAM_CHANNEL } from "./constants.mjs";
import { i as diagnostics, n as getTerminalManager, r as setTerminalManager, t as serverFunctions } from "../rpc-qXpNicsS.mjs";
import process from "node:process";
import { nanoid } from "devframe/utils/nanoid";
import { OSCInspector } from "zigpty/osc";
import { spawn } from "node:child_process";
//#region src/node/backend.ts
/** TERM name handed to the PTY; also used to reject fallback process labels. */
const PTY_TERM_NAME = "xterm-256color";
let zigptyPromise;
/**
* Lazily load the PTY backend. The import is deferred so the native binding
* only loads once a terminal actually spawns, and stays a runtime import for
* bundled hosts (Next/Turbopack). Resolves to `undefined` when the module
* fails to load, letting interactive sessions degrade to a piped child
* process.
*/
async function loadZigpty() {
	zigptyPromise ??= import("zigpty").then((m) => m, () => void 0);
	return zigptyPromise;
}
/**
* Whether real pseudo-terminals are available in this runtime, i.e. zigpty's
* native bindings loaded. Without them interactive sessions still run through
* zigpty's pipe-based emulation, with degraded TUI fidelity.
*/
async function isPtyAvailable() {
	return (await loadZigpty())?.hasNative ?? false;
}
/**
* Spawn an interactive terminal via zigpty. Uses a real PTY when the native
* bindings are available, and zigpty's pipe-based emulation (line discipline,
* signal translation, best-effort resize) otherwise; the reported `backend`
* reflects which one the session got. Returns `undefined` when the module
* itself is unavailable or spawning throws.
*/
async function spawnPty(options) {
	const zigpty = await loadZigpty();
	if (!zigpty) return void 0;
	let proc;
	try {
		proc = zigpty.spawn(options.command, options.args, {
			name: PTY_TERM_NAME,
			cols: options.cols,
			rows: options.rows,
			cwd: options.cwd,
			env: options.env
		});
	} catch (error) {
		diagnostics.DP_TERMINALS_0004({
			command: options.command,
			reason: error instanceof Error ? error.message : String(error)
		}, { method: "warn" });
		return;
	}
	return {
		backend: zigpty.hasNative ? "pty" : "pipe",
		get pid() {
			return proc.pid;
		},
		write: (data) => {
			try {
				proc.write(data);
			} catch {}
		},
		resize: (cols, rows) => {
			try {
				proc.resize(Math.max(1, cols), Math.max(1, rows));
			} catch {}
		},
		kill: (signal) => {
			try {
				proc.kill(signal);
			} catch {}
		},
		onData: (cb) => {
			proc.onData((data) => {
				cb(typeof data === "string" ? data : data.toString("utf8"));
			});
		},
		onExit: (cb) => proc.onExit((e) => cb(e.exitCode ?? 0)),
		getProcessName: () => {
			try {
				const name = proc.process;
				return name && name !== PTY_TERM_NAME ? name : void 0;
			} catch {
				return;
			}
		}
	};
}
/**
* Spawn a piped child process. Used for readonly sessions and as the last
* interactive fallback when zigpty is unavailable entirely. stdout/stderr
* are merged into a single ordered text stream.
*/
function spawnPipe(options) {
	const dataCbs = [];
	const exitCbs = [];
	let exited = false;
	let pendingCr = false;
	const normalizeNewlines = (data) => {
		const out = data.replace(/\r?\n/g, (match, offset) => match === "\n" && !(offset === 0 && pendingCr) ? "\r\n" : match);
		pendingCr = out.endsWith("\r");
		return out;
	};
	const emitData = (data) => {
		const normalized = normalizeNewlines(data);
		for (const cb of dataCbs) cb(normalized);
	};
	const emitExit = (code) => {
		if (exited) return;
		exited = true;
		for (const cb of exitCbs) cb(code);
	};
	const child = spawn(options.command, options.args, {
		cwd: options.cwd,
		env: options.env,
		stdio: [
			options.input ? "pipe" : "ignore",
			"pipe",
			"pipe"
		],
		windowsHide: true
	});
	child.stdout?.on("data", (chunk) => emitData(chunk.toString("utf8")));
	child.stderr?.on("data", (chunk) => emitData(chunk.toString("utf8")));
	child.on("error", (error) => {
		emitData(`\r\n[failed to start: ${error.message}]\r\n`);
		emitExit(1);
	});
	child.on("exit", (code) => emitExit(code ?? 0));
	return {
		backend: "pipe",
		get pid() {
			return child.pid;
		},
		write: (data) => {
			if (options.input && child.stdin && !child.stdin.destroyed) child.stdin.write(data);
		},
		resize: () => {},
		kill: (signal) => {
			try {
				child.kill(signal ?? "SIGTERM");
			} catch {}
		},
		onData: (cb) => dataCbs.push(cb),
		onExit: (cb) => exitCbs.push(cb)
	};
}
/**
* Spawn the most capable backend for the requested interaction. Interactive
* sessions prefer a real PTY (for TUIs) and degrade through zigpty's pipe
* emulation; readonly sessions use a piped child process.
*/
async function spawnBackend(options, preferPty) {
	if (preferPty) {
		if (!await isPtyAvailable()) diagnostics.DP_TERMINALS_0005({}, { method: "warn" });
		const pty = await spawnPty(options);
		if (pty) return pty;
	}
	return spawnPipe(options);
}
//#endregion
//#region src/node/manager.ts
/** How often a PTY session's foreground process name is polled. */
const PROCESS_POLL_INTERVAL = 1e3;
/** Map the plugin's session status onto the hub's coarser status set. */
const HUB_STATUS = {
	running: "running",
	exited: "stopped",
	error: "error"
};
/**
* Preserve explicit masks; normalize other hub icons (`ph:code-duotone`) to
* UnoCSS classes (`i-ph-code-duotone`), choosing the light variant of theme pairs.
* Class icons render only if the terminal SPA's UnoCSS build includes them
* (see the safelist in `uno.config.ts`); masks load their image URL directly.
*/
function toIconClass(icon) {
	const raw = typeof icon === "string" ? icon : icon?.light;
	if (!raw) return void 0;
	return raw.startsWith("mask:") || raw.startsWith("i-") ? raw : `i-${raw.replace(":", "-")}`;
}
function defaultShell() {
	if (process.platform === "win32") return process.env.COMSPEC || "powershell.exe";
	return process.env.SHELL || "bash";
}
/**
* Owns terminal session lifecycle: spawns PTY / piped backends, streams
* their output over the `devframes:plugin:terminals:output` channel (one
* stream per session, stable for the session's whole life so restarts reuse
* the same id), and mirrors a serializable session list into shared state.
*/
var TerminalManager = class {
	ctx;
	options;
	shell;
	shellArgs;
	defaultCwd;
	defaultMode;
	allowArbitraryCommands;
	presets;
	channel;
	sessionsState;
	sessions = /* @__PURE__ */ new Map();
	ptyAvailable = false;
	/** Session ids this manager has registered into the hub (when hubbed). */
	hubOwned = /* @__PURE__ */ new Set();
	constructor(ctx, options = {}) {
		this.ctx = ctx;
		this.options = options;
		this.shell = options.shell ?? defaultShell();
		this.shellArgs = options.shellArgs ?? [];
		this.defaultCwd = options.cwd ?? ctx.cwd;
		this.defaultMode = options.defaultMode ?? "interactive";
		this.allowArbitraryCommands = options.allowArbitraryCommands ?? false;
		this.presets = options.presets ?? [];
		this.channel = ctx.rpc.streaming.create(TERMINAL_STREAM_CHANNEL, { replayWindow: options.scrollback ?? 5e3 });
	}
	/** Resolve shared state, probe the PTY backend, publish the preset catalog. */
	async init() {
		if (this.sessionsState) return;
		this.ptyAvailable = await isPtyAvailable();
		this.sessionsState = await this.ctx.rpc.sharedState.get(SESSIONS_STATE_KEY, { initialValue: { sessions: [] } });
		(await this.ctx.rpc.sharedState.get(PRESETS_STATE_KEY, { initialValue: { presets: [] } })).mutate((draft) => {
			draft.presets = this.presets.map((p) => ({
				id: p.id,
				title: p.title,
				command: p.command,
				args: p.args ?? [],
				mode: p.mode ?? "readonly",
				icon: p.icon
			}));
		});
		this.hubTerminals()?.events?.on("terminals:session:updated", (session) => {
			if (!this.sessions.has(session.id)) this.refreshSessionsState();
		});
	}
	/** The hub's terminals subsystem when mounted in a hub, else undefined. */
	hubTerminals() {
		return this.ctx.terminals;
	}
	list() {
		const own = Array.from(this.sessions.values()).map((s) => ({ ...s.info }));
		const hub = this.hubTerminals();
		if (!hub?.sessions) return own;
		const foreign = [];
		for (const session of hub.sessions.values()) {
			if (this.sessions.has(session.id)) continue;
			foreign.push({
				id: session.id,
				title: session.title,
				mode: session.interactive ? "interactive" : "readonly",
				status: session.status === "stopped" ? "exited" : session.status,
				backend: session.interactive ? "pty" : "pipe",
				command: "",
				args: [],
				cwd: "",
				cols: 80,
				rows: 24,
				createdAt: 0,
				icon: toIconClass(session.icon),
				channel: HUB_TERMINAL_STREAM_CHANNEL,
				restartable: session.restartable
			});
		}
		return [...own, ...foreign];
	}
	getPresets() {
		return this.presets.map((p) => ({ ...p }));
	}
	buildEnv(extra) {
		const base = {};
		for (const [k, v] of Object.entries(process.env)) if (v !== void 0) base[k] = v;
		return {
			...base,
			TERM: "xterm-256color",
			COLORTERM: "truecolor",
			FORCE_COLOR: "1",
			...this.options.env,
			...extra
		};
	}
	resolveSpawn(req) {
		const cols = req.cols ?? 80;
		const rows = req.rows ?? 24;
		if (req.presetId) {
			const preset = this.presets.find((p) => p.id === req.presetId);
			if (!preset) throw diagnostics.DP_TERMINALS_0006({ id: req.presetId });
			return {
				command: preset.command,
				args: req.args ?? preset.args ?? [],
				cwd: req.cwd ?? preset.cwd ?? this.defaultCwd,
				mode: req.mode ?? preset.mode ?? "readonly",
				env: this.buildEnv({
					...preset.env,
					...req.env
				}),
				title: req.title ?? preset.title,
				cols,
				rows,
				presetId: preset.id
			};
		}
		if (req.command) {
			if (!this.allowArbitraryCommands && req.command !== this.shell) throw diagnostics.DP_TERMINALS_0002({ command: req.command });
			return {
				command: req.command,
				args: req.args ?? [],
				cwd: req.cwd ?? this.defaultCwd,
				mode: req.mode ?? "interactive",
				env: this.buildEnv(req.env),
				title: req.title ?? req.command,
				cols,
				rows
			};
		}
		return {
			command: this.shell,
			args: this.shellArgs,
			cwd: req.cwd ?? this.defaultCwd,
			mode: req.mode ?? this.defaultMode,
			env: this.buildEnv(req.env),
			title: req.title ?? "Shell",
			cols,
			rows
		};
	}
	/**
	* Spawn a session and return its descriptor immediately. The OS process
	* is launched in the background and streams into the session's stream as
	* soon as it produces output; clients can subscribe by id right away.
	*/
	spawn(req = {}) {
		const spawn = this.resolveSpawn(req);
		const id = nanoid();
		const usePty = spawn.mode === "interactive" && this.ptyAvailable;
		const sink = this.channel.start({ id });
		const info = {
			id,
			title: spawn.title,
			mode: spawn.mode,
			status: "running",
			backend: usePty ? "pty" : "pipe",
			command: spawn.command,
			args: spawn.args,
			cwd: spawn.cwd,
			cols: spawn.cols,
			rows: spawn.rows,
			presetId: spawn.presetId,
			createdAt: Date.now()
		};
		const session = {
			info,
			sink,
			spawn
		};
		this.sessions.set(id, session);
		this.launch(session);
		this.publish();
		return { ...info };
	}
	async launch(session) {
		const { spawn, sink } = session;
		let proc;
		try {
			proc = await spawnBackend({
				command: spawn.command,
				args: spawn.args,
				cwd: spawn.cwd,
				env: spawn.env,
				cols: spawn.cols,
				rows: spawn.rows,
				input: spawn.mode === "interactive"
			}, spawn.mode === "interactive");
		} catch (error) {
			const reason = error instanceof Error ? error.message : String(error);
			diagnostics.DP_TERMINALS_0004({
				command: spawn.command,
				reason
			}, { method: "warn" });
			session.info.status = "error";
			session.info.pid = void 0;
			if (!sink.closed) sink.write(`\r\n\x1B[31m[failed to start: ${reason}]\x1B[0m\r\n`);
			this.publish();
			return;
		}
		session.proc = proc;
		session.info.status = "running";
		session.info.exitCode = void 0;
		session.info.backend = proc.backend;
		session.info.pid = proc.pid;
		session.osc?.dispose();
		session.info.termTitle = void 0;
		session.info.termCwd = void 0;
		const osc = new OSCInspector();
		session.osc = osc;
		osc.onStateChange((state) => {
			const termTitle = state.title?.trim() || void 0;
			const termCwd = state.cwd?.path;
			if (termTitle !== session.info.termTitle || termCwd !== session.info.termCwd) {
				session.info.termTitle = termTitle;
				session.info.termCwd = termCwd;
				this.publish();
			}
		});
		proc.onData((data) => {
			osc.feed(data);
			if (!sink.closed) sink.write(data);
		});
		proc.onExit((code) => {
			if (session.proc !== proc) return;
			this.stopProcessPoll(session);
			session.osc?.dispose();
			session.osc = void 0;
			session.info.status = code === 0 ? "exited" : "error";
			session.info.exitCode = code;
			session.info.pid = void 0;
			session.info.processName = void 0;
			if (!sink.closed) sink.write(`\r\n\x1B[2m[process exited with code ${code}]\x1B[0m\r\n`);
			this.publish();
		});
		this.startProcessPoll(session);
		this.publish();
	}
	/**
	* Poll the PTY foreground process name and reflect changes (e.g. `bash`
	* → `vim` → `bash`) into the session info. The interval is `unref`'d so
	* it never keeps the process alive on its own.
	*/
	startProcessPoll(session) {
		if (!session.proc?.getProcessName) return;
		const poll = () => {
			const name = session.proc?.getProcessName?.();
			if (name && name !== session.info.processName) {
				session.info.processName = name;
				this.publish();
			}
		};
		poll();
		session.pollTimer = setInterval(poll, PROCESS_POLL_INTERVAL);
		session.pollTimer.unref?.();
	}
	stopProcessPoll(session) {
		if (session.pollTimer) {
			clearInterval(session.pollTimer);
			session.pollTimer = void 0;
		}
	}
	write(id, data) {
		const session = this.sessions.get(id);
		if (!session) throw diagnostics.DP_TERMINALS_0001({ id });
		if (session.info.mode !== "interactive") throw diagnostics.DP_TERMINALS_0003({ id });
		if (session.info.status !== "running") return;
		session.proc?.write(data);
	}
	resize(id, cols, rows) {
		const session = this.sessions.get(id);
		if (!session) throw diagnostics.DP_TERMINALS_0001({ id });
		session.info.cols = cols;
		session.info.rows = rows;
		session.spawn.cols = cols;
		session.spawn.rows = rows;
		session.proc?.resize(cols, rows);
	}
	/** Stop the process but keep the session (and its stream) around. */
	terminate(id) {
		const session = this.sessions.get(id);
		if (!session) throw diagnostics.DP_TERMINALS_0001({ id });
		session.proc?.kill();
	}
	/** Set a custom display name. Pass an empty string to clear it. */
	rename(id, title) {
		const session = this.sessions.get(id);
		if (!session) throw diagnostics.DP_TERMINALS_0001({ id });
		const trimmed = title.trim();
		session.info.customTitle = trimmed.length ? trimmed : void 0;
		this.publish();
	}
	/** Restart the session's command in place, reusing the same stream id. */
	restart(id) {
		const session = this.sessions.get(id);
		if (!session) throw diagnostics.DP_TERMINALS_0001({ id });
		const previous = session.proc;
		session.proc = void 0;
		this.stopProcessPoll(session);
		previous?.kill();
		if (!session.sink.closed) session.sink.write("\r\n\x1B[2m[restarting…]\x1B[0m\r\n");
		session.info.status = "running";
		session.info.exitCode = void 0;
		this.launch(session);
		this.publish();
		return { ...session.info };
	}
	/** Kill the process, close the stream, and drop the session. */
	remove(id) {
		const session = this.sessions.get(id);
		if (!session) throw diagnostics.DP_TERMINALS_0001({ id });
		this.disposeSession(id, session);
		this.publish();
	}
	/**
	* Drop every stopped (non-running) session this manager owns, closing their
	* streams and forgetting their scrollback. Sessions still running are left
	* untouched; aggregated hub sessions aren't owned here so they never appear
	* in {@link sessions}. Publishes once for the whole sweep.
	*/
	clearExited() {
		for (const [id, session] of this.sessions) if (session.info.status !== "running") this.disposeSession(id, session);
		this.publish();
	}
	/**
	* Tear a single session down: kill its process, stop polling, dispose the
	* OSC inspector, close the sink, and drop it from the store, all without
	* publishing. Callers publish once they've finished mutating the store.
	*/
	disposeSession(id, session) {
		const proc = session.proc;
		session.proc = void 0;
		this.stopProcessPoll(session);
		session.osc?.dispose();
		session.osc = void 0;
		proc?.kill();
		if (!session.sink.closed) session.sink.close();
		this.sessions.delete(id);
	}
	/** Tear everything down; used on server shutdown and in tests. */
	dispose() {
		for (const session of this.sessions.values()) {
			this.stopProcessPoll(session);
			session.osc?.dispose();
			session.osc = void 0;
			session.proc?.kill();
			if (!session.sink.closed) session.sink.close();
		}
		this.sessions.clear();
		this.publish();
	}
	publish() {
		this.refreshSessionsState();
		this.syncHub();
	}
	/**
	* Push the current session list (own + aggregated hub sessions) into shared
	* state. Kept separate from {@link publish} so the hub-session listener can
	* refresh without re-running {@link syncHub} (which would re-emit hub events
	* and loop).
	*/
	refreshSessionsState() {
		this.sessionsState?.mutate((draft) => {
			draft.sessions = this.list();
		});
	}
	/**
	* Reflect the live session list into the hub's terminals subsystem when this
	* devframe is mounted in a hub. `ctx.terminals` only exists on a
	* `DevframeHubContext`, so it's accessed by duck-typing: standalone runtimes
	* (CLI / Vite / build) have no such property and skip silently. This is what
	* surfaces the plugin's sessions in the hub's own terminals panel.
	*/
	syncHub() {
		const hub = this.ctx.terminals;
		if (!hub?.register) return;
		const live = /* @__PURE__ */ new Set();
		for (const { info } of this.sessions.values()) {
			live.add(info.id);
			const entry = {
				id: info.id,
				title: info.customTitle || info.termTitle || info.processName || info.title,
				description: `${info.mode} · ${info.backend}${info.pid ? ` · pid ${info.pid}` : ""}`,
				status: HUB_STATUS[info.status] ?? "stopped",
				icon: "ph:terminal-window-duotone"
			};
			if (hub.sessions.has(info.id)) hub.update(entry);
			else {
				hub.register(entry);
				this.hubOwned.add(info.id);
			}
		}
		for (const id of [...this.hubOwned]) if (!live.has(id)) {
			hub.remove?.({ id });
			this.hubOwned.delete(id);
		}
	}
};
//#endregion
//#region src/node/setup.ts
/**
* Wire the terminals subsystem onto a devframe node context: create the
* {@link TerminalManager}, publish presets + the session list into shared
* state, and register the control RPC functions. Returns the manager so
* callers can spawn sessions or dispose it on shutdown.
*
* Works in any devframe runtime (CLI, Vite, build), since it only depends on the
* core `ctx.rpc` streaming + shared-state surface, not on the hub.
*/
async function setupTerminals(ctx, options = {}) {
	const manager = new TerminalManager(ctx, options);
	setTerminalManager(ctx, manager);
	await manager.init();
	for (const fn of serverFunctions) ctx.rpc.register(fn);
	return manager;
}
//#endregion
export { TerminalManager, diagnostics, getTerminalManager, isPtyAvailable, setTerminalManager, setupTerminals };
