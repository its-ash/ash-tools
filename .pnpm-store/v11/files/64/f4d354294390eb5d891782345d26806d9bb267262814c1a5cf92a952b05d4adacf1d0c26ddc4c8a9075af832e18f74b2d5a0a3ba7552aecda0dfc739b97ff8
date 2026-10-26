import { CodeServerBackend, CodeServerConnect, CodeServerDetection, CodeServerLogin, CodeServerMode, CodeServerOptions, CodeServerStartRequest, CodeServerStartResult, CodeServerStatusResult } from "./types.mjs";
import { DevframeNodeContext } from "devframe";
//#region src/node/supervisor.d.ts
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
export declare class CodeServerSupervisor {
  private readonly ctx;
  private readonly mode;
  private readonly explicitBackend?;
  private readonly explicitBin?;
  private readonly workspace;
  private readonly host;
  private readonly forcedPort?;
  private readonly extraArgs;
  private readonly extraEnv;
  private readonly cookieSuffix?;
  private readonly cookieName;
  private readonly startTimeout;
  private readonly reuseExistingServer;
  private readonly tunnelName;
  /** Resolved after the first detection. */
  private backend;
  private bin;
  private profile;
  private state?;
  private detection;
  private server;
  private proc?;
  /** Context of the live launch, used to compute the client connect descriptor. */
  private launchCtx?;
  /** Whether the running server was adopted (reused) rather than launched. */
  private adopted;
  /** Captured `vscode.dev` URL for a running tunnel. */
  private readyUrl?;
  private logBuffer;
  private exitHandler?;
  /** Stable id of the hub terminal session, reused across start/stop. */
  private readonly sessionId;
  /** The live hub terminal session when launched through `ctx.terminals`. */
  private session?;
  constructor(ctx: DevframeNodeContext, options?: CodeServerOptions);
  /** Resolve shared state, register process-exit cleanup, run first detection. */
  init(): Promise<void>;
  /**
   * Probe for a usable editor binary and publish the result. Resolves the
   * backend + binary when the caller left them implicit: tunnel mode always
   * uses `code`; an explicit backend or `bin` is honored as-is; otherwise the
   * plugin tries each {@link AUTO_DETECT_ORDER} candidate and keeps the first
   * that is installed.
   */
  detect(): Promise<CodeServerDetection>;
  /** Current status (+ connect info when running) for the launcher UI. */
  status(): CodeServerStatusResult;
  /**
   * The server info projected to clients and shared state, tagged with the
   * live hub terminal session id when one exists so the launcher can offer a
   * "view in terminal" jump. Standalone runtimes hold no session and leave it
   * undefined.
   */
  private serverInfo;
  /**
   * Launch the editor (if not already up) and resolve once it is reachable.
   * Idempotent while starting/running; returns the live status instead of
   * spawning a second process. In tunnel mode it resolves as soon as either
   * the `vscode.dev` URL or a device-login prompt is seen, so the action never
   * blocks on interactive authentication.
   */
  start(req?: CodeServerStartRequest): Promise<CodeServerStartResult>;
  /** Adopt an already-running local server on the target port, if one answers. */
  private tryAdopt;
  /**
   * Resolve the port to launch on: 0 for tunnels/dynamic launches, otherwise a
   * free port starting from the forced port (if any) or the default -
   * verified free before it's ever handed to the editor binary's `--port`.
   */
  private resolveInitialPort;
  /** The child's environment: inherited env, caller overrides, then profile shaping. */
  private buildLaunchEnv;
  /** Buffer output and latch the port / login / ready-URL signals from each line. */
  private consumeOutput;
  private latchPort;
  private latchLogin;
  private latchReadyUrl;
  private handleChildError;
  private handleChildExit;
  /** Wait for the dynamic port then readiness, and flip to `running`. */
  private finalizeLocalStart;
  private handleStartFailure;
  /** Stop the editor process and reset to `stopped`. */
  stop(): CodeServerStatusResult;
  /** Kill the process on host shutdown / test teardown. */
  dispose(): void;
  /** Resolved backend for tests / callers. */
  get resolvedBackend(): CodeServerBackend;
  private setResolved;
  private baseCtx;
  /** Clear per-launch process state (keeps `server`/`detection`). */
  private reset;
  /**
   * Resolve start() for a tunnel: succeed as soon as the `vscode.dev` URL
   * appears (→ running) or a device-login prompt is seen (→ starting, so the
   * user can authenticate while the log stream continues to a running URL).
   */
  private awaitTunnel;
  private connectInfo;
  private terminate;
  /**
   * Resolve the hub's terminals subsystem when this devframe is mounted in a
   * hub. `ctx.terminals` only exists on a `DevframeHubContext`, so it is
   * duck-typed: standalone runtimes (CLI / Vite / build) have no such property
   * and fall back to a direct child process.
   */
  private resolveHubTerminals;
  /** Update the mirrored hub terminal session's status, when one exists. */
  private reflectHub;
  /**
   * Launch the editor binary. In a hub, spawn it through `ctx.terminals` so it
   * shows up as a read-only terminal session (proper icon + name) whose output
   * the hub streams to its terminals panel; standalone, spawn it directly.
   * Either way, return the underlying {@link ChildProcess} so the shared
   * readiness / port / log wiring in `start()` is identical.
   */
  private launchProcess;
  /** Spawn through the hub's terminals subsystem so it shows as a terminal session. */
  private launchViaHub;
  /** Spawn the editor binary directly (standalone runtimes with no hub). */
  private launchDirect;
  private appendLog;
  private lastLog;
  private publish;
  /**
   * Poll the server's readiness path until it responds or the timeout elapses.
   * Returns false if the process exits first.
   */
  private waitForReady;
  private registerCleanup;
}
//#endregion
//#region src/node/backends.d.ts
/**
 * The internal launch profile the supervisor drives. One of three "kinds":
 * the two local {@link CodeServerBackend}s plus the `code tunnel` profile
 * selected by `mode: 'tunnel'`. Each profile owns the backend-specific pieces
 * (binary, arguments, auth env, readiness detection, and how the client
 * ultimately reaches the editor), while the supervisor owns the shared
 * spawn / log / publish lifecycle.
 */
