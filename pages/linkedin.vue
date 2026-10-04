<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free LinkedIn Post Formatter | Bold, Italic, Bullets & More in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser LinkedIn post formatter. Write with bold, italic, strikethrough, underline, monospace, bullets, numbered lists, and dividers using Unicode characters LinkedIn actually renders. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/linkedin/' },
    { property: 'og:title', content: 'Free LinkedIn Post Formatter | Bold, Italic, Bullets & More' },
    { property: 'og:description', content: 'Format LinkedIn posts privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/linkedin/' }],
})

useToolSchema({
  name: 'LinkedIn Post Formatter',
  description: 'Format LinkedIn posts with bold, italic, bullets, and more Unicode styling, locally in your browser.',
  url: 'https://ash-tools.store/linkedin/',
  category: 'UtilitiesApplication',
})

const LINKEDIN_SAFE_LIMIT = 3000
const SAMPLE = `Excited to share something I've been building. 🚀

**The problem:** most tools ask you to upload files to a server.

*The idea:* what if everything just ran locally, in your browser?

~~Cloud-only~~ Local-first. That's the shift.

Key takeaways:
- No uploads, no accounts
- Works offline once loaded
- Your data never leaves your device

---

1. Pick a tool
2. Drop your file
3. Done — no waiting on a server

Would love to hear your thoughts. \`#buildinpublic\``

const input = ref(SAMPLE)
const status = ref('Write your post using **bold**, *italic*, and more.')

// ==================== On-device AI (Chrome Gemini Nano) ====================
// Chrome's built-in AI has shipped under two shapes: the older origin-trial
// `window.ai.languageModel` and the newer `window.LanguageModel` (Chrome
// ~138+). Support both, and treat neither being present as "unavailable"
// rather than an error — most visitors won't have this enabled.
type NanoAvailability = 'unavailable' | 'downloadable' | 'downloading' | 'available'

const getLanguageModelApi = (): any | null => {
  const w = window as any
  if (w.LanguageModel) return w.LanguageModel
  if (w.ai?.languageModel) return w.ai.languageModel
  return null
}

const aiState = ref<'checking' | NanoAvailability>('checking')
const aiFormatting = ref(false)
const aiDownloadProgress = ref(0)

const checkAiAvailability = async () => {
  const api = getLanguageModelApi()
  if (!api) {
    aiState.value = 'unavailable'
    return
  }
  try {
    const availability = typeof api.availability === 'function'
      ? await api.availability()
      : await api.capabilities?.().then((c: any) => c.available)
    aiState.value = (availability as NanoAvailability) || 'unavailable'
  } catch {
    aiState.value = 'unavailable'
  }
}

const AI_PROMPT_PREFIX = `You are formatting a post for LinkedIn. Rewrite the text below into a punchy, well-structured LinkedIn post: a strong one-line hook first, short paragraphs (1-3 sentences), natural line breaks between ideas, and a bullet list (using "- ") if there are multiple points worth listing. Keep the author's original meaning, facts, and tone. Use **bold** around the 1-3 most important phrases. Do not add hashtags unless the original text already had them. Reply with only the rewritten post text, no preamble or explanation.

Text to reformat:
"""
`

const autoFormatWithAI = async () => {
  const api = getLanguageModelApi()
  if (!api || !input.value.trim()) return

  aiFormatting.value = true
  status.value = 'Formatting with on-device AI...'
  try {
    if (aiState.value === 'downloadable' || aiState.value === 'downloading') {
      status.value = 'Downloading on-device model (first use only)...'
    }
    const session = await api.create({
      monitor(m: any) {
        m.addEventListener?.('downloadprogress', (e: any) => {
          aiDownloadProgress.value = Math.round((e.loaded || 0) * 100)
          status.value = `Downloading on-device model... ${aiDownloadProgress.value}%`
        })
      },
    })
    const prompt = `${AI_PROMPT_PREFIX}${input.value.trim()}\n"""`
    const result: string = await session.prompt(prompt)
    session.destroy?.()
    input.value = result.trim()
    status.value = 'Formatted with on-device AI. Review before posting.'
    aiState.value = 'available'
  } catch (error) {
    status.value = error instanceof Error ? `AI formatting failed: ${error.message}` : 'AI formatting failed.'
  } finally {
    aiFormatting.value = false
  }
}

