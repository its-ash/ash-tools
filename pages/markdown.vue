<script setup lang="ts">
import { marked } from 'marked'
import DOMPurify from 'dompurify'

definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Markdown Previewer | Live Markdown to HTML in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser Markdown previewer. Write Markdown and see sanitized, live-rendered HTML instantly. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/markdown/' },
    { property: 'og:title', content: 'Free Markdown Previewer | Live Markdown to HTML' },
    { property: 'og:description', content: 'Preview Markdown privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/markdown/' }],
})

useToolSchema({
  name: 'Markdown Previewer',
  description: 'Write Markdown and see sanitized, live-rendered HTML, locally in your browser.',
  url: 'https://ash-tools.store/markdown/',
  category: 'DeveloperApplication',
})

const SAMPLE = `# Ash Tools

A **local-first** toolkit for everyday file tasks.

- No uploads
- Works offline
- No accounts

## Code example

\`\`\`js
console.log('hello world')
\`\`\`

> Everything runs in your browser.

[Learn more](https://ash-tools.store)
`

const input = ref(SAMPLE)
const status = ref('Type Markdown to see a live preview.')

marked.setOptions({ gfm: true, breaks: false })

const renderedHtml = computed(() => {
  try {
    const raw = marked.parse(input.value, { async: false }) as string
    return DOMPurify.sanitize(raw, { USE_PROFILES: { html: true } })
  } catch {
    return ''
  }
})

const wordCount = computed(() => (input.value.trim() ? input.value.trim().split(/\s+/).length : 0))

const copyHtml = async () => {
  if (!renderedHtml.value) return
  try {
    await navigator.clipboard.writeText(renderedHtml.value)
    status.value = 'HTML copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const downloadHtml = () => {
  if (!renderedHtml.value) return
  const doc = `<!doctype html>\n<html><head><meta charset="utf-8"><title>Document</title></head><body>\n${renderedHtml.value}\n</body></html>`
  const blob = new Blob([doc], { type: 'text/html' })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = 'document.html'
  document.body.appendChild(anchor)
  anchor.click()
  anchor.remove()
  URL.revokeObjectURL(url)
}

const downloadMarkdown = () => {
  if (!input.value) return
  const blob = new Blob([input.value], { type: 'text/markdown' })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = 'document.md'
  document.body.appendChild(anchor)
  anchor.click()
  anchor.remove()
  URL.revokeObjectURL(url)
}

const clearAll = () => {
  input.value = ''
  status.value = 'Type Markdown to see a live preview.'
}

const onFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return
  const reader = new FileReader()
  reader.onload = () => {
    input.value = String(reader.result || '')
    status.value = `Loaded ${file.name}.`
  }
  reader.readAsText(file)
}

watch(input, () => {
  status.value = input.value.trim() ? `${wordCount.value} words.` : 'Type Markdown to see a live preview.'
})
</script>

