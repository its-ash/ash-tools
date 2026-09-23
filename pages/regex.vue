<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Regex Tester | Live Pattern Matching in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser regex tester. Test regular expressions against sample text with live match highlighting and capture groups. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/regex/' },
    { property: 'og:title', content: 'Free Regex Tester | Live Pattern Matching' },
    { property: 'og:description', content: 'Test regular expressions privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/regex/' }],
})

useToolSchema({
  name: 'Regex Tester',
  description: 'Test regular expressions with live match highlighting, locally in your browser.',
  url: 'https://ash-tools.store/regex/',
  category: 'DeveloperApplication',
})

const pattern = ref('(\\w+)@(\\w+\\.\\w+)')
const testText = ref('Contact us at hello@ash-tools.store or support@example.com for help.')
const flagGlobal = ref(true)
const flagIgnoreCase = ref(false)
const flagMultiline = ref(false)
const flagDotAll = ref(false)
const flagUnicode = ref(false)
const status = ref('Enter a pattern to test.')
const isError = ref(false)

const flagsString = computed(() => {
  let f = ''
  if (flagGlobal.value) f += 'g'
  if (flagIgnoreCase.value) f += 'i'
  if (flagMultiline.value) f += 'm'
  if (flagDotAll.value) f += 's'
  if (flagUnicode.value) f += 'u'
  return f
})

type MatchInfo = { match: string; index: number; groups: (string | undefined)[]; namedGroups: Record<string, string> | null }

const compiledRegex = computed<RegExp | null>(() => {
  if (!pattern.value) return null
  try {
    const re = new RegExp(pattern.value, flagsString.value)
    isError.value = false
    return re
  } catch (error) {
    isError.value = true
    status.value = error instanceof Error ? error.message : 'Invalid regular expression.'
    return null
  }
})

const matches = computed<MatchInfo[]>(() => {
  const re = compiledRegex.value
  if (!re || !testText.value) return []
  const results: MatchInfo[] = []
  if (flagGlobal.value) {
    const globalRe = new RegExp(re.source, re.flags.includes('g') ? re.flags : re.flags + 'g')
    let m: RegExpExecArray | null
    let guard = 0
    while ((m = globalRe.exec(testText.value)) !== null && guard < 10000) {
      results.push({ match: m[0], index: m.index, groups: m.slice(1), namedGroups: m.groups ? { ...m.groups } : null })
      if (m[0] === '') globalRe.lastIndex += 1
      guard += 1
    }
  } else {
    const m = re.exec(testText.value)
    if (m) results.push({ match: m[0], index: m.index, groups: m.slice(1), namedGroups: m.groups ? { ...m.groups } : null })
  }
  return results
})

watch([pattern, testText, flagsString], () => {
  if (isError.value) return
  if (!pattern.value) { status.value = 'Enter a pattern to test.'; return }
  status.value = `${matches.value.length} match${matches.value.length === 1 ? '' : 'es'} found.`
})

const highlightedSegments = computed(() => {
  if (!testText.value) return [{ text: '', isMatch: false }]
  if (!matches.value.length) return [{ text: testText.value, isMatch: false }]
  const segments: { text: string; isMatch: boolean }[] = []
  let cursor = 0
  matches.value.forEach((m) => {
    if (m.index > cursor) segments.push({ text: testText.value.slice(cursor, m.index), isMatch: false })
    segments.push({ text: m.match || '', isMatch: true })
    cursor = m.index + m.match.length
  })
  if (cursor < testText.value.length) segments.push({ text: testText.value.slice(cursor), isMatch: false })
  return segments
})

