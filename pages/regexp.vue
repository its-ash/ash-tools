<script setup lang="ts">
let wasmGenerate: ((lines: string[]) => string) | null = null
let wasmReadyPromise: Promise<void> | null = null

if (typeof window !== 'undefined') {
  const ensureWasmGenerate = async () => {
    if (!wasmReadyPromise) {
      wasmReadyPromise = (async () => {
        try {
          // Use a variable to completely hide the path from Vite's static analysis.
          // This forces a true runtime browser import of the public directory asset.
          const moduleUrl = '/regexp/pkg/regexp_wasm.js'
          // @ts-ignore
          const wasmModule = await import(/* @vite-ignore */ moduleUrl)
          
          // Depending on the environment, the default export may be the module or just the function
          const initWasm = wasmModule.default || wasmModule.__wbg_init
          
          if (typeof initWasm !== 'function') {
            throw new TypeError(`initWasm is not a function. wasmModule: ${JSON.stringify(wasmModule)}`)
          }
          
          let wasm
          try {
            wasm = await initWasm()
          } catch (e) {
            console.warn('regexp wasm: init threw', e)
            wasm = null
          }

          // Log module keys after initialization for debugging
          try {
            console.debug('regexp wasm module keys:', wasmModule ? Object.keys(wasmModule) : wasmModule)
            if (wasmModule?.RegExpBuilder) {
              console.debug('RegExpBuilder found:', wasmModule.RegExpBuilder)
            }
            if (typeof wasmModule?.generate_regex === 'function') {
              console.debug('generate_regex found:', wasmModule.generate_regex)
            }
          } catch (e) { /* noop */ }

          // After init, check 'wasmModule' for the exports (JS wrapper classes/functions)
          if (wasmModule?.RegExpBuilder && typeof wasmModule.RegExpBuilder.from === 'function') {
            wasmGenerate = (lines: string[]) => {
              const builder = wasmModule.RegExpBuilder.from(lines)
              try { return builder.build() }
              finally { if (typeof builder.free === 'function') builder.free() }
            }
          } else if (typeof wasmModule?.generate_regex === 'function') {
            wasmGenerate = wasmModule.generate_regex.bind(wasmModule)
          } else {
            console.warn(
              'regexp wasm: RegExpBuilder/generate_regex not found on wasmModule exports',
              wasmModule ? Object.keys(wasmModule) : wasmModule,
              '\nFull exports:', wasmModule
            )
            wasmGenerate = null
          }
        } catch (e) {
          wasmReadyPromise = null
          throw e
        }
      })()
    }
    return wasmReadyPromise
  }

  onMounted(async () => {
    try {
      await ensureWasmGenerate()

      // Set default values (match previous behavior)
      if (inputTextEl.value) {
        inputTextEl.value.value = `2024-02-15 10:00:01 [INFO] User admin@example.com logged in from 192.168.1.10\n2024-02-15 10:05:23 [WARN] Failed login attempt from test_user@sub.domain.org\n2024-02-15 10:15:00 [ERROR] Connection lost to db-prod-01 (10.0.0.5)`
      }
      if (targetTextEl.value) {
        targetTextEl.value.value = `admin@example.com\ntest_user@sub.domain.org`
      }
    } catch (e) {
      // eslint-disable-next-line no-console
      console.error('Failed to load regexp wasm module', e)
    }
  })
}

const inputTextEl = ref<HTMLTextAreaElement | null>(null)
const targetTextEl = ref<HTMLTextAreaElement | null>(null)
const regexOutputEl = ref<HTMLElement | null>(null)
const previewEl = ref<HTMLElement | null>(null)
const showResult = ref(false)


