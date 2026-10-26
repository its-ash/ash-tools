<script setup lang="ts">
import type { ToolCategory } from '../composables/useTools'

useHead({
  title: 'Ash Tools | Private Browser Tools for Media, Docs, and Dev',
  meta: [
    { name: 'description', content: 'Ash Tools is a privacy-first toolkit for media editing, PDF and ZIP workflows, and developer utilities. Everything runs locally in your browser with no uploads.' },
    { name: 'keywords', content: 'free tools, image editor, pdf merger, zip compressor, online tools, web tools, code sandbox, json formatter, local processing, offline tools, privacy tools' },
    { property: 'og:type', content: 'website' },
    { property: 'og:url', content: 'https://ash-tools.store/' },
    { property: 'og:title', content: 'Ash Tools | Private Browser Tools for Media, Docs, and Dev' },
    { property: 'og:description', content: 'Privacy-first browser tools for media, docs, and developer workflows. Local processing, no uploads, offline-ready.' },
    { name: 'twitter:card', content: 'summary_large_image' },
    { name: 'robots', content: 'index,follow,max-snippet:-1,max-image-preview:large,max-video-preview:-1' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/' }],
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
        itemListElement: toolList.map((t, i) => ({
          '@type': 'ListItem',
          position: i + 1,
          name: t.title,
          url: `https://ash-tools.store${t.route}/`,
        })),
      }),
    },
  ],
})

const router = useRouter()
const query = ref('')
const filter = ref<ToolCategory | 'all'>('all')

const results = computed(() => matchTools(query.value).filter((t) => filter.value === 'all' || t.category === filter.value))

const groups = computed(() =>
  toolCategories
    .map((c) => ({ ...c, tools: results.value.filter((t) => t.category === c.id) }))
    .filter((g) => g.tools.length),
)

const countFor = (id: ToolCategory) => toolList.filter((t) => t.category === id).length

const onEnter = () => {
  const first = results.value[0]
  if (first) router.push(first.route)
}

const reset = () => {
  query.value = ''
  filter.value = 'all'
}
</script>

<template>
  <div class="mx-auto w-full max-w-7xl px-4 pb-20 pt-10 md:px-6 md:pt-16">
    <section class="max-w-3xl">
      <h1 class="text-4xl leading-[1.05] tracking-tight md:text-5xl">
        Everyday tools that never leave your browser
      </h1>
      <p class="mt-4 max-w-[60ch] text-base leading-relaxed text-[var(--ink-2)] md:text-lg">
        Edit images, merge PDFs, format JSON, and more. Your files stay on your device.
      </p>

      <label for="home-search" class="sr-only">Search tools</label>
      <div class="group mt-7 flex items-center gap-3 rounded-[var(--radius-lg)] border border-[var(--border-strong)] bg-[var(--surface-1)] px-4 shadow-[var(--shadow-md)] transition-[border-color,box-shadow] duration-150 focus-within:border-[var(--accent)] focus-within:shadow-[0_0_0_4px_color-mix(in_oklab,var(--accent)_18%,transparent)]">
        <svg class="h-5 w-5 shrink-0 text-[var(--ink-3)] group-focus-within:text-[var(--accent)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.8" aria-hidden="true">
          <circle cx="9" cy="9" r="6" />
          <path stroke-linecap="round" d="m17 17-4.3-4.3" />
        </svg>
        <input
          id="home-search"
          v-model="query"
          type="search"
          autocomplete="off"
          placeholder="Find a tool, like pdf, image, or json"
          class="h-14 w-full bg-transparent text-base text-[var(--ink-1)] outline-none placeholder:text-[var(--ink-3)] [&::-webkit-search-cancel-button]:hidden"
          @keydown.enter.prevent="onEnter"
          @keydown.esc="query = ''"
        >
        <button
          v-if="query"
          type="button"
          class="shrink-0 rounded-[var(--radius-sm)] px-2 py-1 text-xs font-semibold text-[var(--ink-3)] hover:bg-[var(--surface-2)] hover:text-[var(--ink-1)]"
          @click="query = ''"
        >
          Clear
        </button>
        <kbd v-else-if="results.length" class="hidden shrink-0 rounded-[6px] border border-[var(--border)] bg-[var(--surface-2)] px-1.5 py-0.5 font-mono text-[11px] text-[var(--ink-2)] sm:block">Enter</kbd>
      </div>
    </section>

    <nav class="mt-8 flex gap-2 overflow-x-auto pb-1" aria-label="Filter by category">
      <button
        type="button"
        class="chip"
        :class="filter === 'all' && 'chip-active'"
        :aria-pressed="filter === 'all'"
        @click="filter = 'all'"
      >
        All
        <span class="chip-count">{{ toolList.length }}</span>
      </button>
      <button
        v-for="c in toolCategories"
        :key="c.id"
        type="button"
        class="chip"
        :class="filter === c.id && 'chip-active'"
        :style="{ '--c': c.color }"
        :aria-pressed="filter === c.id"
        @click="filter = filter === c.id ? 'all' : c.id"
      >
        {{ c.label }}
        <span class="chip-count">{{ countFor(c.id) }}</span>
      </button>
    </nav>

    <div v-if="!groups.length" class="mt-10 rounded-[var(--radius-lg)] border border-dashed border-[var(--border-strong)] px-6 py-14 text-center">
      <p class="text-base font-semibold text-[var(--ink-1)]">No tools match "{{ query }}"</p>
      <p class="mt-1 text-sm text-[var(--ink-3)]">Try a shorter word, or look at all tools.</p>
      <button type="button" class="ui-button-secondary mt-5" @click="reset">Show all tools</button>
    </div>

    <section v-for="g in groups" :key="g.id" class="mt-10" :style="{ '--c': g.color }">
      <h2 class="flex items-center gap-2 text-lg tracking-tight">
        <span class="h-4 w-1 rounded-full bg-[var(--c)]" aria-hidden="true" />
        {{ g.label }}
      </h2>
      <div class="mt-4 grid grid-cols-[repeat(auto-fill,minmax(min(100%,17rem),1fr))] gap-3">
        <NuxtLink
          v-for="(t, i) in g.tools"
          :key="t.route"
          :to="t.route"
          class="tool-card"
          :style="{ '--i': i }"
        >
          <ToolIcon :tool="t" :size="44" />
          <span class="min-w-0">
            <span class="block text-[15px] font-bold leading-snug text-[var(--ink-1)]">{{ t.title }}</span>
            <span class="mt-0.5 block text-sm leading-snug text-[var(--ink-3)]">{{ t.description }}</span>
          </span>
        </NuxtLink>
      </div>
    </section>
  </div>
