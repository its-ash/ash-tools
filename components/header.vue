<script setup lang="ts">
const THEME_KEY = 'theme'
const currentTheme = ref<'dark' | 'light'>('light')
const route = useRoute()

const activeTool = computed(() => toolList.find((t) => route.path.replace(/\/$/, '') === t.route))

const getEffectiveTheme = (): 'dark' | 'light' => {
  const html = document.documentElement
  if (html.classList.contains('dark')) return 'dark'
  if (html.classList.contains('light')) return 'light'
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}

const applyTheme = (theme: 'dark' | 'light') => {
  const html = document.documentElement
  html.classList.remove('light', 'dark')
  html.classList.add(theme)
  currentTheme.value = theme
}

onMounted(() => {
  const stored = localStorage.getItem(THEME_KEY)
  stored === 'dark' || stored === 'light' ? applyTheme(stored) : (currentTheme.value = getEffectiveTheme())
})

const toggleTheme = () => {
  const next = currentTheme.value === 'dark' ? 'light' : 'dark'
  applyTheme(next)
  localStorage.setItem(THEME_KEY, next)
}

const themeLabel = computed(() => currentTheme.value === 'dark' ? 'Switch to light theme' : 'Switch to dark theme')
</script>

<template>
  <header class="sticky top-0 z-50 border-b border-[var(--border)] bg-[color-mix(in_oklab,var(--surface-0)_85%,transparent)] backdrop-blur-md">
    <div class="brand-stripe h-[3px]" aria-hidden="true" />
    <div class="mx-auto flex h-16 max-w-7xl items-center gap-3 px-4 md:px-6">
      <NuxtLink to="/" class="flex shrink-0 items-center gap-2.5 rounded-[var(--radius-sm)]">
        <span class="grid h-8 w-8 place-items-center rounded-[10px] bg-[var(--accent)] shadow-[var(--shadow-sm)]" aria-hidden="true">
          <svg width="18" height="18" viewBox="0 0 24 24">
            <path d="M6 12.5L10 16.5L18 8" fill="none" stroke="var(--accent-ink)" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </span>
        <span class="hidden text-base font-extrabold tracking-tight text-[var(--ink-1)] sm:inline">Ash Tools</span>
      </NuxtLink>

      <template v-if="activeTool">
        <span class="hidden text-[var(--border-strong)] md:inline" aria-hidden="true">/</span>
        <span class="hidden min-w-0 items-center gap-2 md:flex">
          <ToolIcon :tool="activeTool" :size="22" />
          <span class="truncate text-sm font-semibold text-[var(--ink-2)]">{{ activeTool.title }}</span>
        </span>
      </template>

      <div class="flex-1" />

      <ToolSearch class="shrink-0" />

      <button
        type="button"
        class="grid h-10 w-10 shrink-0 place-items-center rounded-[var(--radius-sm)] text-[var(--ink-2)] transition-colors duration-150 hover:bg-[var(--surface-2)] hover:text-[var(--ink-1)]"
        :aria-label="themeLabel"
        :title="themeLabel"
        @click="toggleTheme"
      >
        <svg v-if="currentTheme === 'dark'" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="4.5" /><path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4" /></svg>
        <svg v-else width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" /></svg>
      </button>
    </div>
  </header>
</template>
