import { useNuxt } from '@nuxt/kit';
import { x } from 'tinyexec';
import { defineDiagnostics, createConsoleReporter } from 'nostics';
export { createConsoleReporter, defineDiagnostics } from 'nostics';

function diagnosticsDocsBase(code) {
  return `https://devtools.nuxt.com/module/migration-v4#${String(code).toLowerCase()}`;
}
const diagnosticCodes = {
  /** `startSubprocess().getProcess()` → `getResult()`. */
  NDT_DEP_0001: {
    why: (p) => `\`${p.api}\` is deprecated.`,
    fix: (p) => `Use \`${p.replacement}\` instead.`
  },
  // NDT_DEP_0002 is retired (was `disableAuthorization`, now a supported
  // first-class option). The code is left unused so numbers stay stable.
  /** `extendServerRpc` → `onDevtoolsReady((ctx) => ctx.rpc.register(...))`. */
  NDT_DEP_0003: {
    why: (p) => `\`${p.api}\` is deprecated.`,
    fix: (p) => `Use \`${p.replacement}\` instead.`
  },
  /** `startSubprocess` → `onDevtoolsReady((ctx) => ctx.terminals.startChildProcess(...))`. */
  NDT_DEP_0004: {
    why: (p) => `\`${p.api}\` is deprecated.`,
    fix: (p) => `Use \`${p.replacement}\` instead.`
  },
  /** `addCustomTab` → `onDevtoolsReady((ctx) => ctx.docks.register(...))`. */
  NDT_DEP_0005: {
    why: (p) => `\`${p.api}\` is deprecated.`,
    fix: (p) => `Use \`${p.replacement}\` instead.`
  },
  /** `refreshCustomTabs` → `onDevtoolsReady((ctx) => ctx.docks.register(...))`. */
  NDT_DEP_0006: {
    why: (p) => `\`${p.api}\` is deprecated.`,
    fix: (p) => `Use \`${p.replacement}\` instead.`
  },
  /** Direct `nuxt.devtools.rpc` access (`broadcast` / `functions`). */
  NDT_DEP_0007: {
    why: (p) => `\`${p.api}\` is deprecated.`,
    fix: (p) => `Use \`${p.replacement}\` instead.`
  },
  /** Removed `vscode` module option → `codeServer`. */
  NDT_DEP_0008: {
    why: (p) => `\`${p.api}\` is deprecated and its legacy modes are no longer supported.`,
    fix: (p) => `Use \`${p.replacement}\` instead. The legacy value is ignored rather than partially translated.`
  },
  /** `getServerData()` RPC → the Data Inspector panel's `Nuxt Application` source. */
  NDT_DEP_0009: {
    why: (p) => `\`${p.api}\` is deprecated.`,
    fix: (p) => `Use \`${p.replacement}\` instead.`
  }
};
const consoleDiagnostics = defineDiagnostics({
  docsBase: diagnosticsDocsBase,
  reporters: [createConsoleReporter()],
  codes: diagnosticCodes
});
const stateByCtx = /* @__PURE__ */ new WeakMap();
const fallbackState = { emitted: /* @__PURE__ */ new Set() };
function getState(ctx) {
  if (!ctx)
    return fallbackState;
  let state = stateByCtx.get(ctx);
  if (!state) {
    state = { emitted: /* @__PURE__ */ new Set() };
    stateByCtx.set(ctx, state);
  }
  return state;
}
function getServerContext(nuxt) {
  return nuxt?.devtools;
}
function registerHostDiagnostics(ctx) {
  const host = ctx.devtoolsKit?.diagnostics;
  if (!host)
    return;
  const catalog = host.defineDiagnostics({
    docsBase: diagnosticsDocsBase,
    codes: diagnosticCodes
  });
  host.register(catalog);
  getState(ctx).hostCatalog = catalog;
}
function deprecate(nuxt, code, params, options = {}) {
  const ctx = getServerContext(nuxt);
  const state = getState(ctx);
  const dedupeKey = `${code}:${options.key ?? code}`;
  if (state.emitted.has(dedupeKey))
    return;
  state.emitted.add(dedupeKey);
  const method = options.method ?? "warn";
  const hostHandle = state.hostCatalog?.[code];
  if (hostHandle)
    return hostHandle(params, { method });
  const handle = consoleDiagnostics[code];
  return handle(params, { method });
}
function defineStandaloneDiagnostics(options) {
  return defineDiagnostics({
    ...options,
    reporters: options.reporters ?? [createConsoleReporter()]
  });
}
function deprecateWithNuxt(code, params, options) {
  return deprecate(useNuxt(), code, params, options);
}