</template>

<style scoped>
.chip {
  display: inline-flex;
  flex-shrink: 0;
  align-items: center;
  gap: 0.5rem;
  height: 2.5rem;
  padding-inline: 1rem;
  border-radius: 999px;
  border: 1px solid var(--border-strong);
  background: var(--surface-1);
  color: var(--ink-2);
  font-size: 0.875rem;
  font-weight: 600;
  transition: background-color 150ms ease, border-color 150ms ease, color 150ms ease, transform 140ms var(--ease-out);
}

.chip:active {
  transform: scale(0.97);
}

.chip-count {
  font-size: 0.75rem;
  color: var(--ink-3);
}

.chip-active {
  --c-chip: var(--c, var(--accent));
  border-color: var(--c-chip);
  background: color-mix(in oklab, var(--c-chip) 12%, var(--surface-1));
  color: var(--ink-1);
}

.chip-active .chip-count {
  color: var(--ink-2);
}

.tool-card {
  display: flex;
  align-items: flex-start;
  gap: 0.875rem;
  padding: 1rem;
  border-radius: var(--radius-lg);
  border: 1px solid var(--border);
  background: var(--surface-1);
  box-shadow: var(--shadow-sm);
  transition: transform 180ms var(--ease-out), box-shadow 180ms var(--ease-out), border-color 180ms ease, background-color 180ms ease;
}

.tool-card:active {
  transform: scale(0.98);
}

@media (hover: hover) and (pointer: fine) {
  .chip:hover:not(.chip-active) {
    background: var(--surface-2);
    color: var(--ink-1);
  }

  .tool-card:hover {
    transform: translateY(-2px);
    border-color: color-mix(in oklab, var(--c) 45%, var(--border));
    background: color-mix(in oklab, var(--c) 5%, var(--surface-1));
    box-shadow: var(--shadow-md);
  }
}

@media (prefers-reduced-motion: no-preference) {
  .tool-card {
    animation: card-in 360ms var(--ease-out) backwards;
    animation-delay: calc(var(--i) * 35ms);
  }
}

@keyframes card-in {
  from {
    opacity: 0;
    transform: translateY(6px);
  }
}
</style>
