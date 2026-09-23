<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Text Case Converter & Lorem Ipsum Generator in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser text tool. Convert case (camelCase, snake_case, kebab-case, Title Case), count words/characters, and generate Lorem Ipsum. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/text/' },
    { property: 'og:title', content: 'Free Text Case Converter & Lorem Ipsum Generator' },
    { property: 'og:description', content: 'Convert text case and generate placeholder text privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/text/' }],
})

useToolSchema({
  name: 'Text Case Converter & Lorem Ipsum Generator',
  description: 'Convert text case, count words and characters, and generate Lorem Ipsum, locally in your browser.',
  url: 'https://ash-tools.store/text/',
  category: 'UtilitiesApplication',
})

const DEFAULT_LOREM_PARAGRAPHS = 3

const input = ref('The quick brown fox jumps over the lazy dog.')
const status = ref('Type or paste text to transform.')

const words = computed(() => (input.value.trim() ? input.value.trim().split(/\s+/) : []))
const stats = computed(() => ({
  words: words.value.length,
  chars: input.value.length,
  charsNoSpaces: input.value.replace(/\s/g, '').length,
  lines: input.value ? input.value.split('\n').length : 0,
  sentences: (input.value.match(/[.!?]+(\s|$)/g) || []).length,
}))

const toWords = (value: string) => value
  .replace(/([a-z])([A-Z])/g, '$1 $2')
  .replace(/[_-]+/g, ' ')
  .trim()
  .split(/\s+/)
  .filter(Boolean)

const caseTransforms: { label: string; fn: (v: string) => string }[] = [
  { label: 'UPPERCASE', fn: (v) => v.toUpperCase() },
  { label: 'lowercase', fn: (v) => v.toLowerCase() },
  { label: 'Title Case', fn: (v) => toWords(v).map((w) => w[0].toUpperCase() + w.slice(1).toLowerCase()).join(' ') },
  { label: 'Sentence case', fn: (v) => v.toLowerCase().replace(/(^\s*\w|[.!?]\s+\w)/g, (m) => m.toUpperCase()) },
  { label: 'camelCase', fn: (v) => toWords(v).map((w, i) => (i === 0 ? w.toLowerCase() : w[0].toUpperCase() + w.slice(1).toLowerCase())).join('') },
  { label: 'PascalCase', fn: (v) => toWords(v).map((w) => w[0].toUpperCase() + w.slice(1).toLowerCase()).join('') },
  { label: 'snake_case', fn: (v) => toWords(v).map((w) => w.toLowerCase()).join('_') },
  { label: 'kebab-case', fn: (v) => toWords(v).map((w) => w.toLowerCase()).join('-') },
  { label: 'CONSTANT_CASE', fn: (v) => toWords(v).map((w) => w.toUpperCase()).join('_') },
  { label: 'aLtErNaTiNg', fn: (v) => v.split('').map((c, i) => (i % 2 === 0 ? c.toLowerCase() : c.toUpperCase())).join('') },
]

const activeTransform = ref(caseTransforms[2].label)

const transformedText = computed(() => {
  const transform = caseTransforms.find((t) => t.label === activeTransform.value)
  return transform && input.value ? transform.fn(input.value) : ''
})

const applyTransform = () => {
  if (!transformedText.value) return
  input.value = transformedText.value
  status.value = `Applied ${activeTransform.value}.`
}