const copyPattern = async () => {
  try {
    await navigator.clipboard.writeText(`/${pattern.value}/${flagsString.value}`)
    status.value = 'Pattern copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const clearAll = () => {
  pattern.value = ''
  testText.value = ''
  status.value = 'Enter a pattern to test.'
}
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
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 10h12M7 5l-3 5 3 5M13 5l3 5-3 5" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Regex Tester</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <section class="ui-panel p-4 flex flex-col gap-3">
          <div class="flex items-center gap-2">
            <span class="font-mono text-lg text-[var(--ink-3)] shrink-0">/</span>
            <input v-model="pattern" type="text" spellcheck="false" placeholder="pattern" class="ui-input font-mono text-sm flex-1" :class="isError ? 'border-red-500' : ''">
            <span class="font-mono text-lg text-[var(--ink-3)] shrink-0">/{{ flagsString }}</span>
            <button class="ui-button-secondary !py-2 !px-3 text-xs shrink-0" @click="copyPattern">Copy</button>
          </div>

          <div class="flex flex-wrap gap-4">
            <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
              <input v-model="flagGlobal" type="checkbox" class="ui-range"> Global (g)
            </label>
            <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
              <input v-model="flagIgnoreCase" type="checkbox" class="ui-range"> Ignore case (i)
            </label>
            <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
              <input v-model="flagMultiline" type="checkbox" class="ui-range"> Multiline (m)
            </label>
            <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
              <input v-model="flagDotAll" type="checkbox" class="ui-range"> Dot all (s)
            </label>
            <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
              <input v-model="flagUnicode" type="checkbox" class="ui-range"> Unicode (u)
            </label>
          </div>
        </section>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Test string</span>
            </div>
            <div class="px-4 pb-4">
              <textarea v-model="testText" spellcheck="false" placeholder="Paste text to test against..." class="ui-input font-mono text-xs w-full min-h-[200px] resize-none" />
            </div>
          </section>

          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Highlighted matches</span>
            </div>
            <div class="px-4 pb-4">
              <div class="ui-input font-mono text-xs w-full min-h-[200px] max-h-64 overflow-auto whitespace-pre-wrap break-words">
                <template v-for="(seg, i) in highlightedSegments" :key="i">
                  <mark v-if="seg.isMatch" style="background: var(--accent-soft); color: var(--accent-hover); padding: 0 1px">{{ seg.text }}</mark>
                  <span v-else>{{ seg.text }}</span>
                </template>
              </div>
            </div>
          </section>
        </div>

        <section class="ui-panel flex flex-col min-h-0">
          <div class="flex items-center justify-between px-4 pt-3 pb-2">
            <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Matches &amp; groups</span>
            <span class="text-xs font-mono text-[var(--ink-3)]">{{ matches.length }} found</span>
          </div>
          <div class="px-4 pb-4">
            <div class="overflow-hidden" style="border: 1px solid var(--border)">
              <div v-if="!matches.length" class="p-4 text-sm text-[var(--ink-3)]" style="background: var(--surface-1)">No matches yet.</div>
              <div v-else class="max-h-72 overflow-auto divide-y" style="border-color: var(--border)">
                <div v-for="(m, i) in matches" :key="i" class="px-3 py-2 flex flex-col gap-1" style="background: var(--surface-1)">
                  <div class="flex items-center gap-2 text-xs font-mono">
                    <span class="font-semibold shrink-0 text-[var(--ink-3)]">#{{ i + 1 }} @{{ m.index }}</span>
                    <span class="truncate text-[var(--ink-1)]" style="background: var(--accent-soft); padding: 0 4px">{{ m.match }}</span>
                  </div>
                  <div v-if="m.groups.length" class="flex flex-wrap gap-1.5 pl-2">
                    <span v-for="(g, gi) in m.groups" :key="gi" class="text-xs font-mono px-1.5 py-0.5" style="background: var(--surface-2); color: var(--ink-2); border: 1px solid var(--border)">
                      ${{ gi + 1 }}: {{ g ?? '—' }}
                    </span>
                  </div>
                  <div v-if="m.namedGroups" class="flex flex-wrap gap-1.5 pl-2">
                    <span v-for="(v, k) in m.namedGroups" :key="k" class="text-xs font-mono px-1.5 py-0.5" style="background: var(--surface-2); color: var(--ink-2); border: 1px solid var(--border)">
                      {{ k }}: {{ v }}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
      </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-3 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="status-dot w-2 h-2 shrink-0 inline-block" :class="isError ? 'err' : 'ok'"></span>
      <span class="truncate" :class="isError ? 'text-red-600' : ''">{{ status }}</span>
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
</style>
