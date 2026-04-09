<script setup lang="ts">
useHead({
  title: 'Resume Optimizer | ATS-Friendly Suggestions',
  meta: [
    {
      name: 'description',
      content:
        'Upload a resume PDF, match it to a target job description, and get ATS-focused improvement suggestions using local WebAssembly and WebLLM.',
    },
    { name: 'robots', content: 'index,follow' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/resume/' }],
})

type AtsResult = {
  ats_score: number
  keywords_total: number
  keywords_matched: number
  matched_keywords: string[]
  missing_keywords: string[]
  suggestions: string[]
}

type ResumeWasmModule = {
  default: () => Promise<void>
  analyze_resume: (resumeText: string, jobDescription: string) => AtsResult
}

type WebLlmModule = {
  prebuiltAppConfig: Record<string, unknown>
  CreateMLCEngine: (
    modelId: string,
    config: {
      appConfig: Record<string, unknown>
      initProgressCallback?: (info: { progress?: number; text?: string }) => void
    },
  ) => Promise<{
    chat: {
      completions: {
        create: (input: {
          messages: Array<{ role: 'system' | 'user' | 'assistant'; content: string }>
          temperature: number
          max_tokens: number
          stream: boolean
        }) => Promise<{
          choices?: Array<{ message?: { content?: string | Array<{ text?: string }> } }>
        }>
      }
    }
  }>
}

type PdfJsModule = {
  GlobalWorkerOptions: { workerSrc: string }
  getDocument: (input: { data: ArrayBuffer }) => {
    promise: Promise<{
      numPages: number
      getPage: (pageNo: number) => Promise<{
        getTextContent: () => Promise<{ items: Array<{ str?: string }> }>
      }>
    }>
  }
}

const MODEL_ID = 'Llama-3.2-1B-Instruct-q4f32_1-MLC'
const LOCAL_MODEL_ROOT = '/webllm/'

const resumeFile = ref<File | null>(null)
const extractedText = ref('')
const jobDescription = ref('')
const atsResult = ref<AtsResult | null>(null)
const llmSuggestions = ref('')

const enhancedScore = ref<{ final: number; breakdown: Record<string, number> } | null>(null)

const computeEnhancedScore = (wasmResult: AtsResult, text: string, hasJobDesc: boolean) => {
  const base = Math.max(0, Math.min(100, Number(wasmResult.ats_score) || 0))

  let keywordScore = 0
  if (hasJobDesc && wasmResult.keywords_total > 0) {
    keywordScore = Math.round((wasmResult.keywords_matched / wasmResult.keywords_total) * 100)
  }

  const hasEmail = /[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}/i.test(text)
  const hasPhone = /\+?\d[\d ()-]{6,}\d/.test(text)
  const hasLinked = /(linkedin\.com|github\.com|github\.io)/i.test(text)
  const contactScore = Math.min(100, (hasEmail ? 50 : 0) + (hasPhone ? 30 : 0) + (hasLinked ? 20 : 0))

  const bullets = (text.match(/(^|\n)\s*[-•*]\s+/g) || []).length
  const headings = (text.match(/(^|\n)\s*(Experience|Education|Skills|Projects|Summary)\s*[:\n]/gi) || []).length
  const longParagraphs = (text.match(/[^\n]{200,}/g) || []).length
  let formatScore = 50
  if (bullets > 3) formatScore += 25
  if (headings > 1) formatScore += 20
  formatScore -= Math.min(30, longParagraphs * 10)
  formatScore = Math.max(10, Math.min(100, Math.round(formatScore)))

  const wordCount = (text.trim().split(/\s+/).filter(Boolean).length)
  let lengthScore = 50
  if (wordCount < 150) lengthScore = 25
  else if (wordCount <= 700) lengthScore = 85
  else lengthScore = 60

  const final = Math.round(
    base * 0.5 + keywordScore * 0.2 + formatScore * 0.15 + contactScore * 0.1 + lengthScore * 0.05,
  )

  return {
    final: Math.max(0, Math.min(100, final)),
    breakdown: {
      wasm_base: base,
      keywords: keywordScore,
      format: formatScore,
      contact: contactScore,
      length: lengthScore,
    },
  }
}

const parsing = ref(false)
const analyzing = ref(false)
const status = ref('Upload a PDF resume to begin.')
const progress = ref(0)

let wasmReadyPromise: Promise<void> | null = null
let webllmReadyPromise: Promise<void> | null = null
let analyzeResumeWasm: ((resumeText: string, jobDescription: string) => AtsResult) | null = null
let llmEngine: Awaited<ReturnType<WebLlmModule['CreateMLCEngine']>> | null = null
let webllmImport: WebLlmModule | null = null

// Markdown rendering (lazy-load marked from esm.run in the browser)
let _markedLib: any = null
const llmSuggestionsHtml = ref('')
const renderMarkdown = async (md: string) => {
  if (!_markedLib) {
    const mod = (0, eval)('import("https://esm.run/marked@5.0.2")') as Promise<any>
    const loaded = await mod
    _markedLib = loaded.marked || loaded.default || loaded
    if (typeof _markedLib.setOptions === 'function') {
      _markedLib.setOptions({ gfm: true, breaks: true })
    }
  }

  try {
    return _markedLib.parse(md || '')
  } catch (e) {
    return (md || '').replace(/</g, '&lt;').replace(/\n/g, '<br/>')
  }
}

const downloadSuggestions = () => {
  const content = llmSuggestions.value || ''
  const blob = new Blob([content], { type: 'text/markdown;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  const name = resumeFile.value ? `${resumeFile.value.name.replace(/\.pdf$/i, '')}-suggestions.md` : 'resume-suggestions.md'
  a.download = name
  document.body.appendChild(a)
  a.click()
  a.remove()
  URL.revokeObjectURL(url)
}

const showRaw = ref(false)
const copySuggestions = async () => {
  try {
    await navigator.clipboard.writeText(llmSuggestions.value || '')
    status.value = 'Suggestions copied to clipboard.'
    setTimeout(() => (status.value = ''), 2500)
  } catch (e) {
    status.value = 'Failed to copy suggestions.'
  }
}

const onResumeChange = (event: Event) => {
  const input = event.target as HTMLInputElement
  resumeFile.value = input.files?.[0] || null
  extractedText.value = ''
  atsResult.value = null
  llmSuggestions.value = ''
  status.value = resumeFile.value ? `Selected ${resumeFile.value.name}` : 'Upload a PDF resume to begin.'
}

const setProgress = (value: number) => {
  progress.value = Math.max(0, Math.min(100, Math.round(value)))
}

const extractTextFromCompletion = (
  completion: Awaited<ReturnType<Awaited<ReturnType<WebLlmModule['CreateMLCEngine']>>['chat']['completions']['create']>>,
) => {
  const choice = completion?.choices?.[0]
  const content = choice?.message?.content

  if (Array.isArray(content)) {
    return content.map((part) => part?.text || '').join('').trim()
  }

  return typeof content === 'string' ? content.trim() : ''
}

const patchModelFetch = () => {
  const globalRef = window as unknown as { __ash_resume_fetch_patched?: boolean }
  if (globalRef.__ash_resume_fetch_patched) return

  const originalFetch = globalThis.fetch?.bind(globalThis)
  if (!originalFetch) return

  globalThis.fetch = async (resource: RequestInfo | URL, init?: RequestInit) => {
    const urlString = typeof resource === 'string' ? resource : resource instanceof URL ? resource.href : resource.url
    const match =
      typeof urlString === 'string'
        ? urlString.match(/huggingface\.co\/mlc-ai\/Llama-3\.2-1B-Instruct-q4f32_1-MLC\/resolve\/main\/(.+)$/)
        : null

    if (match) {
      const artifactPath = match[1]
      const localUrl = `${LOCAL_MODEL_ROOT}Llama-3.2-1B-Instruct-q4f32_1-MLC/${artifactPath}`
      return originalFetch(localUrl, init)
    }

    return originalFetch(resource, init)
  }

  globalRef.__ash_resume_fetch_patched = true
}

const ensureWasm = async () => {
  if (!wasmReadyPromise) {
    wasmReadyPromise = ((0, eval)('import("/resume/pkg/resume_tools.js")') as Promise<ResumeWasmModule>)
      .then(async (mod) => {
        await mod.default()
        analyzeResumeWasm = mod.analyze_resume
      })
      .catch((error) => {
        wasmReadyPromise = null
        throw error
      })
  }

  await wasmReadyPromise
}

const ensureWebLlm = async () => {
  if (!webllmReadyPromise) {
    webllmReadyPromise = ((0, eval)(
      'import("https://esm.run/@mlc-ai/web-llm")',
    ) as Promise<WebLlmModule>)
      .then(async (mod) => {
        webllmImport = mod
        patchModelFetch()

        llmEngine = await mod.CreateMLCEngine(MODEL_ID, {
          appConfig: mod.prebuiltAppConfig,
          initProgressCallback: (info) => {
            const pct = Math.max(0, Math.min(1, info?.progress ?? 0))
            setProgress(pct * 100)
            if (info?.text) {
              status.value = info.text
            }
          },
        })
      })
      .catch((error) => {
        webllmReadyPromise = null
        throw error
      })
  }

  await webllmReadyPromise
}

const parsePdfToText = async (file: File) => {
  const pdfjs = ((0, eval)(
    'import("https://esm.sh/pdfjs-dist@4.7.76/build/pdf.mjs")',
  ) as Promise<PdfJsModule>)

  const pdf = await pdfjs
  pdf.GlobalWorkerOptions.workerSrc = 'https://esm.sh/pdfjs-dist@4.7.76/build/pdf.worker.mjs'

  const bytes = await file.arrayBuffer()
  const doc = await pdf.getDocument({ data: bytes }).promise

  const chunks: string[] = []
  for (let pageNo = 1; pageNo <= doc.numPages; pageNo += 1) {
    const page = await doc.getPage(pageNo)
    const text = await page.getTextContent()
    const line = text.items.map((item) => item.str || '').join(' ')
    chunks.push(line)
  }

  return chunks.join('\n').replace(/\s+/g, ' ').trim()
}

const runOptimization = async () => {
  if (!resumeFile.value) {
    status.value = 'Please select a PDF resume first.'
    return
  }

  parsing.value = true
  analyzing.value = true
  setProgress(5)
  status.value = 'Loading WebAssembly and model...'

  try {
    await ensureWasm()
    await ensureWebLlm()

    if (!analyzeResumeWasm || !llmEngine || !webllmImport) {
      throw new Error('Runtime modules are not ready')
    }

    status.value = 'Parsing PDF...'
    setProgress(20)
    const resumeText = await parsePdfToText(resumeFile.value)
    extractedText.value = resumeText

    status.value = 'Running ATS analysis with Rust WASM...'
    setProgress(45)
    const hasJobDesc = Boolean(jobDescription.value && jobDescription.value.trim())
    const wasmJobDesc = hasJobDesc ? jobDescription.value : ''
    const result = analyzeResumeWasm(resumeText, wasmJobDesc)
    // If no job description provided, clear keyword matching results
    if (!hasJobDesc) {
      result.keywords_total = 0
      result.keywords_matched = 0
      result.matched_keywords = []
      result.missing_keywords = []
    }
    atsResult.value = result
    // compute enhanced score combining wasm result with heuristics
    enhancedScore.value = computeEnhancedScore(result, resumeText, Boolean(jobDescription.value && jobDescription.value.trim()))

    status.value = 'Generating rewrite suggestions with WebLLM...'
    setProgress(70)

    const prompt = [
      'You are an ATS resume expert. Your task is to analyze the resume and job description for ATS optimization. Be specific and actionable.',
      'Return markdown with clear sections:',
      '1) Key Gaps (missing skills, keywords, or experience)',
      '2) Bullet Rewrite Suggestions (improve resume for ATS, use relevant keywords)',
      '3) ATS Optimization Checklist (short, actionable items)',
      '',
      `ATS score: ${result.ats_score}`,
      ...(hasJobDesc ? [`Matched keywords: ${(result.matched_keywords || []).join(', ')}`, `Missing keywords: ${(result.missing_keywords || []).join(', ')}`] : []),
      '',
      (hasJobDesc ? 'Job description:' : 'Job description: (none provided - skipping keyword matching)'),
      (hasJobDesc ? jobDescription.value.slice(0, 6000) : ''),
      '',
      'Resume text:',
      resumeText.slice(0, 9000),
    ].filter(Boolean).join('\n')

    const completion = await llmEngine.chat.completions.create({
      messages: [{ role: 'user', content: prompt }],
      temperature: 0.4,
      max_tokens: 700,
      stream: false,
    })

    llmSuggestions.value = extractTextFromCompletion(completion) || 'No suggestion generated.'
    // Render markdown to HTML for display
    try {
      llmSuggestionsHtml.value = await renderMarkdown(llmSuggestions.value)
    } catch (e) {
      llmSuggestionsHtml.value = llmSuggestions.value
    }
    setProgress(100)
    status.value = 'Done. Review ATS score and suggested improvements.'
  } catch (error) {
    console.error(error)
    status.value = `Error: ${error instanceof Error ? error.message : String(error)}`
  } finally {
    parsing.value = false
    analyzing.value = false
  }
}

</script>

<template>
  <div class="min-h-[calc(100vh-48px)] bg-[#FFFDF5] text-black p-6">
    <main class="mx-auto max-w-6xl grid grid-cols-1 lg:grid-cols-2 gap-6">
      <section class="neo-shell bg-white p-6 space-y-4">
        <h1 class="text-3xl font-bold tracking-tight">Resume Optimizer</h1>
        <p class="text-sm text-black">
          Upload your PDF resume and compare it with a job description. Rust WebAssembly computes ATS metrics, and the same WebLLM model suggests improvements.
        </p>

        <label for="resume-input" class="block text-xs uppercase tracking-wider text-black font-semibold">Resume PDF</label>
        <input id="resume-input" type="file" accept="application/pdf" class="block w-full text-sm text-black" @change="onResumeChange">

        <label for="job-desc" class="block text-xs uppercase tracking-wider text-black font-semibold">Target Job Description <span class="text-xs text-black">(optional)</span></label>
        <textarea
          id="job-desc"
          v-model="jobDescription"
          class="w-full min-h-56 border-4 border-black bg-white text-black text-sm p-3 outline-none"
          placeholder="Paste job description here (optional)..."
        />

        <button
          class="neo-button w-full bg-[#FF6B6B] text-black disabled:opacity-50"
          :disabled="analyzing"
          @click="runOptimization"
        >
          {{ analyzing ? 'Analyzing...' : 'Analyze Resume' }}
        </button>

        <div class="text-xs text-black">{{ status }}</div>
        <div class="h-2 rounded-full border-2 border-black bg-white overflow-hidden">
          <div class="h-full bg-linear-to-r from-cyan-400 to-teal-400 transition-all" :style="{ width: `${progress}%` }" />
        </div>
      </section>

      <section class="neo-shell bg-white p-6 space-y-4">
        <h2 class="text-xl font-semibold">Results</h2>

        <div v-if="atsResult" class="space-y-3 text-sm">
          <div class="rounded-xl border-4 border-black bg-[#FFD93D] p-3">
              <div class="text-xs text-black">Enhanced ATS Score</div>
              <div class="text-3xl font-bold text-emerald-300">{{ enhancedScore ? enhancedScore.final : '-' }}/100</div>
              <div class="text-xs text-black mt-1">Breakdown: Keywords {{ enhancedScore?.breakdown.keywords }} • Format {{ enhancedScore?.breakdown.format }} • Contact {{ enhancedScore?.breakdown.contact }} • Length {{ enhancedScore?.breakdown.length }}</div>
          </div>

          <div class="rounded-xl border-4 border-black bg-white p-3">
            <div class="text-xs uppercase tracking-wider text-black mb-2">Missing Keywords</div>
            <div class="flex flex-wrap gap-2">
              <span
                v-for="keyword in atsResult.missing_keywords"
                :key="keyword"
                class="px-2 py-1 text-xs rounded-full bg-red-500/20 border border-red-400/30 text-red-200"
              >
                {{ keyword }}
              </span>
            </div>
          </div>

          <div class="rounded-xl border-4 border-black bg-white p-3">
            <div class="text-xs uppercase tracking-wider text-black mb-2">Rust WASM Suggestions</div>
            <ul class="list-disc pl-5 space-y-1 text-black">
              <li v-for="item in atsResult.suggestions" :key="item">{{ item }}</li>
            </ul>
          </div>
        </div>

        <!-- LLM suggestions moved to full-width panel below -->
        <div class="text-xs text-black">LLM suggestions are shown in the full-width Suggestions panel below.</div>
      </section>
    </main>

    <!-- Full-width Suggestions Panel -->
    <section class="mx-auto max-w-6xl mt-6 neo-shell bg-white p-6">
      <div class="flex items-start justify-between gap-3 mb-4">
        <div>
          <h3 class="text-lg font-semibold">LLM Rewrite Suggestions</h3>
          <div class="text-sm text-black">AI-generated, ATS-focused rewrite suggestions and checklist.</div>
        </div>
        <div class="flex items-center gap-2">
          <button @click="downloadSuggestions" :disabled="!llmSuggestions" class="px-3 py-1 bg-[#FFD93D] border-2 border-black rounded text-sm disabled:opacity-40">Download .md</button>
          <button @click="copySuggestions" :disabled="!llmSuggestions" class="px-3 py-1 bg-[#C4B5FD] border-2 border-black rounded text-sm disabled:opacity-40">Copy</button>
          <button @click="showRaw = !showRaw" :disabled="!llmSuggestions" class="px-3 py-1 bg-[#FF6B6B] border-2 border-black rounded text-sm disabled:opacity-40">{{ showRaw ? 'Formatted' : 'Raw' }}</button>
        </div>
      </div>

      <div v-if="!llmSuggestions" class="text-sm text-black">Run analysis to generate personalized rewrite suggestions.</div>

      <div v-else>
        <div v-show="!showRaw" v-html="llmSuggestionsHtml" class="text-sm leading-relaxed max-w-none [&_ul]:list-disc [&_ul]:pl-5 [&_ul]:mt-2 [&_li]:mb-2 [&_h2]:text-lg [&_h3]:text-base [&_strong]:font-semibold" />
        <pre v-show="showRaw" class="whitespace-pre-wrap text-sm bg-[#FFFDF5] border-4 border-black p-4 rounded mt-2 text-black">{{ llmSuggestions }}</pre>
      </div>
    </section>
  </div>
</template>

