<script setup lang="ts">
useHead({
  title: 'Ash Tools | Private Browser Tools for Media, Docs, and Dev',
  meta: [
    { name: 'description', content: 'Ash Tools is a privacy-first toolkit for media editing, PDF and ZIP workflows, and developer utilities. Everything runs locally in your browser with no uploads.' },
    { name: 'keywords', content: 'free tools, video compressor, file zipper, online tools, web tools, compression, zip files, video editor, code sandbox, rust execution, javascript runner, local processing, offline tools, privacy tools' },
    { property: 'og:type', content: 'website' },
    { property: 'og:url', content: 'https://ash-tools.store/' },
    { property: 'og:title', content: 'Ash Tools | Private Browser Tools for Media, Docs, and Dev' },
    { property: 'og:description', content: 'Privacy-first browser tools for media, docs, and developer workflows. Local processing, no uploads, offline-ready.' },
    { name: 'twitter:card', content: 'summary_large_image' },
    { name: 'robots', content: 'index,follow,max-snippet:-1,max-image-preview:large,max-video-preview:-1' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/' }],
})

const tools = [
  {
    title: 'Video Studio',
    route: '/video',
    description: 'Trim, crop, and compress videos with WebAssembly and FFmpeg.',
    tag: 'Media',
  },
  {
    title: 'Image Editor',
    route: '/image',
    description: 'Crop, warp, and optimize images without leaving your browser.',
    tag: 'Media',
  },
  {
    title: 'ZIP Compressor',
    route: '/zip',
    description: 'Create ZIP archives from files and folders with drag-and-drop.',
    tag: 'Files',
  },
  {
    title: 'PDF Merger',
    route: '/pdf',
    description: 'Merge, reorder, and compress PDF documents offline.',
    tag: 'Files',
  },
  {
    title: 'PDF Sign',
    route: '/pdf-sign',
    description: 'Draw or upload a signature and place it anywhere on a PDF.',
    tag: 'Files',
  },
  {
    title: 'Code Sandbox',
    route: '/sandbox',
    description: 'Run and test code in multiple languages in an isolated runtime.',
    tag: 'Developer',
  },
  {
    title: 'JSON Formatter',
    route: '/json',
    description: 'Format, minify, and validate JSON with instant error feedback.',
    tag: 'Developer',
  },
  {
    title: 'Encode / Decode / Hash',
    route: '/encode',
    description: 'Base64, URL, and hex encoding plus SHA hash digests.',
    tag: 'Developer',
  },
  {
    title: 'Color Picker',
    route: '/color',
    description: 'Pick colors and convert between HEX, RGB, and HSL with palettes.',
    tag: 'Developer',
  },
  {
    title: 'QR Code Generator',
    route: '/qr',
    description: 'Turn text or links into a downloadable QR code, styled to match.',
    tag: 'Developer',
  },
  {
    title: 'Markdown Previewer',
    route: '/markdown',
    description: 'Write Markdown and see sanitized, live-rendered HTML side by side.',
    tag: 'Developer',
  },
  {
    title: 'UUID & Password Generator',
    route: '/uuid',
    description: 'Generate v4 UUIDs and strong random passwords with strength feedback.',
    tag: 'Developer',
  },
  {
    title: 'Diff Checker',
    route: '/diff',
    description: 'Compare two blocks of text line by line with additions and removals highlighted.',
    tag: 'Developer',
  },
  {
    title: 'Unit Converter',
    route: '/units',
    description: 'Convert length, weight, temperature, data size, and time units instantly.',
    tag: 'Developer',
  },
  {
    title: 'Timestamp Converter',
    route: '/timestamp',
    description: 'Convert between Unix timestamps, ISO 8601, and human-readable dates.',
    tag: 'Developer',
  },
  {
    title: 'Text Case & Lorem Ipsum',
    route: '/text',
    description: 'Convert text case, count words, and generate placeholder text.',
    tag: 'Developer',
  },
  {
    title: 'Regex Tester',
    route: '/regex',
    description: 'Test regular expressions against sample text with live match highlighting.',
    tag: 'Developer',
  },
  {
    title: 'CSV ⇄ JSON',
    route: '/csv',
    description: 'Convert CSV or TSV tables to JSON and back, with delimiter and header options.',
    tag: 'Developer',
  },
  {
    title: 'Cron Explainer',
    route: '/cron',
    description: 'Translate cron expressions into plain English and preview next run times.',
    tag: 'Developer',
  },
  {
    title: 'Image ⇄ Base64',
    route: '/base64-image',
    description: 'Convert images to Base64 data URIs and back, fully offline.',
    tag: 'Developer',
  },
  {
    title: 'LinkedIn Post Formatter',
    route: '/linkedin',
    description: 'Write with bold, italic, and bullets that render correctly in LinkedIn posts.',
    tag: 'Content',
  },
]

useHead({
  script: [
    {
      type: 'application/ld+json',
      innerHTML: JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'WebSite',
        name: 'Ash Tools',
        url: 'https://ash-tools.store/',
        description: 'Ash Tools is a free, privacy-first toolkit of browser utilities for media editing, PDF and ZIP workflows, and developer tasks.',
      }),
    },
    {
      type: 'application/ld+json',
      innerHTML: JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'ItemList',
        itemListElement: tools.map((tool, index) => ({
          '@type': 'ListItem',
          position: index + 1,
          name: tool.title,
          url: `https://ash-tools.store${tool.route}/`,
        })),
      }),
    },
  ],
})