<template>
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <div class="flex-1 flex flex-col md:flex-row">
      <aside class="order-2 md:order-1 shrink-0 border-t md:border-t-0 md:border-r md:w-[76px] w-full" style="background: var(--surface-1); border-color: var(--border)">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-4 overflow-x-auto md:overflow-visible">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
          <label for="md-file-input" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Open a file" aria-label="Open a file">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M6 3h5l3 3v11a1 1 0 01-1 1H6a1 1 0 01-1-1V4a1 1 0 011-1z" /><path stroke-linecap="round" stroke-linejoin="round" d="M11 3v3h3" /></svg>
            <span class="text-[10px] font-medium">Open</span>
          </label>
          <input id="md-file-input" type="file" accept=".md,.markdown,text/markdown,text/plain" class="hidden" @change="onFileChange">
          <button class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Clear" aria-label="Clear" @click="clearAll">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M5 6h10M8 6V4h4v2m-6 0l1 10h6l1-10" /></svg>
            <span class="text-[10px] font-medium">Clear</span>
          </button>
        </div>
      </aside>

      <main class="order-1 md:order-2 flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel p-4 flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2.5" y="4" width="15" height="12" rx="1.5" /><path stroke-linecap="round" stroke-linejoin="round" d="M5 13V7l2.5 3L10 7v6M13 7v6M12 10.5l1.5 1.5 1.5-1.5" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Markdown Previewer</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="ui-panel p-3 flex flex-wrap items-center gap-2">
          <span class="text-xs font-mono text-[var(--ink-3)]">{{ input.length }} chars</span>
          <div class="ml-auto flex items-center gap-2">
            <button class="ui-button-secondary !py-1.5 !px-3 text-xs" :disabled="!input" @click="downloadMarkdown">Download .md</button>
            <button class="ui-button-secondary !py-1.5 !px-3 text-xs" :disabled="!renderedHtml" @click="copyHtml">Copy HTML</button>
            <button class="ui-button !py-1.5 !px-3 text-xs" :disabled="!renderedHtml" @click="downloadHtml">Download HTML</button>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 flex-1 min-h-0">
          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Markdown</span>
            </div>
            <div class="px-4 pb-4 flex-1 min-h-0">
              <textarea
                v-model="input"
                spellcheck="false"
                placeholder="# Type Markdown here..."
                class="ui-input font-mono text-xs w-full h-full min-h-[360px] resize-none"
              />
            </div>
          </section>

          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Preview</span>
            </div>
            <div class="px-4 pb-4 flex-1 min-h-0">
              <div
                class="ui-input markdown-preview w-full h-full min-h-[360px] overflow-auto"
                v-html="renderedHtml || '<p style=\'color: var(--ink-3)\'>Preview will appear here.</p>'"
              />
            </div>
          </section>
        </div>
      </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-3 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="status-dot ok w-2 h-2 shrink-0 inline-block"></span>
      <span class="truncate">{{ status }}</span>
    </div>
  </div>
</template>

<style scoped>
.rail-btn {
  transition: background-color 140ms ease, color 140ms ease;
}

.rail-btn:hover {
  background: var(--surface-2);
  color: var(--ink-1);
}

.rail-btn:active {
  transform: scale(0.97);
}

@media (prefers-reduced-motion: reduce) {
  .rail-btn {
    transition: none !important;
  }
}

.markdown-preview :deep(h1),
.markdown-preview :deep(h2),
.markdown-preview :deep(h3) {
  font-weight: 700;
  margin: 0.75em 0 0.4em;
  color: var(--ink-1);
}
.markdown-preview :deep(h1) { font-size: 1.5em; }
.markdown-preview :deep(h2) { font-size: 1.25em; }
.markdown-preview :deep(h3) { font-size: 1.1em; }
.markdown-preview :deep(p) { margin: 0.5em 0; line-height: 1.6; }
.markdown-preview :deep(ul),
.markdown-preview :deep(ol) { margin: 0.5em 0; padding-left: 1.5em; }
.markdown-preview :deep(li) { margin: 0.2em 0; }
.markdown-preview :deep(a) { color: var(--accent); text-decoration: underline; }
.markdown-preview :deep(code) {
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  background: var(--surface-2);
  padding: 0.1em 0.35em;
  border: 1px solid var(--border);
  font-size: 0.9em;
}
.markdown-preview :deep(pre) {
  background: var(--surface-2);
  padding: 0.75em 1em;
  border: 1px solid var(--border);
  overflow-x: auto;
  margin: 0.6em 0;
}
.markdown-preview :deep(pre code) { border: none; }
.markdown-preview :deep(pre code) { background: none; padding: 0; }
.markdown-preview :deep(blockquote) {
  border-left: 3px solid var(--border-strong);
  padding-left: 1em;
  margin: 0.6em 0;
  color: var(--ink-2);
}
.markdown-preview :deep(table) { border-collapse: collapse; margin: 0.6em 0; }
.markdown-preview :deep(th),
.markdown-preview :deep(td) { border: 1px solid var(--border); padding: 0.4em 0.7em; }
.markdown-preview :deep(img) { max-width: 100%; }
</style>