useHead({
  title: 'Regex Generator | Ash Tools',
  meta: [
    { name: 'description', content: 'Free online Regex Generator tool. Create optimized Regular Expressions from examples instantly. Powered by Rust and WebAssembly, runs fully offline in your browser.' },
    { name: 'keywords', content: 'regex generator, regular expression tool, rust regex, webassembly regex, offline regex tool, pattern matching, grex online' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:title', content: 'Regex Generator | Ash Tools' },
    { property: 'og:description', content: 'Generate optimized Regular Expressions from examples. Process text offline with Rust + WASM.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/regexp/' }],
})

const handleGenerate = async () => {
  try {
    const rawTargets = targetTextEl.value?.value ?? ''
    const lines = rawTargets.split('\n').map(line => line.trim()).filter(line => line.length > 0)

    if (lines.length === 0) {
      alert('Please enter at least one target string to match.')
      return
    }

    if (!wasmGenerate) {
      try {
        await ensureWasmGenerate()
      } catch (e) {
        throw new Error('Regex generator not loaded yet.')
      }
    }

    try {
      const gen = wasmGenerate
      if (!gen) throw new Error('Regex generator not available after load')
      const generatedRegexSrc = gen(lines)
      if (regexOutputEl.value) regexOutputEl.value.textContent = generatedRegexSrc
      showResult.value = true
      highlightMatches(generatedRegexSrc)
    } catch (e) {
      console.error(e)
      alert('Error generating regex: ' + String(e))
    }
  } catch (e) {
    console.error(e)
  }
}

function highlightMatches(regexStr: string) {
  const text = inputTextEl.value?.value ?? ''
  if (!text) {
    if (previewEl.value) previewEl.value.textContent = "No input text to preview matches against."
    return
  }

  try {
    const re = new RegExp(regexStr, 'g')
    // Build HTML with highlighted matches
    const matches = [...text.matchAll(re)]
    if (matches.length === 0) {
      if (previewEl.value) previewEl.value.textContent = text
      return
    }

    let html = ''
    let cursor = 0
    matches.forEach(m => {
      const start = m.index ?? 0
      const end = start + (m[0]?.length ?? 0)
      html += escapeHtml(text.slice(cursor, start))
      html += `<span class="match">${escapeHtml(m[0] ?? '')}</span>`
      cursor = end
    })
    html += escapeHtml(text.slice(cursor))
    if (previewEl.value) previewEl.value.innerHTML = html
  } catch (e) {
    if (previewEl.value) previewEl.value.textContent = "Invalid Regex generated (or not supported by JS engine): " + regexStr
  }
}

function escapeHtml(text: string) {
  return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;').replace(/'/g, '&#039;')
}
</script>

<template>
  <div class="min-h-[calc(100vh-48px)] bg-slate-950 text-slate-200 flex justify-center p-3.5">
    <div class="w-full max-w-6xl flex flex-col gap-3.5">

      <!-- Hero Header -->
      <header class="bg-slate-900/90 border border-white/10 rounded-xl p-4 backdrop-blur-md shadow-lg">
        <p class="text-xs uppercase tracking-widest text-slate-500 font-semibold mb-1">Local only</p>
        <h1 class="text-2xl md:text-3xl font-bold tracking-tight -mt-0.5 mb-2">Regex Generator</h1>
        <p class="text-slate-400 text-sm leading-relaxed">Generate optimized Regular Expressions from examples. Enter text and the parts you want to match.</p>
      </header>

      <!-- Input & Controls -->
      <section class="bg-slate-900/90 border border-white/10 rounded-xl p-5 backdrop-blur-md shadow-lg flex flex-col gap-4">
        
        <!-- Two-Column Input -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <label class="flex flex-col gap-2">
            <span class="text-xs uppercase tracking-widest text-slate-500 font-semibold">Input Text (Source)</span>
              <textarea id="inputText" ref="inputTextEl" placeholder="Paste your source text here..." class="w-full min-h-48 p-3 bg-white/2 border border-white/10 rounded-lg text-slate-200 font-mono text-sm resize-vertical focus:outline-none focus:border-pink-500/50 transition-colors"></textarea>
          </label>

          <label class="flex flex-col gap-2">
            <span class="text-xs uppercase tracking-widest text-slate-500 font-semibold">Desired Matches (Output)</span>
            <textarea id="targetText" ref="targetTextEl" placeholder="Paste the exact strings you want to match from the input (one per line)..." class="w-full min-h-48 p-3 bg-white/2 border border-white/10 rounded-lg text-slate-200 font-mono text-sm resize-vertical focus:outline-none focus:border-pink-500/50 transition-colors"></textarea>
          </label>
        </div>

        <!-- Generate Button -->
        <div class="flex justify-end">
          <button id="generateBtn" @click="handleGenerate" class="px-6 py-2.5 bg-gradient-to-r from-pink-500 to-rose-400 text-white rounded-lg font-bold text-sm hover:-translate-y-0.5 transition-all hover:shadow-lg">Generate Regex</button>
        </div>

        <!-- Output -->
        <div id="resultBox" v-show="showResult" class="bg-white/2 p-5 rounded-lg border border-white/10">
          <h3 class="text-sm uppercase tracking-wider font-bold text-pink-400 mb-4">Generated Regex</h3>
          <div id="regexOutput" ref="regexOutputEl" class="min-h-auto p-5 text-pink-400 font-mono text-base bg-black/20 rounded-lg border border-white/5 mb-4"></div>

          <p class="text-xs uppercase tracking-widest text-slate-500 font-semibold mb-2.5">Preview matches in Input Text:</p>
          <div id="preview" ref="previewEl" class="font-mono text-xs text-slate-300 whitespace-pre-wrap bg-black/20 p-3 rounded-lg border border-white/5 max-h-60 overflow-y-auto"></div>
        </div>
      </section>
    </div>
  </div>
</template>

