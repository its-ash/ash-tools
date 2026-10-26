import { createCodeServerDevframe } from '@devframes/plugin-code-server';
import { setupCodeServer } from '@devframes/plugin-code-server/node';
import { deprecate, onDevtoolsReady, NUXT_DEVTOOLS_GROUP_ID } from '@nuxt/devtools-kit';

const RESERVED_ARGS = ["--auth", "--bind-addr", "--cookie-suffix"];
const RESERVED_ENV = ["PASSWORD", "HASHED_PASSWORD"];
function matchesOption(arg, option) {
  return arg === option || arg.startsWith(`${option}=`);
}
function resolveCodeServerOptions(options, rootDir) {
  const args = options?.args;
  const reservedArg = args?.find((arg) => RESERVED_ARGS.some((option) => matchesOption(arg, option)));
  if (reservedArg) {
    throw new Error(
      `[Nuxt DevTools] codeServer.args cannot override ${reservedArg.split("=")[0]}. Configure host and cookieSuffix through their dedicated codeServer options; authentication is managed automatically.`
    );
  }
  const env = options?.env;
  const reservedEnv = RESERVED_ENV.find((key) => Object.hasOwn(env ?? {}, key));
  if (reservedEnv) {
    throw new Error(
      `[Nuxt DevTools] codeServer.env cannot set ${reservedEnv}. Code Server authentication credentials are generated and managed automatically.`
    );
  }
  return {
    // The 0.7 plugin also knows about Microsoft's serve-web and tunnels. Nuxt
    // intentionally exposes only Coder code-server for this integration.
    backend: "code-server",
    bin: options?.bin,
    cwd: options?.cwd ?? rootDir,
    serverPort: options?.serverPort,
    host: options?.host,
    args,
    env,
    cookieSuffix: options?.cookieSuffix,
    startTimeout: options?.startTimeout
  };
}
function setup(ctx) {
  const { nuxt, options } = ctx;
  if (options.vscode !== void 0) {
    deprecate(nuxt, "NDT_DEP_0008", {
      api: "devtools.vscode",
      replacement: "devtools.codeServer"
    });
  }
  if (options.codeServer?.enabled === false)
    return;
  const codeServerOptions = resolveCodeServerOptions(options.codeServer, nuxt.options.rootDir);
  const definition = createCodeServerDevframe(codeServerOptions);
  let supervisor;
  let setupPromise;
  const mountedDefinition = {
    ...definition,
    async setup(devframeCtx) {
      setupPromise = setupCodeServer(devframeCtx, codeServerOptions);
      supervisor = await setupPromise;
    }
  };
  nuxt.hook("close", async () => {
    await setupPromise;
    supervisor?.dispose();
  });
  onDevtoolsReady((kit) => {
    return kit.install(mountedDefinition, {
      dock: {
        groupId: NUXT_DEVTOOLS_GROUP_ID,
        category: "modules"
      }
    });
  }, nuxt);
}

export { resolveCodeServerOptions, setup };