type CodeServerProfileKind = 'code-server' | 'serve-web' | 'tunnel';
/** Everything a profile needs to build a launch and the client's connect info. */
interface ProfileContext {
  host: string;
  /** Resolved local port (0 until dynamically allocated). Unused by tunnel. */
  port: number;
  folder: string;
  /** Fresh per-launch secret (session token / connection token). */
  secret: string;
  /** Session cookie name for the `code-server` backend. */
  cookieName: string;
  extraArgs: string[];
  /** Machine name for the tunnel profile. */
  tunnelName: string;
}
interface CodeServerProfile {
  kind: CodeServerProfileKind;
  /** Public backend id (tunnel reports `ms-code-serve-web`'s sibling `code` binary). */
  backend: CodeServerBackend;
  /** Default binary when the caller doesn't override `bin`. */
  defaultBin: string;
  /** Build the argv passed to the binary. */
  buildArgs: (c: ProfileContext) => string[];
  /** Merge auth material into the child environment. */
  buildEnv: (c: ProfileContext, base: Record<string, string>) => Record<string, string>;
  /** Local readiness path polled over HTTP, or `null` for log-driven (tunnel). */
  healthPath: string | null;
  /** Parse a log line for a dynamically-bound local port. */
  matchPort?: (line: string) => number | undefined;
  /** Parse a log line for the tunnel's `vscode.dev` URL (marks it ready). */
  matchReadyUrl?: (line: string) => string | undefined;
  /** Parse a log line for a device-login prompt. */
  matchLogin?: (line: string) => CodeServerLogin | undefined;
  /** Compute the client connect descriptor for a freshly launched server. */
  connect: (c: ProfileContext & {
    readyUrl?: string;
  }) => CodeServerConnect;
  /** Connect descriptor for an adopted (reused) server we didn't launch. */
  connectReused: (c: {
    port: number;
  }) => CodeServerConnect;
}
/** Resolve the launch profile for a mode + backend. */
export declare function resolveProfile(mode: CodeServerMode, backend: CodeServerBackend): CodeServerProfile;
//#endregion
//#region src/node/context.d.ts
export declare function setCodeServerSupervisor(ctx: DevframeNodeContext, supervisor: CodeServerSupervisor): void;
export declare function getCodeServerSupervisor(ctx: DevframeNodeContext): CodeServerSupervisor;
//#endregion
//#region src/node/detect.d.ts
interface DetectCodeServerResult {
  installed: boolean;
  version?: string;
  bin: string;
}
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
export declare function detectCodeServer(bin?: string, timeoutMs?: number): Promise<DetectCodeServerResult>;
//#endregion
//#region ../../node_modules/.pnpm/nostics@1.3.0/node_modules/nostics/dist/diagnostic-C0zzcNxA.d.mts
//#region src/utils.d.ts
/**
 * A value of type T, or a function that resolves T from a single params object.
 *
 * @internal
 */