// Hero search: a standalone, always-inline search (distinct from the header's
// ⌘K command palette) so both can exist on this page without fighting over
// the same open/close state or keyboard shortcut.
const heroQuery = ref('')
const heroActiveIndex = ref(0)
const heroInputFocused = ref(false)
const router = useRouter()

const heroResults = computed(() => {
  const q = heroQuery.value.trim().toLowerCase()
  if (!q) return tools
  return tools.filter((tool) =>
    tool.title.toLowerCase().includes(q) || tool.description.toLowerCase().includes(q) || tool.tag.toLowerCase().includes(q)
  )
})

watch(heroResults, () => { heroActiveIndex.value = 0 })

const heroShowResults = computed(() => heroInputFocused.value && heroQuery.value.trim().length > 0)

const goToTool = (route: string) => {
  heroInputFocused.value = false
  router.push(route)
}

// Delayed on blur so a click on a result (mousedown) has a chance to fire
// before the dropdown unmounts.
const onHeroBlur = () => {
  window.setTimeout(() => { heroInputFocused.value = false }, 150)
}

const onHeroKeydown = (event: KeyboardEvent) => {
  if (!heroShowResults.value) return
  if (event.key === 'ArrowDown') {
    event.preventDefault()
    heroActiveIndex.value = Math.min(heroActiveIndex.value + 1, heroResults.value.length - 1)
  } else if (event.key === 'ArrowUp') {
    event.preventDefault()
    heroActiveIndex.value = Math.max(heroActiveIndex.value - 1, 0)
  } else if (event.key === 'Enter') {
    event.preventDefault()
    const match = heroResults.value[heroActiveIndex.value]
    if (match) goToTool(match.route)
  } else if (event.key === 'Escape') {
    heroInputFocused.value = false
  }
}
</script>

