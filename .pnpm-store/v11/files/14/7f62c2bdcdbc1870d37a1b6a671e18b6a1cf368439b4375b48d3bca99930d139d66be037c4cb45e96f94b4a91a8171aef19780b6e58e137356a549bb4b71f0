import { SpawnRequest, TerminalMode, TerminalPreset, TerminalSessionInfo, TerminalsOptions } from "./types.mjs";
import { DevframeNodeContext } from "devframe";
//#region src/node/manager.d.ts
/**
 * Owns terminal session lifecycle: spawns PTY / piped backends, streams
 * their output over the `devframes:plugin:terminals:output` channel (one
 * stream per session, stable for the session's whole life so restarts reuse
 * the same id), and mirrors a serializable session list into shared state.
 */
export declare class TerminalManager {
  private ctx;
  private options;
  readonly shell: string;
  readonly shellArgs: string[];
  readonly defaultCwd: string;
  readonly defaultMode: TerminalMode;
  readonly allowArbitraryCommands: boolean;
  readonly presets: TerminalPreset[];
  private channel;
  private sessionsState?;
  private sessions;
  private ptyAvailable;
  /** Session ids this manager has registered into the hub (when hubbed). */
  private hubOwned;
  constructor(ctx: DevframeNodeContext, options?: TerminalsOptions);
  /** Resolve shared state, probe the PTY backend, publish the preset catalog. */
  init(): Promise<void>;
  /** The hub's terminals subsystem when mounted in a hub, else undefined. */
  private hubTerminals;
  list(): TerminalSessionInfo[];
  getPresets(): TerminalPreset[];
  private buildEnv;
  private resolveSpawn;
  /**
   * Spawn a session and return its descriptor immediately. The OS process
   * is launched in the background and streams into the session's stream as
   * soon as it produces output; clients can subscribe by id right away.
   */
  spawn(req?: SpawnRequest): TerminalSessionInfo;
  private launch;
  /**
   * Poll the PTY foreground process name and reflect changes (e.g. `bash`
   * → `vim` → `bash`) into the session info. The interval is `unref`'d so
   * it never keeps the process alive on its own.
   */
  private startProcessPoll;
  private stopProcessPoll;
  write(id: string, data: string): void;
  resize(id: string, cols: number, rows: number): void;
  /** Stop the process but keep the session (and its stream) around. */
  terminate(id: string): void;
  /** Set a custom display name. Pass an empty string to clear it. */
  rename(id: string, title: string): void;
  /** Restart the session's command in place, reusing the same stream id. */
  restart(id: string): TerminalSessionInfo;
  /** Kill the process, close the stream, and drop the session. */
  remove(id: string): void;
  /**
   * Drop every stopped (non-running) session this manager owns, closing their
   * streams and forgetting their scrollback. Sessions still running are left
   * untouched; aggregated hub sessions aren't owned here so they never appear
   * in {@link sessions}. Publishes once for the whole sweep.
   */
  clearExited(): void;
  /**
   * Tear a single session down: kill its process, stop polling, dispose the
   * OSC inspector, close the sink, and drop it from the store, all without
   * publishing. Callers publish once they've finished mutating the store.
   */
  private disposeSession;
  /** Tear everything down; used on server shutdown and in tests. */
  dispose(): void;
  private publish;
  /**
   * Push the current session list (own + aggregated hub sessions) into shared
   * state. Kept separate from {@link publish} so the hub-session listener can
   * refresh without re-running {@link syncHub} (which would re-emit hub events
   * and loop).
   */
  private refreshSessionsState;
  /**
   * Reflect the live session list into the hub's terminals subsystem when this
   * devframe is mounted in a hub. `ctx.terminals` only exists on a
   * `DevframeHubContext`, so it's accessed by duck-typing: standalone runtimes
   * (CLI / Vite / build) have no such property and skip silently. This is what
   * surfaces the plugin's sessions in the hub's own terminals panel.
   */
  private syncHub;
}
//#endregion
//#region src/node/backend.d.ts
/**
 * Whether real pseudo-terminals are available in this runtime, i.e. zigpty's
 * native bindings loaded. Without them interactive sessions still run through
 * zigpty's pipe-based emulation, with degraded TUI fidelity.
 */
export declare function isPtyAvailable(): Promise<boolean>;
//#endregion
//#region src/node/context.d.ts
export declare function setTerminalManager(ctx: DevframeNodeContext, manager: TerminalManager): void;
export declare function getTerminalManager(ctx: DevframeNodeContext): TerminalManager;
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
 * Structured diagnostics for the terminals plugin. Uses the plugin's own
 * `DP_TERMINALS_` prefix per the built-in plugin convention, keeping it
 * collision-free with devframe core (`DF`) and the hub (`DF8xxx`).
 */
export declare const diagnostics: Diagnostics<{
  readonly DP_TERMINALS_0001: {
    readonly why: (p: {
      id: string;
    }) => string;
    readonly fix: "Spawn a session first, or refresh the session list.";
  };
  readonly DP_TERMINALS_0002: {
    readonly why: (p: {
      command: string;
    }) => string;
    readonly fix: "Add it to `presets`, or pass `allowArbitraryCommands: true` to createTerminalsDevframe().";
  };
  readonly DP_TERMINALS_0003: {
    readonly why: (p: {
      id: string;
    }) => string;
    readonly fix: "Spawn the session with `mode: \"interactive\"` to accept input.";
  };
  readonly DP_TERMINALS_0004: {
    readonly why: (p: {
      command: string;
      reason: string;
    }) => string;
  };
  readonly DP_TERMINALS_0005: {
    readonly why: "Native PTY bindings (zigpty) are unavailable; interactive sessions fall back to pipe-based terminal emulation. Full-screen TUIs may not render correctly.";
    readonly fix: "Check that this platform is covered by zigpty's prebuilds (Linux/macOS/Windows, x64/arm64, glibc/musl) and that the dependency installed intact.";
  };
  readonly DP_TERMINALS_0006: {
    readonly why: (p: {
      id: string;
    }) => string;
  };
  readonly DP_TERMINALS_0007: {
    readonly why: "Terminals manager is not initialised on this context";
    readonly fix: "Call setupTerminals(ctx) (or use createTerminalsDevframe) before invoking terminal RPCs.";
  };
}, readonly [(d: Diagnostic, { method }?: {
  method?: "log" | "warn" | "error";
}) => void], never>;
//#endregion
//#region src/node/setup.d.ts
/**
 * Wire the terminals subsystem onto a devframe node context: create the
 * {@link TerminalManager}, publish presets + the session list into shared
 * state, and register the control RPC functions. Returns the manager so
 * callers can spawn sessions or dispose it on shutdown.
 *
 * Works in any devframe runtime (CLI, Vite, build), since it only depends on the
 * core `ctx.rpc` streaming + shared-state surface, not on the hub.
 */
export declare function setupTerminals(ctx: DevframeNodeContext, options?: TerminalsOptions): Promise<TerminalManager>;
//#endregion