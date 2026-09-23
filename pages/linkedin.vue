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

const toolbarGroups: { label: string; buttons: { label: string; title: string; action: () => void; className?: string }[] }[] = [
  {
    label: 'Style selection',
    buttons: [
      { label: 'B', title: 'Bold', action: () => wrapSelection('**', 'Bold'), className: 'font-bold' },
      { label: 'I', title: 'Italic', action: () => wrapSelection('*', 'Italic'), className: 'italic' },
      { label: 'B/I', title: 'Bold + Italic', action: () => wrapSelection('***', 'Bold + Italic'), className: 'font-bold italic' },
      { label: 'S', title: 'Strikethrough', action: () => wrapSelection('~~', 'Strikethrough'), className: 'line-through' },
      { label: 'U', title: 'Underline', action: () => wrapSelection('__', 'Underline'), className: 'underline' },
      { label: '</>', title: 'Monospace (code)', action: () => wrapSelection('`', 'Monospace') },
      { label: 'ᴀᴀ', title: 'Small caps', action: applySmallCapsToSelection },
    ],
  },
  {
    label: 'Insert block',
    buttons: [
      { label: '• List', title: 'Insert bullet list', action: insertBullets },
      { label: '1. List', title: 'Insert numbered list', action: insertNumbered },
      { label: '☑ Checklist', title: 'Insert checklist', action: insertChecklist },
      { label: '── Divider', title: 'Insert section divider', action: insertDivider },
    ],
  },
]