<template>
  <div class="mx-auto max-w-7xl px-4 py-14 md:px-6 md:py-20">
    <section class="mx-auto max-w-2xl text-center">
      <span class="ui-badge ui-badge-accent">Local-first toolkit</span>
      <p class="mt-5 text-base leading-relaxed text-[var(--ink-2)]">
        20+ media, document, and developer tools that run entirely in your browser. No uploads, no accounts.
      </p>

      <div class="relative mt-6">
        <div class="ui-panel flex items-center gap-3 px-4 py-3.5 text-left">
          <svg class="h-5 w-5 shrink-0 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6">
            <circle cx="9" cy="9" r="6" />
            <path stroke-linecap="round" d="m17 17-4.3-4.3" />
          </svg>
          <input
            v-model="heroQuery"
            type="text"
            placeholder="Search 20+ tools... try &quot;pdf&quot; or &quot;json&quot;"
            class="w-full bg-transparent text-base text-[var(--ink-1)] outline-none placeholder:text-[var(--ink-3)]"
            @focus="heroInputFocused = true"
            @blur="onHeroBlur"
            @keydown="onHeroKeydown"
          >
        </div>

        <div
          v-if="heroShowResults"
          class="absolute inset-x-0 top-full z-20 mt-2 max-h-80 overflow-y-auto border-2 border-[var(--border)] bg-[var(--surface-1)] p-2 text-left"
        >
          <p v-if="!heroResults.length" class="px-3 py-6 text-center text-sm text-[var(--ink-3)]">
            No tools match "{{ heroQuery }}"
          </p>
          <button
            v-for="(tool, index) in heroResults"
            :key="tool.route"
            type="button"
            class="flex w-full items-center justify-between gap-3 rounded-none px-3 py-2.5 text-left transition-colors duration-100"
            :class="index === heroActiveIndex ? 'bg-[var(--ink-1)] text-white' : 'hover:bg-[var(--surface-2)]'"
            @mousedown.prevent="goToTool(tool.route)"
            @mouseenter="heroActiveIndex = index"
          >
            <span class="min-w-0">
              <span class="block truncate text-sm font-medium" :class="index === heroActiveIndex ? 'text-white' : 'text-[var(--ink-1)]'">{{ tool.title }}</span>
              <span class="block truncate text-xs" :class="index === heroActiveIndex ? 'text-white/70' : 'text-[var(--ink-3)]'">{{ tool.description }}</span>
            </span>
            <span class="ui-badge shrink-0" :class="index === heroActiveIndex ? '!border-white !text-white' : ''">{{ tool.tag }}</span>
          </button>
        </div>
      </div>

      <div class="mt-5 flex flex-wrap items-center justify-center gap-x-5 gap-y-2 text-xs mono text-[var(--ink-3)]">
        <span class="inline-flex items-center gap-1.5">
          <span class="h-1.5 w-1.5 shrink-0 rounded-full" style="background: var(--success)" />
          100% local processing
        </span>
        <span class="inline-flex items-center gap-1.5">
          <span class="h-1.5 w-1.5 shrink-0 rounded-full" style="background: var(--success)" />
          Works offline
        </span>
        <span class="inline-flex items-center gap-1.5">
          <span class="h-1.5 w-1.5 shrink-0 rounded-full" style="background: var(--success)" />
          No accounts
        </span>
      </div>
    </section>

    <section class="mt-20 border-t border-[var(--border)] pt-14">
      <h2 class="text-2xl font-extrabold tracking-tight text-[var(--ink-1)]">All tools</h2>
      <div class="mt-8 grid grid-cols-1 gap-px border border-[var(--border)] bg-[var(--border)] sm:grid-cols-2 lg:grid-cols-3">
        <NuxtLink
          v-for="tool in tools"
          :key="tool.route"
          :to="tool.route"
          class="group flex flex-col gap-3 bg-[var(--surface-1)] p-5 transition-colors duration-150 hover:bg-[var(--surface-2)]"
        >
          <span class="ui-badge w-fit">{{ tool.tag }}</span>
          <h3 class="text-base font-bold text-[var(--ink-1)]">{{ tool.title }}</h3>
          <p class="text-sm leading-relaxed text-[var(--ink-2)]">{{ tool.description }}</p>
          <span class="mt-1 inline-flex items-center gap-1 text-sm font-bold text-[var(--ink-1)]">
            Open tool
            <span class="transition-transform duration-150 group-hover:translate-x-0.5">→</span>
          </span>
        </NuxtLink>
      </div>
    </section>
  </div>
</template>