const NUXT_DEVTOOLS_GROUP_ID = "nuxt";
function addCustomTab(tab, nuxt = useNuxt()) {
  deprecate(nuxt, "NDT_DEP_0005", {
    api: "addCustomTab",
    replacement: "onDevtoolsReady((ctx) => ctx.docks.register(...))"
  }, { key: typeof tab === "function" ? void 0 : tab.name });
  nuxt.hook("devtools:customTabs", async (tabs) => {
    if (typeof tab === "function")
      tab = await tab();
    tabs.push(tab);
  });
}
function refreshCustomTabs(nuxt = useNuxt()) {
  deprecate(nuxt, "NDT_DEP_0006", {
    api: "refreshCustomTabs",
    replacement: "onDevtoolsReady((ctx) => ctx.docks.register(...).update(...))"
  });
  return nuxt.callHook("devtools:customTabs:refresh");
}
function startSubprocess(execaOptions, tabOptions, nuxt = useNuxt()) {
  deprecate(nuxt, "NDT_DEP_0004", {
    api: "startSubprocess",
    replacement: "onDevtoolsReady((ctx) => ctx.terminals.startChildProcess(...))"
  }, { key: tabOptions.id });
  const id = tabOptions.id;
  let restarting = false;
  function start() {
    const proc = x(
      execaOptions.command,
      execaOptions.args,
      {
        nodeOptions: {
          ...execaOptions.nodeOptions,
          cwd: execaOptions.cwd ?? execaOptions.nodeOptions?.cwd,
          env: {
            ...process.env,
            COLORS: "true",
            FORCE_COLOR: "true",
            ...execaOptions.env,
            ...execaOptions.nodeOptions?.env,
            __CLI_ARGV__: void 0
          }
        }
      }
    );
    nuxt.callHook("devtools:terminal:write", { id, data: `> ${[execaOptions.command, ...execaOptions.args || []].join(" ")}

` });
    proc.process?.stdout?.on("data", (data) => {
      nuxt.callHook("devtools:terminal:write", { id, data: data.toString() });
    });
    proc.process?.stderr?.on("data", (data) => {
      nuxt.callHook("devtools:terminal:write", { id, data: data.toString() });
    });
    proc.process?.on("exit", (code) => {
      if (!restarting) {
        nuxt.callHook("devtools:terminal:write", { id, data: `
> process terminated with ${code}
` });
        nuxt.callHook("devtools:terminal:exit", { id, code: code || 0 });
      }
    });
    return proc;
  }
  register();
  nuxt.hook("close", () => {
    terminate();
  });
  let result = start();
  function restart() {
    restarting = true;
    result.kill();
    clear();
    result = start();
    restarting = false;
  }
  function clear() {
    tabOptions.buffer = "";
    register();
  }
  function terminate() {
    restarting = false;
    try {
      result.kill();
    } catch {
    }
    nuxt.callHook("devtools:terminal:remove", { id });
  }
  function register() {
    nuxt.callHook("devtools:terminal:register", {
      onActionRestart: tabOptions.restartable === false ? void 0 : restart,
      onActionTerminate: tabOptions.terminatable === false ? void 0 : terminate,
      isTerminated: false,
      ...tabOptions
    });
  }
  return {
    /** @deprecated Use `getResult()` instead */
    getProcess: () => {
      deprecate(nuxt, "NDT_DEP_0001", {
        api: "startSubprocess().getProcess()",
        replacement: "getResult()"
      }, { key: id });
      return result.process;
    },
    getResult: () => result,
    terminate,
    restart,
    clear
  };
}
function extendServerRpc(namespace, functions, nuxt = useNuxt()) {
  const ctx = _getContext(nuxt);
  if (!ctx)
    throw new Error("[Nuxt DevTools] Failed to get devtools context.");
  return ctx.extendServerRpc(namespace, functions);
}
function onDevToolsInitialized(fn, nuxt = useNuxt()) {
  nuxt.hook("devtools:initialized", fn);
}
function onDevtoolsReady(fn, nuxt = useNuxt()) {
  nuxt.hook("devtools:ready", fn);
}
function _getContext(nuxt = useNuxt()) {
  return nuxt?.devtools;
}

export { NUXT_DEVTOOLS_GROUP_ID, addCustomTab, consoleDiagnostics, defineStandaloneDiagnostics, deprecate, deprecateWithNuxt, diagnosticCodes, diagnosticsDocsBase, extendServerRpc, onDevToolsInitialized, onDevtoolsReady, refreshCustomTabs, registerHostDiagnostics, startSubprocess };