const copyFormatted = async () => {
  if (!formattedText.value) return
  try {
    await navigator.clipboard.writeText(formattedText.value)
    status.value = 'Copied — paste directly into LinkedIn.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const clearAll = () => {
  input.value = ''
  status.value = 'Write your post using **bold**, *italic*, and more.'
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
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <div class="flex-1 flex flex-col md:flex-row">
      <aside class="order-2 md:order-1 shrink-0 border-t md:border-t-0 md:border-r md:w-[76px] w-full" style="background: var(--surface-1); border-color: var(--border)">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-4 overflow-x-auto md:overflow-visible">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
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
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="3" width="14" height="14" rx="2" /><path stroke-linecap="round" stroke-linejoin="round" d="M6.5 8.5v5M6.5 6.5v.01M10 13.5v-3a1.7 1.7 0 013.4 0v3M10 10.5v3" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">LinkedIn Post Formatter</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="ui-panel p-3 flex flex-col gap-2.5">
          <div v-for="group in toolbarGroups" :key="group.label" class="flex flex-wrap items-center gap-2">
            <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)] w-24 shrink-0">{{ group.label }}</span>
            <button
              v-for="btn in group.buttons"
              :key="btn.label"
              class="ui-button-secondary !py-1.5 !px-3 text-xs"
              :class="btn.className"
              :title="btn.title"
              @click="btn.action"
            >
              {{ btn.label }}
            </button>
          </div>
          <div class="flex items-center gap-2 pt-1 border-t" style="border-color: var(--border)">
            <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)] w-24 shrink-0">Actions</span>
            <button class="ui-button-secondary !py-1.5 !px-3 text-xs" @click="loadSample">Load sample</button>
            <button
              class="ui-button-secondary !py-1.5 !px-3 text-xs inline-flex items-center gap-1.5"
              :disabled="aiState === 'unavailable' || aiState === 'checking' || aiFormatting || !input.trim()"
              :title="aiState === 'unavailable'
                ? 'Chrome built-in AI (Gemini Nano) is not available in this browser. Try Chrome 138+ with the on-device model enabled.'
                : 'Rewrite this post into LinkedIn style using on-device AI'"
              @click="autoFormatWithAI"
            >
              <span v-if="aiFormatting" class="h-3 w-3 shrink-0 animate-spin rounded-full border-2 border-current border-t-transparent" aria-hidden="true" />
              <span v-else>✨</span>
              Auto-format{{ aiState === 'unavailable' ? ' (unavailable)' : '' }}
            </button>
            <button class="ui-button !py-1.5 !px-3 text-xs ml-auto" :disabled="!formattedText" @click="copyFormatted">Copy for LinkedIn</button>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 flex-1 min-h-0">
          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Write (Markdown-style)</span>
              <span class="text-xs font-mono text-[var(--ink-3)]">{{ lineCount }} lines</span>
            </div>
            <div class="px-4 pb-4 flex-1 min-h-0">
              <textarea
                id="post-input"
                v-model="input"
                spellcheck="true"
                placeholder="Write your post... use **bold**, *italic*, ~~strike~~, __underline__, `code`, - bullets, 1. numbers, [ ] checklist, --- divider"
                class="ui-input font-mono text-xs w-full h-full min-h-[360px] resize-none"
              />
            </div>
          </section>

          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Preview (as LinkedIn will show it)</span>
              <span class="text-xs font-mono" :class="overLimit ? 'text-red-600' : 'text-[var(--ink-3)]'">{{ charCount }} / {{ LINKEDIN_SAFE_LIMIT }}</span>
            </div>
            <div class="px-4 pb-4 flex-1 min-h-0">
              <div class="ui-input font-sans text-sm w-full h-full min-h-[360px] overflow-auto whitespace-pre-wrap break-words leading-relaxed">{{ formattedText || 'Formatted preview will appear here.' }}</div>
            </div>
          </section>
        </div>

        <section class="ui-panel p-4">
          <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Formatting reference</span>
          <div class="mt-3 grid grid-cols-1 sm:grid-cols-2 gap-x-6 gap-y-1.5 text-xs">
            <div class="flex items-center justify-between gap-2 py-1 border-b" style="border-color: var(--border)">
              <code class="font-mono px-1 rounded-none" style="background: var(--surface-2)">**bold**</code>
              <span class="text-[var(--ink-2)]">𝐁𝐨𝐥𝐝 text</span>
            </div>
            <div class="flex items-center justify-between gap-2 py-1 border-b" style="border-color: var(--border)">
              <code class="font-mono px-1 rounded-none" style="background: var(--surface-2)">*italic*</code>
              <span class="text-[var(--ink-2)]">𝘐𝘵𝘢𝘭𝘪𝘤 text</span>
            </div>
            <div class="flex items-center justify-between gap-2 py-1 border-b" style="border-color: var(--border)">
              <code class="font-mono px-1 rounded-none" style="background: var(--surface-2)">***bold italic***</code>
              <span class="text-[var(--ink-2)]">𝑩𝒐𝒍𝒅 𝒊𝒕𝒂𝒍𝒊𝒄</span>
            </div>
            <div class="flex items-center justify-between gap-2 py-1 border-b" style="border-color: var(--border)">
              <code class="font-mono px-1 rounded-none" style="background: var(--surface-2)">~~strike~~</code>
              <span class="text-[var(--ink-2)]">S̶t̶r̶i̶k̶e̶ text</span>
            </div>
            <div class="flex items-center justify-between gap-2 py-1 border-b" style="border-color: var(--border)">
              <code class="font-mono px-1 rounded-none" style="background: var(--surface-2)">__underline__</code>
              <span class="text-[var(--ink-2)]">U̲n̲d̲e̲r̲l̲i̲n̲e̲ text</span>
            </div>
            <div class="flex items-center justify-between gap-2 py-1 border-b" style="border-color: var(--border)">
              <code class="font-mono px-1 rounded-none" style="background: var(--surface-2)">`code`</code>
              <span class="text-[var(--ink-2)]">𝚖𝚘𝚗𝚘𝚜𝚙𝚊𝚌𝚎 text</span>
            </div>
            <div class="flex items-center justify-between gap-2 py-1 border-b" style="border-color: var(--border)">
              <code class="font-mono px-1 rounded-none" style="background: var(--surface-2)">- item</code>
              <span class="text-[var(--ink-2)]">• Bullet list</span>
            </div>
            <div class="flex items-center justify-between gap-2 py-1 border-b" style="border-color: var(--border)">
              <code class="font-mono px-1 rounded-none" style="background: var(--surface-2)">1. item</code>
              <span class="text-[var(--ink-2)]">① Numbered list</span>
            </div>
            <div class="flex items-center justify-between gap-2 py-1 border-b" style="border-color: var(--border)">
              <code class="font-mono px-1 rounded-none" style="background: var(--surface-2)">[ ] / [x]</code>
              <span class="text-[var(--ink-2)]">☐ / ☑ Checklist</span>
            </div>
            <div class="flex items-center justify-between gap-2 py-1">
              <code class="font-mono px-1 rounded-none" style="background: var(--surface-2)">---</code>
              <span class="text-[var(--ink-2)]">─── Divider</span>
            </div>
            <div class="flex items-center justify-between gap-2 py-1">
              <span class="text-[var(--ink-3)]">Select text + "ᴀᴀ" button</span>
              <span class="text-[var(--ink-2)]">sᴍᴀʟʟ ᴄᴀᴘs</span>
            </div>
          </div>
          <p class="mt-3 text-xs leading-relaxed text-[var(--ink-2)]">
            LinkedIn posts don't support real HTML formatting — these effects come from swapping letters for styled
            Unicode characters, or attaching invisible combining marks for strikethrough and underline. Anything typed
            here works when pasted into LinkedIn, X, Slack, or anywhere else plain Unicode text is accepted.
          </p>
        </section>
      </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-4 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
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
