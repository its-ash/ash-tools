<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Diff Checker | Compare Text Locally in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser diff checker. Compare two text blocks line by line and character by character. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/diff/' },
    { property: 'og:title', content: 'Free Diff Checker | Compare Text Locally' },
    { property: 'og:description', content: 'Compare text privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/diff/' }],
})

useToolSchema({
  name: 'Diff Checker',
  description: 'Compare two blocks of text line by line, locally in your browser.',
  url: 'https://ash-tools.store/diff/',
  category: 'DeveloperApplication',
})

const left = ref('')
const right = ref('')
const ignoreWhitespace = ref(false)
const ignoreCase = ref(false)
const status = ref('Paste text on both sides to compare.')

type LineOp = { type: 'equal' | 'add' | 'remove'; text: string; leftNo: number | null; rightNo: number | null }

const normalize = (line: string) => {
  let value = line
  if (ignoreCase.value) value = value.toLowerCase()
  if (ignoreWhitespace.value) value = value.trim().replace(/\s+/g, ' ')
  return value
}

// Myers-style LCS diff for lines (O(n*m), fine for typical text sizes).
const diffLines = (a: string[], b: string[]): LineOp[] => {
  const an = a.map(normalize)
  const bn = b.map(normalize)
  const n = a.length
  const m = b.length
  const dp: number[][] = Array.from({ length: n + 1 }, () => new Array(m + 1).fill(0))

  for (let i = n - 1; i >= 0; i -= 1) {
    for (let j = m - 1; j >= 0; j -= 1) {
      dp[i][j] = an[i] === bn[j] ? dp[i + 1][j + 1] + 1 : Math.max(dp[i + 1][j], dp[i][j + 1])
    }
  }

  const ops: LineOp[] = []
  let i = 0
  let j = 0
  while (i < n && j < m) {
    if (an[i] === bn[j]) {
      ops.push({ type: 'equal', text: a[i], leftNo: i + 1, rightNo: j + 1 })
      i += 1
      j += 1
    } else if (dp[i + 1][j] >= dp[i][j + 1]) {
      ops.push({ type: 'remove', text: a[i], leftNo: i + 1, rightNo: null })
      i += 1
    } else {
      ops.push({ type: 'add', text: b[j], leftNo: null, rightNo: j + 1 })
      j += 1
    }
  }
  while (i < n) { ops.push({ type: 'remove', text: a[i], leftNo: i + 1, rightNo: null }); i += 1 }
  while (j < m) { ops.push({ type: 'add', text: b[j], leftNo: null, rightNo: j + 1 }); j += 1 }
  return ops
}

const diffResult = computed<LineOp[]>(() => {
  if (!left.value && !right.value) return []
  return diffLines(left.value.split('\n'), right.value.split('\n'))
})

const stats = computed(() => {
  const added = diffResult.value.filter((op) => op.type === 'add').length
  const removed = diffResult.value.filter((op) => op.type === 'remove').length
  const unchanged = diffResult.value.filter((op) => op.type === 'equal').length
  return { added, removed, unchanged }
})

watch(diffResult, () => {
  if (!left.value && !right.value) {
    status.value = 'Paste text on both sides to compare.'
    return
  }
  status.value = `${stats.value.added} added · ${stats.value.removed} removed · ${stats.value.unchanged} unchanged.`
})

const swap = () => {
  const temp = left.value
  left.value = right.value
  right.value = temp
}

const clearAll = () => {
  left.value = ''
  right.value = ''
}

const onLeftFile = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => { left.value = String(reader.result || '') }
  reader.readAsText(file)
}

const onRightFile = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => { right.value = String(reader.result || '') }
  reader.readAsText(file)
}
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
          <button class="rail-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Swap sides" aria-label="Swap sides" @click="swap">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M7 4L4 7l3 3M4 7h12M13 16l3-3-3-3M16 13H4" /></svg>
            <span class="text-[10px] font-medium">Swap</span>
          </button>
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
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M6 4v9a2 2 0 002 2h6M14 16V7a2 2 0 00-2-2H6" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Diff Checker</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="ui-panel p-3 flex flex-wrap items-center gap-4">
          <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
            <input v-model="ignoreWhitespace" type="checkbox" class="ui-range"> Ignore whitespace
          </label>
          <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
            <input v-model="ignoreCase" type="checkbox" class="ui-range"> Ignore case
          </label>
          <span class="ml-auto text-xs font-mono text-[var(--ink-3)]">
            <span style="color: #16a34a">+{{ stats.added }}</span> ·
            <span style="color: #dc2626">-{{ stats.removed }}</span>
          </span>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Original</span>
              <label class="ui-button-secondary !py-1 !px-2.5 text-xs cursor-pointer">
                Load file
                <input type="file" accept="text/*" class="hidden" @change="onLeftFile">
              </label>
            </div>
            <div class="px-4 pb-4">
              <textarea v-model="left" spellcheck="false" placeholder="Paste original text..." class="ui-input font-mono text-xs w-full min-h-[180px] resize-none" />
            </div>
          </section>

          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Changed</span>
              <label class="ui-button-secondary !py-1 !px-2.5 text-xs cursor-pointer">
                Load file
                <input type="file" accept="text/*" class="hidden" @change="onRightFile">
              </label>
            </div>
            <div class="px-4 pb-4">
              <textarea v-model="right" spellcheck="false" placeholder="Paste changed text..." class="ui-input font-mono text-xs w-full min-h-[180px] resize-none" />
            </div>
          </section>
        </div>

        <section class="ui-panel flex flex-col min-h-0 flex-1">
          <div class="flex items-center justify-between px-4 pt-3 pb-2">
            <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Diff</span>
          </div>
          <div class="px-4 pb-4">
            <div class="rounded-none overflow-hidden border" style="border-color: var(--border)">
              <div v-if="!diffResult.length" class="p-4 text-sm text-[var(--ink-3)]" style="background: var(--surface-1)">No differences to show yet.</div>
              <div v-else class="max-h-[480px] overflow-auto font-mono text-xs">
                <div
                  v-for="(op, i) in diffResult"
                  :key="i"
                  class="flex gap-3 px-3 py-0.5"
                  :style="{
                    background: op.type === 'add' ? 'rgba(22,163,74,0.1)' : op.type === 'remove' ? 'rgba(220,38,38,0.1)' : 'var(--surface-1)',
                  }"
                >
                  <span class="w-8 shrink-0 text-right select-none" style="color: var(--ink-3)">{{ op.leftNo ?? '' }}</span>
                  <span class="w-8 shrink-0 text-right select-none" style="color: var(--ink-3)">{{ op.rightNo ?? '' }}</span>
                  <span class="w-3 shrink-0 select-none" :style="{ color: op.type === 'add' ? '#16a34a' : op.type === 'remove' ? '#dc2626' : 'var(--ink-3)' }">
                    {{ op.type === 'add' ? '+' : op.type === 'remove' ? '-' : ' ' }}
                  </span>
                  <span class="whitespace-pre-wrap break-all text-[var(--ink-1)]">{{ op.text || ' ' }}</span>
                </div>
              </div>
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