type ValueOrFn<T, P = any> = T | ((params: P) => T);
/**
 * Extracts the param type from a single-arg function, or `never` for
 * non-function inputs. Pairs with {@link ValueOrFn}.
 *
 * @internal
 */
type ExtractFnParam<T> = T extends ((params: infer P) => any) ? P : never;
/**
 * Converts a union of types to their intersection.
 *
 * @internal
 */
type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends ((k: infer I) => void) ? I : never;
/**
 * `true` when `T` is the `any` type.
 *
 * @internal
 */
type IsAny<Type> = 0 extends 1 & Type ? true : false;
/**
 * `true` when `T` is the `unknown` type (and not `any`).
 *
 * @internal
 */
type IsUnknown<Type> = IsAny<Type> extends true ? false : unknown extends Type ? true : false;
/**
 * Expands a type to its property listing so editor hovers show the resolved
 * shape instead of a chain of aliases / intersections.
 *
 * @internal
 */
type Prettify<Type> = { [Key in keyof Type]: Type[Key]; };
//#endregion
//#region src/diagnostic.d.ts
/**
 * Define-time shape of a diagnostic. Each field can be a static value or a
 * function that resolves it from a shared `params` object passed at call
 * time. Runtime-only fields (`cause`, `sources`) from {@link DiagnosticInit}
 * are intentionally omitted: they're only meaningful at the call site.
 */
interface DiagnosticDefinition<P = any, Data extends Record<string, unknown> = Record<string, unknown>> {
  /**
   * The error message: why this failed. String, or a function of `params`.
   *
   * @example
   * ```ts
   * why: (p: { name: string }) => `module "${p.name}" failed to load`
   * ```
   */
  why: ValueOrFn<string, P>;
  /**
   * Actionable instructions on how to resolve the problem. String, or a
   * function of `params`.
   *
   * @example
   * ```ts
   * fix: (p: { name: string }) => `run "npm install ${p.name}"`
   * ```
   */
  fix?: ValueOrFn<string, P>;
  /**
   * Per-code docs URL. A string overrides
   * {@link DefineDiagnosticsOptions.docsBase} for this code; `false` opts this
   * code out entirely, even when `docsBase` is set. When omitted, the URL is
   * derived from `docsBase`.
   */
  docs?: string | false;
  /**
   * Structured information associated with this diagnostic. A function can
   * derive the data from the same params object as `why` and `fix`.
   */
  data?: ValueOrFn<Data, P>;
}
/**
 * Runtime-only fields that can be passed alongside the interpolation params
 * at call time. Merged into the same object so callers pass everything in
 * one place.
 */
interface DiagnosticCallParams {
  /**
   * Original error or exception that triggered this diagnostic. Pass it
   * through when re-throwing so the original stack trace is preserved.
   */
  cause?: unknown;
  /**
   * Locations in user code that contributed to this diagnostic, in
   * `file:line:column` format. Useful for compilers and other tools where the
   * JS stack trace doesn't reflect the user's source.
   */
  sources?: string[];
}
interface DiagnosticInitBase extends DiagnosticCallParams {
  /**
   * The diagnostic code, e.g. `MATH_E001`. Appear as {@link Diagnostic.name}.
   */
  code: string;
  /**
   * The actual error message: why this failed.
   * Mirrored to `Error.message`.
   */
  why: string;
  /**
   * Optional actionable instructions on how to resolve the problem.
   */
  fix?: string;
  /**
   * URL to extended documentation for this diagnostic.
   */
  docs?: string;
}
/**
 * Structured initializer for a {@link Diagnostic}. `why` is the only required
 * field: it becomes the {@link Diagnostic.message}. The remaining fields are
 * optional metadata that reporters and consumers can render or forward.
 */
type DiagnosticInit<Data = undefined> = DiagnosticInitBase & ([Data] extends [undefined] ? {
  data?: undefined;
} : {
  data: Data;
});
/**
 * Serializable representation returned by {@link Diagnostic.toJSON}.
 */
interface DiagnosticJSON<Data = unknown> {
  name: string;
  why: string;
  fix?: string;
  docs?: string;
  sources?: string[];
  cause?: unknown;
  data: Data;
  stack?: string;
}
/**
 * Permissive reporter constraint used internally so reporters with 1 arg,
 * required options, or optional options all satisfy the array constraint.
 *
 * @internal
 */
