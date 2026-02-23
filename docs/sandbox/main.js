
const languageSelect = document.getElementById("languageSelect");
const timeoutSelect = document.getElementById("timeoutSelect");
const runBtn = document.getElementById("runBtn");
const stopBtn = document.getElementById("stopBtn");
const consoleOutput = document.getElementById("consoleOutput");
const errorOutput = document.getElementById("errorOutput");
const compileOutput = document.getElementById("compileOutput");
const timingEl = document.getElementById("timing");
const memoryEl = document.getElementById("memory");
const exitCodeEl = document.getElementById("exitCode");
const statusEl = document.getElementById("status");
const runtimeStatus = document.getElementById("runtimeStatus");
const stdinInput = document.getElementById("stdinInput");
const copyConsoleBtn = document.getElementById("copyConsoleBtn");
const copyErrorBtn = document.getElementById("copyErrorBtn");

let activeWorker = null;
let runTimeoutId = null;
let currentLanguage = "javascript";
let runCompleted = false;

const LANGUAGE_CONFIG = {
  javascript: {
    label: "JavaScript",
    worker: "./workers/js-runner.js",
    starter: `// JS: Worker sandbox\nfunction fib(n) {\n  if (n < 2) return n;\n  return fib(n - 1) + fib(n - 2);\n}\n\nconst result = fib(12);\nconsole.log("fib(12)", result);\nresult;`
  },
  python: {
    label: "Python",
    worker: "./workers/python-runner.js",
    starter: `# Python via Pyodide\nimport math\n\nvalues = [math.sqrt(x) for x in range(1, 8)]\nprint("sqrt", values)\nvalues` 
  },
  rust: {
    label: "Rust",
    worker: "./workers/rust-runner.js",
    starter: `// Rust (WASI)\nuse std::io::{self, Read};\n\nfn main() {\n    let mut input = String::new();\n    io::stdin().read_to_string(&mut input).ok();\n    println!("Hello from Rust!");\n    println!("stdin bytes: {}", input.len());\n}`
  },
  cpp: {
    label: "C/C++",
    worker: "./workers/clang-runner.js",
    starter: `// C++ (WASI)\n#include <iostream>\n#include <string>\n\nint main() {\n    std::string name = "WASM";\n    std::cout << "Hello " << name << " from C++!" << std::endl;\n    return 0;\n}`
  }
};

const editedKey = (lang) => `sandbox.edited.${lang}`;

const formatBytes = (bytes) => {
  if (!Number.isFinite(bytes)) return "n/a";
  if (Math.abs(bytes) < 1024) return `${bytes} B`;
  const kb = bytes / 1024;
  if (Math.abs(kb) < 1024) return `${kb.toFixed(1)} KB`;
  return `${(kb / 1024).toFixed(2)} MB`;
};

// Save Monaco editor contents to localStorage (if Monaco is present)
const saveDraft = (lang = languageSelect.value) => {
  try {
    const getter = (typeof globalThis !== 'undefined' && globalThis.__getMonacoCode) ? globalThis.__getMonacoCode : (window && window.__getMonacoCode ? window.__getMonacoCode : null);
    if (!getter) return;
    const code = getter() || '';
    localStorage.setItem(`sandbox.code.${lang}`, code);
    localStorage.setItem("sandbox.lang", lang);
    localStorage.setItem(editedKey(lang), "true");
  } catch (e) {
    // ignore
  }
};


const setStatus = (text, isError = false) => {
  statusEl.textContent = text;
  statusEl.style.color = isError ? "#f87171" : "";
};

const clearOutputs = () => {
  consoleOutput.textContent = "";
  errorOutput.textContent = "";
  compileOutput.textContent = "";
  timingEl.textContent = "time: --";
  memoryEl.textContent = "--";
  exitCodeEl.textContent = "--";
};

const stopRun = (manual = true) => {
  if (activeWorker) {
    activeWorker.terminate();
    activeWorker = null;
  }
  if (runTimeoutId) {
    clearTimeout(runTimeoutId);
    runTimeoutId = null;
  }
  runBtn.disabled = false;
  stopBtn.disabled = true;
  if (manual) {
    setStatus("Stopped", true);
    runtimeStatus.textContent = "Idle";
  }
  // allow future runs
  runCompleted = false;
};


