
// Loads clang.wasm from CDN and exposes self.clangWasmToolchain.compile for the sandbox
async function loadClangWasmToolchain() {
  if (self.clangWasmToolchain) return;
  // Load the clang.js loader from CDN
  importScripts('https://webassembly.sh/clang/clang.js');
  // Wait for the toolchain to initialize (clang.js should set up self.clangWasmToolchain)
  if (!self.clangWasmToolchain) {
    throw new Error('clangWasmToolchain not initialized by clang.js');
  }
}

self.compileCToWasm = async (source, log) => {
  log = log || (() => {});
  log('C/C++ toolchain adapter loaded from CDN.');
  await loadClangWasmToolchain();
  if (!self.clangWasmToolchain) {
    throw new Error('clangWasmToolchain not found after loading clang.js');
  }
  const result = await self.clangWasmToolchain.compile(source, log);
  if (!result || !result.wasmBytes) {
    throw new Error('Toolchain compile did not return wasmBytes');
  }
  return result.wasmBytes;
};