onMounted(checkAiAvailability)

// Unicode "Mathematical Alphanumeric Symbols" blocks that render as styled
// text anywhere plain Unicode is accepted (LinkedIn, X, etc.) since there is
// no real bold/italic markup outside a small set of platforms.
const BOLD_MAP: Record<string, number> = {}
const ITALIC_MAP: Record<string, number> = {}
const BOLD_ITALIC_MAP: Record<string, number> = {}
const MONOSPACE_MAP: Record<string, number> = {}
const SANS_MAP: Record<string, number> = {}

const fill = (map: Record<string, number>, base: number, chars: string) => {
  for (let i = 0; i < chars.length; i += 1) map[chars[i]] = base + i
}

const UPPER = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
const LOWER = 'abcdefghijklmnopqrstuvwxyz'
const DIGITS = '0123456789'

fill(BOLD_MAP, 0x1d400, UPPER)
fill(BOLD_MAP, 0x1d41a, LOWER)
fill(BOLD_MAP, 0x1d7ce, DIGITS)

fill(ITALIC_MAP, 0x1d434, UPPER)
fill(ITALIC_MAP, 0x1d44e, LOWER)
// Italic digits don't exist in the math-alphanumeric block; digits pass through unchanged.

fill(BOLD_ITALIC_MAP, 0x1d468, UPPER)
fill(BOLD_ITALIC_MAP, 0x1d482, LOWER)
fill(BOLD_ITALIC_MAP, 0x1d7ce, DIGITS)

fill(MONOSPACE_MAP, 0x1d670, UPPER)
fill(MONOSPACE_MAP, 0x1d68a, LOWER)
fill(MONOSPACE_MAP, 0x1d7f6, DIGITS)

// Sans-serif bold, used for the "all caps header" helper so headers stand out
// without relying on real caps (which some readers find shouty).
fill(SANS_MAP, 0x1d5d4, UPPER)
fill(SANS_MAP, 0x1d5ee, LOWER)
fill(SANS_MAP, 0x1d7ec, DIGITS)

// Small-caps block borrows from the Unicode Phonetic Extensions / Latin
// Extended ranges — irregular codepoints, so a lookup table rather than a
// contiguous range fill.
const SMALL_CAPS_MAP: Record<string, string> = {
  a: 'ᴀ', b: 'ʙ', c: 'ᴄ', d: 'ᴅ', e: 'ᴇ', f: 'ꜰ', g: 'ɢ', h: 'ʜ', i: 'ɪ', j: 'ᴊ',
  k: 'ᴋ', l: 'ʟ', m: 'ᴍ', n: 'ɴ', o: 'ᴏ', p: 'ᴘ', q: 'ǫ', r: 'ʀ', s: 'ꜱ', t: 'ᴛ',
  u: 'ᴜ', v: 'ᴠ', w: 'ᴡ', x: 'x', y: 'ʏ', z: 'ᴢ',
}

const mapChar = (char: string, map: Record<string, number>) => {
  const code = map[char]
  return code !== undefined ? String.fromCodePoint(code) : char
}

const applyUnicodeStyle = (text: string, map: Record<string, number>) =>
  Array.from(text).map((c) => mapChar(c, map)).join('')

const applySmallCaps = (text: string) =>
  Array.from(text).map((c) => SMALL_CAPS_MAP[c.toLowerCase()] || c).join('')

// Underline and strikethrough use combining diacritical marks placed after
// every character, which is the only way to fake either effect in plain
// Unicode text (there is no dedicated underlined/struck letter block).
const applyCombining = (text: string, combiningChar: string) =>
  Array.from(text).map((c) => (c === '\n' ? c : c + combiningChar)).join('')