type AnyDiagnosticReporter = (diagnostic: Diagnostic<any>, options: any) => void;
/**
 * Resolves the `params` type a code expects from the intersection of params
 * across all function-typed fields, falling back to `unknown` when every field is
 * static. Merged with {@link DiagnosticCallParams} at the call site.
 *
 * @internal
 */
type InferCodeParams<Def> = [ExtractFnParam<Def[keyof Def]>] extends [never] ? unknown : UnionToIntersection<ExtractFnParam<Def[keyof Def]>>;
/**
 * Resolves the data stored by a code definition. Codes without data return a
 * diagnostic whose data is `undefined`.
 *
 * @internal
 */
type InferCodeData<Def> = Def extends {
  data: infer Data;
} ? Data extends ((...args: any[]) => infer ResolvedData) ? ResolvedData : Data : undefined;
/**
 * The first positional argument of a {@link DiagnosticHandle} call:
 * interpolation params merged with the runtime-only call-site fields
 * (`cause`, `sources`).
 *
 * @internal
 */
type CallSiteParams<Params> = Params & DiagnosticCallParams;
/**
 * Checks whether an object has required fields. Making every field optional
 * changes assignability only when the original type has required fields.
 *
 * @internal
 */
type HasRequiredFields<Type> = Partial<Type> extends Type ? false : true;
/**
 * Resolves the full argument tuple for a {@link DiagnosticHandle} call.
 * Branches on whether params and reporter options each have required fields.
 * Required positions become required tuple elements, all-optional ones
 * become `?`, and when no reporter declares any options the parameter is
 * omitted entirely.
 *
 * @internal
 */
type ActionArgs<Params, ReporterOpts> = keyof ReporterOpts extends never ? HasRequiredFields<Params> extends false ? [params?: CallSiteParams<Params>] : [params: CallSiteParams<Params>] : HasRequiredFields<ReporterOpts> extends false ? HasRequiredFields<Params> extends false ? [params?: CallSiteParams<Params>, reporterOptions?: ReporterOpts] : [params: CallSiteParams<Params>, reporterOptions?: ReporterOpts] : HasRequiredFields<Params> extends false ? [params: CallSiteParams<Params> | undefined, reporterOptions: ReporterOpts] : [params: CallSiteParams<Params>, reporterOptions: ReporterOpts];
/**
 * Per-code handle exposed by {@link defineDiagnostics}. Each code is a
 * callable: invoke it to build the diagnostic and run every reporter, or
 * prefix the call with `throw` to raise it.
 *
 * @example
 * ```ts
 * diagnostics.MATH_E001({ name: 'x' })           // report
 * throw diagnostics.MATH_E001({ name: 'x' })     // throw
 * ```
 */
interface DiagnosticHandle<Params, ReporterOpts, Data = undefined> {
  /**
   * Builds the diagnostic, runs every reporter, and returns the diagnostic
   * instance. The returned diagnostic can be inspected, attached as `cause`,
   * or thrown with `throw`.
   */
  (...args: ActionArgs<Params, ReporterOpts>): Diagnostic<Data>;
}
/**
 * Return type of {@link defineDiagnostics}.
 */
