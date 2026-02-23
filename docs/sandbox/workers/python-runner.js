let pyodideReady = null;

const loadPyodideRuntime = async () => {
  if (!pyodideReady) {
    importScripts("https://cdn.jsdelivr.net/pyodide/v0.24.1/full/pyodide.js");
    pyodideReady = loadPyodide({
      stdin: () => "",
      stdout: (text) => self.postMessage({ type: "stdout", text: text + "\n" }),
      stderr: (text) => self.postMessage({ type: "stderr", text: text + "\n" }),
    });
  }
  return pyodideReady;
};

self.onmessage = async (event) => {
  const { type, code, input = "" } = event.data || {};
  if (type !== "run") return;

  try {
    const pyodide = await loadPyodideRuntime();

    // Wrap user's code so built-in input() reads from provided `input` string line-by-line.
    // We create an iterator over lines; input() will return the next line or raise EOFError.
    const wrapper = `import builtins\n` +
      `_lines = iter(${JSON.stringify(input)}.splitlines(True))\n` +
      `def _input(prompt=''):\n` +
      `    try:\n` +
      `        return next(_lines).rstrip('\\n')\n` +
      `    except StopIteration:\n` +
      `        raise EOFError('EOF when reading a line')\n` +
      `builtins.input = _input\n` +
      `\n` + code;

    const result = await pyodide.runPythonAsync(wrapper);
    if (result !== undefined) {
      self.postMessage({ type: "stdout", text: String(result) + "\n" });
    }
    self.postMessage({ type: "result", ok: true, exitCode: 0 });
  } catch (err) {
    self.postMessage({ type: "stderr", text: (err && err.stack) ? err.stack + "\n" : String(err) + "\n" });
    self.postMessage({ type: "result", ok: false, exitCode: 1 });
  }
};