const NUMERIC_BULLETS: Record<number, string> = {
  1: '①', 2: '②', 3: '③', 4: '④', 5: '⑤', 6: '⑥', 7: '⑦', 8: '⑧', 9: '⑨', 10: '⑩',
}

// Convert markdown-style markers to styled Unicode. Order matters: longer /
// more specific markers must be matched before their shorter substrings
// (e.g. bold-italic before bold, checklists before plain bullets).
const formatPost = (text: string): string => {
  let result = text

  // Horizontal divider: a line containing only three or more dashes.
  result = result.replace(/^-{3,}\s*$/gm, '───────────────')

  // Checklist: "[ ] item" / "[x] item" before generic bullets.
  result = result.replace(/^[ \t]*\[ \][ \t]+/gm, '☐ ')
  result = result.replace(/^[ \t]*\[[xX]\][ \t]+/gm, '☑ ')

  // Numbered list: "1. item" -> circled digit, falls back to "N." past 10.
  result = result.replace(/^[ \t]*(\d{1,2})\.[ \t]+/gm, (_, num: string) => {
    const n = parseInt(num, 10)
    return `${NUMERIC_BULLETS[n] || `${n}.`} `
  })

  // Inline code: `text` -> monospace Unicode, evaluated before bold/italic
  // so asterisks inside code spans aren't misread as emphasis.
  result = result.replace(/`([^`\n]+)`/g, (_, inner: string) => applyUnicodeStyle(inner, MONOSPACE_MAP))

  result = result.replace(/\*\*\*(.+?)\*\*\*/gs, (_, inner: string) => applyUnicodeStyle(inner, BOLD_ITALIC_MAP))
  result = result.replace(/\*\*(.+?)\*\*/gs, (_, inner: string) => applyUnicodeStyle(inner, BOLD_MAP))
  result = result.replace(/(?<!\*)\*(?!\*)(.+?)(?<!\*)\*(?!\*)/gs, (_, inner: string) => applyUnicodeStyle(inner, ITALIC_MAP))
  result = result.replace(/~~(.+?)~~/gs, (_, inner: string) => applyCombining(inner, '̶'))
  result = result.replace(/__(.+?)__/gs, (_, inner: string) => applyCombining(inner, '̲'))

  // Plain bullets last, so "- [ ]" style lines were already consumed above.
  result = result.replace(/^[ \t]*[-*][ \t]+/gm, '• ')

  return result
}

const formattedText = computed(() => (input.value ? formatPost(input.value) : ''))

const charCount = computed(() => Array.from(formattedText.value).length)
const overLimit = computed(() => charCount.value > LINKEDIN_SAFE_LIMIT)
const lineCount = computed(() => (input.value ? input.value.split('\n').length : 0))

const getTextarea = () => document.getElementById('post-input') as HTMLTextAreaElement | null

const wrapSelection = (marker: string, label: string) => {
  const textarea = getTextarea()
  if (!textarea) return
  const { selectionStart, selectionEnd, value } = textarea
  if (selectionStart === selectionEnd) {
    status.value = `Select text first, then click ${label}.`
    return
  }
  const selected = value.slice(selectionStart, selectionEnd)
  const before = value.slice(0, selectionStart)
  const after = value.slice(selectionEnd)
  input.value = `${before}${marker}${selected}${marker}${after}`
  nextTick(() => {
    textarea.focus()
    const cursor = selectionStart + marker.length + selected.length + marker.length
    textarea.setSelectionRange(cursor, cursor)
  })
}

const insertAtCursor = (block: string) => {
  const textarea = getTextarea()
  if (!textarea) return
  const { selectionStart, value } = textarea
  const before = value.slice(0, selectionStart)
  const after = value.slice(selectionStart)
  const needsNewline = before.length && !before.endsWith('\n')
  const insertion = `${needsNewline ? '\n' : ''}${block}`
  input.value = before + insertion + after
  nextTick(() => textarea.focus())
}

const insertBullets = () => insertAtCursor('- Point one\n- Point two\n- Point three\n')
const insertNumbered = () => insertAtCursor('1. First step\n2. Second step\n3. Third step\n')
const insertChecklist = () => insertAtCursor('[ ] To do\n[x] Done\n')
const insertDivider = () => insertAtCursor('---\n')

const applySmallCapsToSelection = () => {
  const textarea = getTextarea()
  if (!textarea) return
  const { selectionStart, selectionEnd, value } = textarea
  if (selectionStart === selectionEnd) {
    status.value = 'Select text first, then click Small Caps.'
    return
  }
  const selected = value.slice(selectionStart, selectionEnd)
  const before = value.slice(0, selectionStart)
  const after = value.slice(selectionEnd)
  const styled = applySmallCaps(selected)
  input.value = before + styled + after
  nextTick(() => {
    textarea.focus()
    const cursor = selectionStart + styled.length
    textarea.setSelectionRange(cursor, cursor)
  })
}

type ToolBtn = { label: string; title: string; action: () => void; className?: string; kbd?: string }

const styleButtons: ToolBtn[] = [
  { label: 'B', title: 'Bold', kbd: '⌘B', action: () => wrapSelection('**', 'Bold'), className: 'font-extrabold' },
  { label: 'I', title: 'Italic', kbd: '⌘I', action: () => wrapSelection('*', 'Italic'), className: 'italic font-serif' },
  { label: 'BI', title: 'Bold + Italic', action: () => wrapSelection('***', 'Bold + Italic'), className: 'font-extrabold italic' },
  { label: 'S', title: 'Strikethrough', action: () => wrapSelection('~~', 'Strikethrough'), className: 'line-through' },
  { label: 'U', title: 'Underline', kbd: '⌘U', action: () => wrapSelection('__', 'Underline'), className: 'underline underline-offset-2' },
  { label: '</>', title: 'Monospace', kbd: '⌘E', action: () => wrapSelection('`', 'Monospace'), className: 'font-mono' },
  { label: 'ᴀᴀ', title: 'Small caps', action: applySmallCapsToSelection },
]

