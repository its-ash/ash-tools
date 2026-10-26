<script setup lang="ts">
import type { Tool } from '../composables/useTools'

const isOpen = ref(false)
const query = ref('')
const activeIndex = ref(0)
const inputEl = ref<HTMLInputElement | null>(null)
const listEl = ref<HTMLUListElement | null>(null)
const isMac = ref(false)
const router = useRouter()

const results = computed(() => matchTools(query.value))

watch(results, () => { activeIndex.value = 0 })

watch(activeIndex, (i) => {
  nextTick(() => listEl.value?.children[i]?.scrollIntoView({ block: 'nearest' }))
})

const open = () => {
  isOpen.value = true
  query.value = ''
  activeIndex.value = 0
  nextTick(() => inputEl.value?.focus())
}

const close = () => { isOpen.value = false }

const go = (tool?: Tool) => {
  const target = tool ?? results.value[activeIndex.value]
  if (!target) return
  close()
  router.push(target.route)
}

const onKeydown = (e: KeyboardEvent) => {
  const n = results.value.length
  if (e.key === 'ArrowDown' && n) {
    e.preventDefault()
    activeIndex.value = (activeIndex.value + 1) % n
  } else if (e.key === 'ArrowUp' && n) {
    e.preventDefault()
    activeIndex.value = (activeIndex.value - 1 + n) % n
  } else if (e.key === 'Enter') {
    e.preventDefault()
    go()
  } else if (e.key === 'Escape') {
    close()
  }
}

const onGlobalKeydown = (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
    e.preventDefault()
    isOpen.value ? close() : open()
  }
}

onMounted(() => {
  isMac.value = /Mac|iPhone|iPad/.test(navigator.platform || navigator.userAgent)
  window.addEventListener('keydown', onGlobalKeydown)
})
onUnmounted(() => window.removeEventListener('keydown', onGlobalKeydown))
</script>

<template>
  <button
    type="button"
    v-bind="$attrs"
    class="flex h-10 items-center gap-2 rounded-[var(--radius-sm)] border border-[var(--border-strong)] bg-[var(--surface-1)] px-3 text-sm text-[var(--ink-3)] shadow-[var(--shadow-sm)] transition-colors duration-150 hover:border-[var(--accent)] hover:text-[var(--ink-1)] sm:w-60"
    aria-label="Search tools"
    @click="open"
  >
    <svg class="h-4 w-4 shrink-0" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.8" aria-hidden="true">
      <circle cx="9" cy="9" r="6" />
      <path stroke-linecap="round" d="m17 17-4.3-4.3" />
    </svg>
    <span class="hidden flex-1 text-left sm:inline">Search tools</span>
    <kbd class="hidden shrink-0 rounded-[6px] border border-[var(--border)] bg-[var(--surface-2)] px-1.5 py-0.5 font-mono text-[11px] text-[var(--ink-2)] sm:block">
      {{ isMac ? '⌘' : 'Ctrl' }} K
    </kbd>
  </button>

  <Teleport to="body">
    <Transition
      enter-active-class="transition-opacity duration-150 ease-out"
      leave-active-class="transition-opacity duration-100 ease-out"
      enter-from-class="opacity-0"
      leave-to-class="opacity-0"
    >
      <div
        v-if="isOpen"
        class="fixed inset-0 z-[100] flex items-start justify-center bg-[rgb(12_16_32/0.45)] px-4 pt-[12vh] backdrop-blur-[2px]"
        @click.self="close"
      >
        <div
          class="palette w-full max-w-xl overflow-hidden rounded-[var(--radius-lg)] border border-[var(--border)] bg-[var(--surface-1)] shadow-[var(--shadow-lg)]"
          role="dialog"
          aria-modal="true"
          aria-label="Search tools"
        >
          <div class="flex items-center gap-3 border-b border-[var(--border)] px-4">
            <svg class="h-5 w-5 shrink-0 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.8" aria-hidden="true">
              <circle cx="9" cy="9" r="6" />
              <path stroke-linecap="round" d="m17 17-4.3-4.3" />
            </svg>
            <input
              ref="inputEl"
              v-model="query"
              type="text"
              placeholder="Search tools..."
              aria-label="Search tools"
              aria-controls="tool-search-list"
              class="h-14 flex-1 bg-transparent text-base text-[var(--ink-1)] outline-none placeholder:text-[var(--ink-3)]"
              @keydown="onKeydown"
            >
            <kbd class="hidden shrink-0 rounded-[6px] border border-[var(--border)] bg-[var(--surface-2)] px-1.5 py-0.5 font-mono text-[11px] text-[var(--ink-2)] sm:block">Esc</kbd>
          </div>

          <p v-if="!results.length" class="px-4 py-10 text-center text-sm text-[var(--ink-3)]">
            No tools match "{{ query }}". Try "pdf", "image", or "json".
          </p>
          <ul v-else id="tool-search-list" ref="listEl" class="max-h-[60vh] overflow-y-auto p-2" role="listbox">
            <li v-for="(tool, i) in results" :key="tool.route" role="option" :aria-selected="i === activeIndex">
              <button
                type="button"
                class="flex w-full items-center gap-3 rounded-[var(--radius-md)] px-3 py-2.5 text-left transition-colors duration-100"
                :class="i === activeIndex ? 'bg-[var(--accent-soft)]' : ''"
                @click="go(tool)"
                @mousemove="activeIndex = i"
              >
                <ToolIcon :tool="tool" :size="32" />
                <span class="min-w-0 flex-1">
                  <span class="block truncate text-sm font-semibold text-[var(--ink-1)]">{{ tool.title }}</span>
                  <span class="block truncate text-xs text-[var(--ink-3)]">{{ tool.description }}</span>
                </span>
                <svg v-if="i === activeIndex" class="h-4 w-4 shrink-0 text-[var(--accent)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.8" aria-hidden="true">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M8.5 4L14 9.5 8.5 15" />
                </svg>
              </button>
            </li>
          </ul>

          <div class="hidden items-center gap-4 border-t border-[var(--border)] bg-[var(--surface-2)] px-4 py-2 text-xs text-[var(--ink-3)] sm:flex">
            <span><kbd class="font-mono">↑↓</kbd> to move</span>
            <span><kbd class="font-mono">Enter</kbd> to open</span>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.palette {
  transition: opacity 160ms var(--ease-out), transform 160ms var(--ease-out);

  @starting-style {
    opacity: 0;
    transform: scale(0.97) translateY(-4px);
  }
}
</style>