const copyTransformed = async () => {
  if (!transformedText.value) return
  try {
    await navigator.clipboard.writeText(transformedText.value)
    status.value = 'Copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const clearAll = () => {
  input.value = ''
  status.value = 'Type or paste text to transform.'
}

// --- Lorem Ipsum generator ---
const LOREM_WORDS = 'lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua enim ad minim veniam quis nostrud exercitation ullamco laboris nisi aliquip ex ea commodo consequat duis aute irure in reprehenderit voluptate velit esse cillum eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt culpa qui officia deserunt mollit anim id est laborum'.split(' ')

const loremParagraphCount = ref(DEFAULT_LOREM_PARAGRAPHS)
const loremStartClassic = ref(true)
const loremOutput = ref('')

const randomWord = () => LOREM_WORDS[Math.floor(Math.random() * LOREM_WORDS.length)]

const generateSentence = (wordCount: number) => {
  const w = Array.from({ length: wordCount }, randomWord)
  const sentence = w.join(' ')
  return sentence[0].toUpperCase() + sentence.slice(1) + '.'
}

const generateParagraph = (sentenceCount: number) => Array.from({ length: sentenceCount }, () => generateSentence(6 + Math.floor(Math.random() * 10))).join(' ')

const generateLorem = () => {
  const count = Math.min(50, Math.max(1, Math.round(loremParagraphCount.value)))
  loremParagraphCount.value = count
  const paragraphs = Array.from({ length: count }, (_, i) => {
    if (i === 0 && loremStartClassic.value) {
      return 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. ' + generateParagraph(4)
    }
    return generateParagraph(5 + Math.floor(Math.random() * 3))
  })
  loremOutput.value = paragraphs.join('\n\n')
  status.value = `Generated ${count} paragraph${count === 1 ? '' : 's'}.`
}

const copyLorem = async () => {
  if (!loremOutput.value) return
  try {
    await navigator.clipboard.writeText(loremOutput.value)
    status.value = 'Lorem Ipsum copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const useLoremAsInput = () => {
  if (!loremOutput.value) return
  input.value = loremOutput.value
  status.value = 'Loaded Lorem Ipsum into the text tool.'
}

onMounted(generateLorem)
</script>

<template>
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <div class="flex-1 flex flex-col md:flex-row">
      <aside class="order-2 md:order-1 shrink-0 border-t md:border-t-0 md:border-r md:w-[76px] w-full" style="background: var(--surface-1); border-color: var(--border)">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-4 overflow-x-auto md:overflow-visible">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
          <button class="rail-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Clear" aria-label="Clear" @click="clearAll">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M5 6h10M8 6V4h4v2m-6 0l1 10h6l1-10" /></svg>
            <span class="text-[10px] font-medium">Clear</span>
          </button>
        </div>
      </aside>

      <main class="order-1 md:order-2 flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel p-4 flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 5h12M10 5v10M7 15h6" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Text Case &amp; Lorem Ipsum</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <section class="ui-panel flex flex-col min-h-0">
          <div class="flex items-center justify-between px-4 pt-3 pb-2">
            <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Text</span>
            <span class="text-xs font-mono text-[var(--ink-3)]">
              {{ stats.words }} words · {{ stats.chars }} chars · {{ stats.sentences }} sentences
            </span>
          </div>
          <div class="px-4 pb-4 flex flex-col gap-3">
            <textarea v-model="input" spellcheck="false" placeholder="Type or paste text here..." class="ui-input font-mono text-xs w-full min-h-[140px] resize-none" />

            <div class="flex flex-wrap gap-1.5">
              <button
                v-for="t in caseTransforms"
                :key="t.label"
                class="px-2.5 py-1 text-xs font-semibold rounded-none transition-colors duration-150"
                :class="activeTransform === t.label ? 'ui-button !py-1 !px-2.5' : 'ui-button-secondary !py-1 !px-2.5'"
                @click="activeTransform = t.label"
              >
                {{ t.label }}
              </button>
            </div>

            <div class="flex items-center gap-2">
              <span class="ui-input font-mono text-xs flex-1 truncate select-all min-h-[38px] flex items-center">{{ transformedText || '—' }}</span>
              <button class="ui-button-secondary !py-2 !px-3 text-xs shrink-0" :disabled="!transformedText" @click="copyTransformed">Copy</button>
              <button class="ui-button !py-2 !px-3 text-xs shrink-0" :disabled="!transformedText" @click="applyTransform">Apply</button>
            </div>
          </div>
        </section>

        <section class="ui-panel flex flex-col min-h-0">
          <div class="flex items-center justify-between px-4 pt-3 pb-2">
            <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Lorem Ipsum generator</span>
          </div>
          <div class="px-4 pb-4 flex flex-col gap-3">
            <div class="flex flex-wrap items-center gap-3">
              <label class="flex items-center gap-1.5 text-xs text-[var(--ink-3)]">
                Paragraphs
                <input v-model.number="loremParagraphCount" type="number" min="1" max="50" class="ui-input !py-1 !px-2 !w-16 text-xs" @change="generateLorem">
              </label>
              <label class="flex items-center gap-1.5 text-xs text-[var(--ink-3)]">
                <input v-model="loremStartClassic" type="checkbox" class="ui-range" @change="generateLorem"> Start with classic opening
              </label>
              <button class="ui-button !py-1.5 !px-3 text-xs ml-auto" @click="generateLorem">Regenerate</button>
            </div>
            <pre class="ui-input font-mono text-xs w-full min-h-[160px] max-h-64 overflow-auto whitespace-pre-wrap">{{ loremOutput || 'Click regenerate to produce text.' }}</pre>
            <div class="flex gap-2">
              <button class="ui-button-secondary flex-1 !py-2 text-xs" :disabled="!loremOutput" @click="copyLorem">Copy</button>
              <button class="ui-button flex-1 !py-2 text-xs" :disabled="!loremOutput" @click="useLoremAsInput">Use as input above</button>
            </div>
          </div>
        </section>
      </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-3 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="status-dot ok w-2 h-2 rounded-full inline-block shrink-0"></span>
      <span class="truncate">{{ status }}</span>
    </div>
  </div>
</template>

<style scoped>
.rail-btn {
  transition: transform 140ms cubic-bezier(0.23, 1, 0.32, 1), background-color 140ms ease, color 140ms ease;
}

.rail-btn:hover {
  background: var(--surface-2);
  color: var(--ink-1);
}

.rail-btn:active {
  transform: scale(0.94);
}

@media (prefers-reduced-motion: reduce) {
  .rail-btn {
    transition: none !important;
  }
}
</style>