const blockButtons: ToolBtn[] = [
  { label: '•  List', title: 'Insert bullet list', action: insertBullets },
  { label: '1.  List', title: 'Insert numbered list', action: insertNumbered },
  { label: '☑  Checklist', title: 'Insert checklist', action: insertChecklist },
  { label: '—  Divider', title: 'Insert divider', action: insertDivider },
]

const REFERENCE = [
  { md: '**bold**', out: '𝐁𝐨𝐥𝐝' },
  { md: '*italic*', out: '𝐼𝑡𝑎𝑙𝑖𝑐' },
  { md: '***both***', out: '𝑩𝒐𝒕𝒉' },
  { md: '~~strike~~', out: 'S̶t̶r̶i̶k̶e̶' },
  { md: '__underline__', out: 'U̲n̲d̲e̲r̲l̲i̲n̲e̲' },
  { md: '`code`', out: '𝚌𝚘𝚍𝚎' },
  { md: '- item', out: '• item' },
  { md: '1. item', out: '① item' },
  { md: '[ ] / [x]', out: '☐ / ☑' },
  { md: '---', out: '───────' },
]

const SHORTCUTS: Record<string, () => void> = {
  b: styleButtons[0].action,
  i: styleButtons[1].action,
  u: styleButtons[4].action,
  e: styleButtons[5].action,
}

const onEditorKeydown = (e: KeyboardEvent) => {
  if (!(e.metaKey || e.ctrlKey) || e.altKey || e.shiftKey) return
  const fn = SHORTCUTS[e.key.toLowerCase()]
  if (!fn) return
  e.preventDefault()
  fn()
}