const runCode = () => {
  // reset run state
  runCompleted = false;
  const lang = languageSelect.value;
  const config = LANGUAGE_CONFIG[lang];
  // Always get code from Monaco
  const code = window.__getMonacoCode ? window.__getMonacoCode() : '';
  const timeoutMs = Number(timeoutSelect.value);

  clearOutputs();
  setStatus("Running...");
  runtimeStatus.textContent = `Executing: ${config.label}`;
  runBtn.disabled = true;
  stopBtn.disabled = false;

  activeWorker = new Worker(config.worker);
  const startTime = performance.now();

  activeWorker.onmessage = (event) => {
    const msg = event.data || {};
    if (runCompleted && msg.type !== 'result') return;
    if (msg.type === "stdout") {
      consoleOutput.textContent += msg.text;
      return;
    }
    if (msg.type === "stderr") {
      errorOutput.textContent += msg.text;
      return;
    }
    if (msg.type === "compile") {
      compileOutput.textContent += msg.text;
      return;
    }
    if (msg.type === "result") {
      // mark completed early to ignore any further stray messages
      runCompleted = true;
      if (runTimeoutId) {
        clearTimeout(runTimeoutId);
        runTimeoutId = null;
      }
      const duration = performance.now() - startTime;
      timingEl.textContent = `time: ${duration.toFixed(1)} ms`;
      memoryEl.textContent = formatBytes(msg.memoryBytes);
      exitCodeEl.textContent = msg.exitCode ?? "--";
      setStatus(msg.ok ? "Completed" : "Failed", !msg.ok);
      runtimeStatus.textContent = "Idle";
      // ensure worker is terminated immediately
      if (activeWorker) {
        try {
          activeWorker.terminate();
        } catch (e) {}
        activeWorker = null;
      }
      stopRun(false); // Do not overwrite status after successful completion
    }
  };

  activeWorker.onerror = (err) => {
    errorOutput.textContent += `Worker error: ${err.message}\n`;
    setStatus("Failed", true);
    runtimeStatus.textContent = "Idle";
    runBtn.disabled = false;
    stopBtn.disabled = true;
  };

  activeWorker.postMessage({ type: "run", code, language: lang, input: stdinInput.value });

  runTimeoutId = setTimeout(() => {
    errorOutput.textContent += `Timeout: exceeded ${timeoutMs} ms\n`;
    stopRun(true);
  }, timeoutMs);
};

runBtn?.addEventListener("click", runCode);
stopBtn?.addEventListener("click", stopRun);


// No need to switch editor, Monaco handles language switching in Vue
languageSelect?.addEventListener("change", (event) => {
  currentLanguage = event.target.value;
  runtimeStatus.textContent = `Ready: ${LANGUAGE_CONFIG[currentLanguage].label}`;
});

window.addEventListener("beforeunload", saveDraft);

document.addEventListener("keydown", (event) => {
  if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "enter") {
    event.preventDefault();
    runCode();
  }
});

copyConsoleBtn?.addEventListener("click", () => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(consoleOutput.textContent).catch(err => {
      console.error("Failed to copy console output:", err);
    });
  } else {
    // Fallback for browsers that don't support Clipboard API
    const textArea = document.createElement("textarea");
    textArea.value = consoleOutput.textContent;
    document.body.appendChild(textArea);
    textArea.select();
    try {
      document.execCommand("copy");
    } catch (err) {
      console.error("Failed to copy console output:", err);
    }
    document.body.removeChild(textArea);
  }
});

copyErrorBtn?.addEventListener("click", () => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(errorOutput.textContent).catch(err => {
      console.error("Failed to copy error output:", err);
    });
  } else {
    // Fallback for browsers that don't support Clipboard API
    const textArea = document.createElement("textarea");
    textArea.value = errorOutput.textContent;
    document.body.appendChild(textArea);
    textArea.select();
    try {
      document.execCommand("copy");
    } catch (err) {
      console.error("Failed to copy error output:", err);
    }
    document.body.removeChild(textArea);
  }
});


const init = async () => {
  try {
    const savedLang = localStorage.getItem("sandbox.lang");
    if (savedLang && LANGUAGE_CONFIG[savedLang]) {
      languageSelect.value = savedLang;
    }
    currentLanguage = languageSelect.value;
    runtimeStatus.textContent = `Ready: ${LANGUAGE_CONFIG[currentLanguage].label}`;
  } catch (err) {
    errorOutput.textContent = `Editor failed to load: ${err instanceof Error ? err.message : String(err)}`;
  }
};

init();