type Diagnostics<Codes extends Record<string, DiagnosticDefinition>, Reporters extends readonly AnyDiagnosticReporter[], DataOverride = never> = { [Code in keyof Codes]: DiagnosticHandle<InferCodeParams<Codes[Code]>, Prettify<ExtractReportersOptions<Reporters>>, [DataOverride] extends [never] ? InferCodeData<Codes[Code]> : DataOverride>; };
type StackTraceFrame = ((...args: never[]) => unknown) | (abstract new (...args: never[]) => unknown);
declare class Diagnostic<Data = undefined> extends Error {
  name: string;
  /**
   * The diagnostic code, e.g. `MATH_E001`.
   * Also appears as the `name` property.
   */
  code: string;
  /**
   * URL to extended documentation for this diagnostic code.
   * Auto-generated from {@link DefineDiagnosticsOptions.docsBase}.
   */
  docs?: string;
  /**
   * Optional actionable instructions on how to resolve the problem.
   */
  fix?: string;
  /**
   * Locations in user code that contributed to this diagnostic, in
   * `file:line:column` format. Relevant when the stack trace doesn't reflect
   * the user's source (e.g. compilers, bundlers), otherwise redundant with the
   * stack and should be omitted.
   */
  sources?: string[];
  /**
   * Structured information associated with this diagnostic code.
   */
  data: Data;
  /**
   * Alias for {@link Error.message}: the reason this diagnostic was raised.
   */
  get why(): string;
  /**
   * @param init        structured initializer; `why` is required
   * @param captureFrom V8 stack-cutoff frame. Defaults to {@link Diagnostic}
   * so the top of the trace is the `new Diagnostic(...)` call site.
   * `defineDiagnostics` passes its action method to strip its own frames too.
   * Ignored on engines without `Error.captureStackTrace`.
   */
  constructor(init: DiagnosticInit<Data>, captureFrom?: StackTraceFrame);
  /**
   * Converts the diagnostic into a serializable structured object.
   */
  toJSON(): DiagnosticJSON<Data>;
}
/**
 * Extracts the options object a reporter accepts as its 2nd argument. Returns
 * `unknown` when the reporter has no 2nd arg (so it contributes nothing to the
 * merged shape).
 */
type ExtractSingleReporterOptions<Reporter> = Reporter extends ((diagnostic: Diagnostic, options: infer ReporterOpts) => any) ? IsUnknown<ReporterOpts> extends true ? unknown : Exclude<ReporterOpts, undefined> : unknown;
/**
 * Intersects every reporter's options shape into a single object. If any
 * reporter has a required field, the merged shape has a required field, and
 * {@link ActionArgs} makes `reporterOptions` required when the merged shape
 * has required keys.
 */
type ExtractReportersOptions<Reporters extends readonly any[]> = Reporters extends readonly [infer First, ...infer Rest] ? ExtractSingleReporterOptions<First> & ExtractReportersOptions<Rest> : unknown;
//#endregion
//#region src/node/diagnostics.d.ts
/**
 * Structured diagnostics for the code-server plugin. Uses the plugin's own
 * `DP_CODE_SERVER_` prefix per the built-in plugin convention, keeping it
 * collision-free with devframe core (`DF`) and the hub (`DF8xxx`).
 */
export declare const diagnostics: Diagnostics<{
  readonly DP_CODE_SERVER_0001: {
    readonly why: (p: {
      bin: string;
    }) => string;
    readonly fix: "Install Coder code-server (`curl -fsSL https://code-server.dev/install.sh | sh`) or the Microsoft `code` CLI, or set the `bin` option to its path. See https://coder.com/docs/code-server/latest/install";
  };
  readonly DP_CODE_SERVER_0002: {
    readonly why: (p: {
      port: number;
      timeout: number;
    }) => string;
    readonly fix: "Check the editor logs for startup errors, raise `startTimeout`, or free the port.";
  };
  readonly DP_CODE_SERVER_0003: {
    readonly why: (p: {
      bin: string;
      reason: string;
    }) => string;
  };
  readonly DP_CODE_SERVER_0004: {
    readonly why: "code-server supervisor is not initialised on this context";
    readonly fix: "Call setupCodeServer(ctx) (or use createCodeServerDevframe) before invoking the code-server RPCs.";
  };
  readonly DP_CODE_SERVER_0005: {
    readonly why: (p: {
      code: number;
    }) => string;
    readonly fix: "Inspect the captured output in the launcher and re-launch.";
  };
  readonly DP_CODE_SERVER_0006: {
    readonly why: (p: {
      timeout: number;
    }) => string;
    readonly fix: "Check the tunnel logs, ensure the `code` CLI is signed in, or raise `startTimeout`.";
  };
}, readonly [(d: Diagnostic, { method }?: {
  method?: "log" | "warn" | "error";
}) => void], never>;
//#endregion
//#region src/node/setup.d.ts
/**
 * Wire the code-server subsystem onto a devframe node context: create the
 * {@link CodeServerSupervisor}, run the initial binary detection, publish
 * status into shared state, and register the control RPC functions. Returns
 * the supervisor so callers can launch/stop or dispose it on shutdown.
 *
 * Works in any devframe runtime (CLI, Vite, embedded, build), since it only relies
 * on the core `ctx.rpc` shared-state surface, not on the hub.
 */
export declare function setupCodeServer(ctx: DevframeNodeContext, options?: CodeServerOptions): Promise<CodeServerSupervisor>;
//#endregion
export type { CodeServerProfile, CodeServerProfileKind };