// LinkedIn collapses the feed post behind "…see more" after roughly 210
// characters or 3 lines — whichever comes first. Preview the same fold so the
// hook can be tuned.
const FOLD_CHARS = 210
const FOLD_LINES = 3
const expanded = ref(false)
const foldIndex = computed(() => {
  const chars = Array.from(formattedText.value)
  let lines = 0
  for (let i = 0; i < chars.length; i += 1) {
    if (chars[i] === '\n' && ++lines >= FOLD_LINES) return i
    if (i >= FOLD_CHARS) return i
  }
  return -1
})
const visibleText = computed(() => {
  if (expanded.value || foldIndex.value < 0) return formattedText.value
  return Array.from(formattedText.value).slice(0, foldIndex.value).join('').trimEnd()
})
const hookText = computed(() =>
  foldIndex.value < 0 ? formattedText.value : Array.from(formattedText.value).slice(0, foldIndex.value).join(''))

const meterPct = computed(() => Math.min(100, (charCount.value / LINKEDIN_SAFE_LIMIT) * 100))
const wordCount = computed(() => (input.value.trim() ? input.value.trim().split(/\s+/).length : 0))
const readSecs = computed(() => Math.max(1, Math.round((wordCount.value / 230) * 60)))

const copied = ref(false)
let copiedTimer: ReturnType<typeof setTimeout> | undefined

