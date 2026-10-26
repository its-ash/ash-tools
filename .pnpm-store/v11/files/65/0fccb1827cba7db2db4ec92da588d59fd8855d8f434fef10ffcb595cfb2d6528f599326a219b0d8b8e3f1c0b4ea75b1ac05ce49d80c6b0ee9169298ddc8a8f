import { addBuildPlugin } from '@nuxt/kit';
import { resolve } from 'pathe';
import { runtimeDir } from '../dirs.mjs';
import escapeRE from 'escape-string-regexp';
import MagicString from 'magic-string';
import { findStaticImports, parseStaticImport } from 'mlly';
import { createUnplugin } from 'unplugin';
import 'node:path';
import 'node:url';

const HELPER_NAME = "__nuxtTimelineWrap";
const RENAME_PREFIX = "_$__";
const RUNTIME_DIR_RE = new RegExp(`^${escapeRE(runtimeDir)}`);
function importKey(source, name) {
  return `${source}\0${name}`;
}
function TimelineWrapPlugin(options) {
  return createUnplugin(() => ({
    name: "nuxt:devtools:timeline-function-wrap",
    enforce: "post",
    transform: {
      filter: {
        id: {
          include: [/^[^?]*\.(?:m?[jt]sx?|vue)(?:$|\?)/],
          exclude: [
            /^\0/,
            /[\\/]node_modules[\\/]/,
            /[?&]macro=true/,
            /[?&]type=(?:style|template|custom)\b/,
            RUNTIME_DIR_RE
          ]
        },
        code: { include: /\bimport\b/ }
      },
      async handler(code) {
        const wrappable = await options.getWrappable();
        if (!wrappable.size)
          return;
        return wrapTimelineImports(code, wrappable, options.helperPath);
      }
    }
  }));
}
function wrapTimelineImports(code, wrappable, helperPath) {
  if (code.includes(HELPER_NAME))
    return;
  const s = new MagicString(code);
  const wrappers = [];
  for (const imp of findStaticImports(code)) {
    const { specifier, defaultImport, namespacedImport, namedImports } = parseStaticImport(imp);
    let wrapped = false;
    const importAs = (name, local) => {
      if (!wrappable.has(importKey(specifier, name)))
        return local;
      wrapped = true;
      wrappers.push(`const ${local} = ${HELPER_NAME}(${JSON.stringify(name === "default" ? local : name)}, ${RENAME_PREFIX}${local});`);
      return RENAME_PREFIX + local;
    };
    const clause = [];
    if (defaultImport)
      clause.push(importAs("default", defaultImport));
    if (namespacedImport)
      clause.push(`* as ${namespacedImport}`);
    const named = Object.entries(namedImports ?? {}).map(([name, local]) => {
      const bound = importAs(name, local);
      return name === bound ? name : `${name} as ${bound}`;
    });
    if (named.length)
      clause.push(`{ ${named.join(", ")} }`);
    if (!wrapped)
      continue;
    const clauseStart = imp.start + imp.code.indexOf(imp.imports);
    s.overwrite(clauseStart, clauseStart + imp.imports.length, `${clause.join(", ")} `);
  }
  if (!wrappers.length)
    return;
  s.prepend([
    `import { ${HELPER_NAME} } from ${JSON.stringify(helperPath)};`,
    ...wrappers,
    ""
  ].join("\n"));
  return {
    code: s.toString(),
    map: s.generateMap({ hires: "boundary" })
  };
}

const DEFINE_UPPER_RE = /^define[A-Z]/;
function setup({ nuxt, options }) {
  const helperPath = resolve(runtimeDir, "function-metrics-helpers");
  const includeFrom = options.timeline?.functions?.includeFrom || [
    "#app",
    "@unhead/vue"
  ];
  const include = options.timeline?.functions?.include || [
    (i) => includeFrom.includes(i.from),
    (i) => i.from.includes("composables")
  ];
  const exclude = options.timeline?.functions?.exclude || [
    DEFINE_UPPER_RE
  ];
  function filter(item) {
    if (item.type)
      return false;
    const name = item.as || item.name;
    if (!include.some((f) => typeof f === "function" ? f(item) : typeof f === "string" ? name === f : f.test(name)))
      return false;
    if (exclude.some((f) => typeof f === "function" ? f(item) : typeof f === "string" ? name === f : f.test(name)))
      return false;
    return true;
  }
  let unimport;
  nuxt.hook("imports:context", (ctx) => {
    unimport = ctx;
  });
  async function getWrappable() {
    if (!unimport)
      return /* @__PURE__ */ new Set();
    const imports = await unimport.getImports();
    const factoryNames = new Set(nuxt.options.optimization.keyedComposableFactories?.map((f) => f.name));
    return new Set(
      imports.filter((i) => filter(i) && !factoryNames.has(i.name)).map((i) => importKey(i.from, i.name))
    );
  }
  nuxt.hook("modules:done", () => {
    nuxt.hook("build:before", () => {
      addBuildPlugin(TimelineWrapPlugin({ helperPath, getWrappable }));
    });
  });
}

export { setup };
