import { addVitePlugin } from '@nuxt/kit';
import { resolve, join } from 'pathe';
import { readdir, lstat } from 'node:fs/promises';
import { isVitePluginInspectAvailable, createVitePluginInspect } from './vite-inspect.mjs';
import '@nuxt/devtools-kit';
import '@vitejs/devtools-kit/node';
import 'local-pkg';
import './module-main.mjs';
import 'node:os';
import 'consola/utils';
import 'devframe/utils/serve-static';
import 'verkit';
import 'vite';
import '../shared/devtools.C3s9ubDZ.mjs';
import '../dirs.mjs';
import 'node:path';
import 'node:url';
import 'node:child_process';
import 'node:fs';
import 'node:util';
import 'tinyglobby';
import 'destr';
import '../../dist/runtime/shared/hooks.js';
import 'devframe/utils/hash';
import 'magicast';
import 'magicast/helpers';
import 'node:process';
import 'node:module';
import '@vitejs/devtools-kit/constants';
import 'perfect-debounce';

async function getFolderSize(dir) {
  const dirents = await readdir(dir, {
    withFileTypes: true
  });
  if (dirents.length === 0)
    return 0;
  const files = [];
  const directorys = [];
  for (const dirent of dirents) {
    if (dirent.isFile()) {
      files.push(dirent);
      continue;
    }
    if (dirent.isDirectory())
      directorys.push(dirent);
  }
  const sizes = await Promise.all(
    [
      files.map(async (file) => {
        const path = resolve(dir, file.name);
        const { size } = await lstat(path);
        return size;
      }),
      directorys.map((directory) => {
        const path = resolve(dir, directory.name);
        return getFolderSize(path);
      })
    ].flat()
  );
  return sizes.reduce((total, size) => total += size, 0);
}

async function setup(nuxt, options) {
  if (options.viteInspect !== false && isVitePluginInspectAvailable(nuxt.options.rootDir)) {
    addVitePlugin(
      await createVitePluginInspect({
        build: true,
        outputDir: join(nuxt.options.analyzeDir, ".vite-inspect")
      })
    );
  }
  nuxt.hook("build:analyze:done", async (meta) => {
    const _meta = meta;
    _meta.size = _meta.size || {};
    const dirs = [join(meta.buildDir, "dist/client"), meta.outDir];
    const [clientBundleSize, nitroBundleSize] = await Promise.all(dirs.map(getFolderSize));
    _meta.size.clientBundle = clientBundleSize;
    _meta.size.nitroBundle = nitroBundleSize;
  });
}

export { setup };