const copyFormatted = async () => {
  if (!formattedText.value) return
  try {
    await navigator.clipboard.writeText(formattedText.value)
    status.value = 'Copied — paste directly into LinkedIn.'
    copied.value = true
    clearTimeout(copiedTimer)
    copiedTimer = setTimeout(() => { copied.value = false }, 1800)
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

onBeforeUnmount(() => clearTimeout(copiedTimer))

const clearAll = () => {
  input.value = ''
  status.value = 'Write your post using **bold**, *italic*, and more.'
  nextTick(() => getTextarea()?.focus())
}

const loadSample = () => {
  input.value = SAMPLE
}

watch(charCount, () => {
  if (!input.value) { status.value = 'Write your post using **bold**, *italic*, and more.'; return }
  status.value = overLimit.value
    ? `${charCount.value} characters — over LinkedIn's ~${LINKEDIN_SAFE_LIMIT} limit.`
    : `${charCount.value} characters.`
})
</script>

<template>
  <div class="min-h-screen bg-[var(--surface-2)] text-[var(--ink-1)] flex flex-col">
    <div class="flex-1 flex flex-col md:flex-row">
      <aside class="order-2 md:order-1 shrink-0 border-t md:border-t-0 md:border-r md:w-[76px] w-full" style="background: var(--surface-1); border-color: var(--border)">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-4 overflow-x-auto md:overflow-visible">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
          <button class="rail-btn shrink-0 w-14 h-14 flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Load sample" aria-label="Load sample" @click="loadSample">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M5 3h7l3 3v11H5zM12 3v3h3M8 10h4M8 13h4" /></svg>
            <span class="text-[10px] font-medium">Sample</span>
          </button>
          <button class="rail-btn shrink-0 w-14 h-14 flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Clear" aria-label="Clear" @click="clearAll">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M5 6h10M8 6V4h4v2m-6 0l1 10h6l1-10" /></svg>
            <span class="text-[10px] font-medium">Clear</span>
          </button>
        </div>
      </aside>

      <main class="order-1 md:order-2 flex-1 min-w-0 p-4 md:p-8">
        <div class="mx-auto w-full max-w-6xl flex flex-col gap-6">
          <header class="flex flex-wrap items-end justify-between gap-4">
            <div>
              <p class="text-[11px] font-mono uppercase tracking-[0.14em] text-[var(--ink-3)]">Writing · Social</p>
              <h1 class="mt-1 text-3xl md:text-4xl font-extrabold leading-[1.05] tracking-[-0.03em]">LinkedIn Post Formatter</h1>
              <p class="mt-2 text-sm text-[var(--ink-2)] max-w-xl">Write in Markdown, get Unicode styling LinkedIn actually renders. Runs entirely in your browser.</p>
            </div>
            <button class="ui-button copy-btn min-w-[9.5rem]" :disabled="!formattedText" @click="copyFormatted">
              <svg v-if="!copied" class="h-4 w-4" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.7"><rect x="7" y="7" width="10" height="10" /><path d="M13 7V3H3v10h4" /></svg>
              <svg v-else class="h-4 w-4" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M4 10.5l4 4 8-9" /></svg>
              <span>{{ copied ? 'Copied' : 'Copy post' }}</span>
            </button>
          </header>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 lg:gap-6 items-start">
            <!-- Editor -->
            <section class="ui-panel flex flex-col min-w-0" aria-labelledby="editor-label">
              <div class="flex flex-wrap items-center gap-1 border-b p-2" role="toolbar" aria-label="Formatting">
                <button
                  v-for="btn in styleButtons"
                  :key="btn.title"
                  class="tool-btn h-9 min-w-9 px-2 text-sm"
                  :class="btn.className"
                  :title="btn.kbd ? `${btn.title} (${btn.kbd})` : btn.title"
                  :aria-label="btn.title"
                  @mousedown.prevent
                  @click="btn.action"
                >{{ btn.label }}</button>
                <span class="mx-1 h-5 w-px bg-[var(--ink-3)] opacity-40" aria-hidden="true" />
                <button
                  v-for="btn in blockButtons"
                  :key="btn.title"
                  class="tool-btn h-9 px-2.5 text-xs font-semibold whitespace-pre"
                  :title="btn.title"
                  @mousedown.prevent
                  @click="btn.action"
                >{{ btn.label }}</button>
              </div>

              <label id="editor-label" for="post-input" class="sr-only">Post text</label>
              <textarea
                id="post-input"
                v-model="input"
                spellcheck="true"
                placeholder="Start with a one-line hook…"
                class="block w-full min-h-[440px] resize-y bg-transparent px-5 py-4 font-mono text-[13px] leading-[1.7] text-[var(--ink-1)] outline-none placeholder:text-[var(--ink-3)]"
                @keydown="onEditorKeydown"
              />

              <div class="flex flex-wrap items-center gap-x-4 gap-y-2 border-t px-4 py-2.5 text-xs font-mono text-[var(--ink-3)]">
                <span>{{ wordCount }} words</span>
                <span>{{ lineCount }} lines</span>
                <span>~{{ readSecs }}s read</span>
                <button
                  class="tool-btn ml-auto h-8 px-2.5 inline-flex items-center gap-1.5 font-sans font-semibold text-[var(--ink-1)] disabled:opacity-40 disabled:cursor-not-allowed"
                  :disabled="aiState === 'unavailable' || aiState === 'checking' || aiFormatting || !input.trim()"
                  :title="aiState === 'unavailable'
                    ? 'Needs Chrome 138+ with built-in AI (Gemini Nano) enabled.'
                    : 'Rewrite into LinkedIn style with on-device AI'"
                  @click="autoFormatWithAI"
                >
                  <span v-if="aiFormatting" class="h-3 w-3 animate-spin rounded-full border-2 border-current border-t-transparent" aria-hidden="true" />
                  <svg v-else class="h-3.5 w-3.5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true"><path d="M10 2l1.8 5.2L17 9l-5.2 1.8L10 16l-1.8-5.2L3 9l5.2-1.8z" /></svg>
                  {{ aiState === 'unavailable' ? 'AI rewrite unavailable' : 'AI rewrite' }}
                </button>
              </div>
            </section>

            <!-- Preview -->
            <section class="flex flex-col gap-3 md:sticky md:top-6 min-w-0" aria-label="Preview">

              <article class="ui-panel">
                <div class="flex items-start gap-3 px-4 pt-4">
                  <div class="h-12 w-12 shrink-0 rounded-full bg-[var(--ink-1)] text-white grid place-items-center text-sm font-bold" aria-hidden="true">You</div>
                  <div class="min-w-0 leading-tight">
                    <p class="text-sm font-bold">Your Name</p>
                    <p class="text-xs text-[var(--ink-3)] truncate">Your headline</p>
                    <p class="mt-0.5 text-xs text-[var(--ink-3)]">now · Public</p>
                  </div>
                </div>
                <div class="px-4 py-3 text-sm leading-[1.6] whitespace-pre-wrap break-words min-h-[8rem]">
                  <template v-if="formattedText">{{ visibleText }}<button
                    v-if="foldIndex >= 0"
                    class="ml-1 text-[var(--ink-3)] hover:text-[var(--ink-1)] hover:underline font-semibold"
                    @click="expanded = !expanded"
                  >{{ expanded ? 'show less' : '…see more' }}</button></template>
                  <span v-else class="text-[var(--ink-3)]">Your formatted post will appear here.</span>
                </div>
                <div class="flex border-t text-xs font-semibold text-[var(--ink-3)]" aria-hidden="true">
                  <span v-for="a in ['Like', 'Comment', 'Repost', 'Send']" :key="a" class="flex-1 py-3 text-center">{{ a }}</span>
                </div>
              </article>

              <p v-if="formattedText" class="text-xs text-[var(--ink-2)]">
                <span class="font-semibold text-[var(--ink-1)]">Above the fold:</span>
                {{ Array.from(hookText).length }} chars visible before "see more".
              </p>
            </section>
          </div>

          <details class="ui-panel group">
            <summary class="flex cursor-pointer list-none items-center justify-between px-4 py-3 text-sm font-bold">
              Formatting reference
              <svg class="h-4 w-4 transition-transform duration-200 group-open:rotate-180" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.8"><path stroke-linecap="round" stroke-linejoin="round" d="M5 8l5 5 5-5" /></svg>
            </summary>
            <div class="border-t px-4 py-4">
              <dl class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-px bg-[var(--surface-3)] border border-[var(--surface-3)]">
                <div v-for="r in REFERENCE" :key="r.md" class="bg-[var(--surface-1)] px-3 py-2.5">
                  <dt class="font-mono text-xs text-[var(--ink-3)]">{{ r.md }}</dt>
                  <dd class="mt-1 text-sm">{{ r.out }}</dd>
                </div>
              </dl>
              <p class="mt-4 max-w-3xl text-xs leading-relaxed text-[var(--ink-2)]">
                LinkedIn doesn't support real rich text. These styles swap letters for Unicode lookalikes, or attach
                combining marks for strikethrough and underline, so they survive pasting into LinkedIn, X, Slack, and
                anywhere else plain text is accepted. Screen readers may read styled letters awkwardly, so use them sparingly.
              </p>
            </div>
          </details>
        </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 text-xs font-mono" style="background: var(--surface-1); color: var(--ink-3)" role="status" aria-live="polite">
      <span class="truncate block">{{ status }}</span>
    </div>
  </div>
</template>

<style scoped>
.rail-btn,
.tool-btn {
  transition: transform 140ms var(--ease-out), background-color 140ms ease, color 140ms ease;
}

.tool-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--ink-1);
}

.tool-btn:focus-visible,
.rail-btn:focus-visible {
  outline: 2px solid var(--ink-1);
  outline-offset: -2px;
}

@media (hover: hover) and (pointer: fine) {
  .rail-btn:hover,
  .tool-btn:not(:disabled):hover {
    background: var(--surface-2);
    color: var(--ink-1);
  }
}

.rail-btn:active,
.tool-btn:not(:disabled):active {
  transform: scale(0.94);
}

.meter {
  transition: transform 200ms var(--ease-out);
}

#post-input:focus-visible {
  box-shadow: inset 0 0 0 2px var(--ink-1);
}

summary::-webkit-details-marker {
  display: none;
}

@media (prefers-reduced-motion: reduce) {
  .rail-btn,
  .tool-btn,
  .meter {
    transition: none !important;
  }
}
</style>
