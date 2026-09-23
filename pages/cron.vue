<script setup lang="ts">
import { CronExpressionParser } from 'cron-parser'
import cronstrue from 'cronstrue'

definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Cron Expression Explainer | Plain English & Next Runs in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser cron expression explainer. Translate cron syntax into plain English and preview upcoming run times. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/cron/' },
    { property: 'og:title', content: 'Free Cron Expression Explainer | Plain English & Next Runs' },
    { property: 'og:description', content: 'Explain cron expressions privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/cron/' }],
})

useToolSchema({
  name: 'Cron Expression Explainer',
  description: 'Translate cron expressions into plain English and preview next run times, locally in your browser.',
  url: 'https://ash-tools.store/cron/',
  category: 'DeveloperApplication',
})

const NEXT_RUN_COUNT = 10

const presets = [
  { label: 'Every minute', expr: '* * * * *' },
  { label: 'Every 5 minutes', expr: '*/5 * * * *' },
  { label: 'Every hour', expr: '0 * * * *' },
  { label: 'Daily at midnight', expr: '0 0 * * *' },
  { label: 'Weekdays at 9am', expr: '0 9 * * 1-5' },
  { label: 'First of month', expr: '0 0 1 * *' },
]

const expression = ref('*/5 * * * *')
const status = ref('Enter a cron expression.')
const isError = ref(false)

const explanation = computed(() => {
  if (!expression.value.trim()) return ''
  try {
    const result = cronstrue.toString(expression.value.trim(), { throwExceptionOnParseError: true })
    isError.value = false
    return result
  } catch (error) {
    isError.value = true
    status.value = error instanceof Error ? error.message : 'Could not parse cron expression.'
    return ''
  }
})

const fieldBreakdown = computed(() => {
  const parts = expression.value.trim().split(/\s+/)
  const labels = ['Minute', 'Hour', 'Day of month', 'Month', 'Day of week']
  if (parts.length !== 5) return []
  return labels.map((label, i) => ({ label, value: parts[i] }))
})

const nextRuns = computed<string[]>(() => {
  if (!expression.value.trim() || isError.value) return []
  try {
    const interval = CronExpressionParser.parse(expression.value.trim())
    const runs: string[] = []
    for (let i = 0; i < NEXT_RUN_COUNT; i += 1) {
      runs.push(interval.next().toDate().toLocaleString(undefined, {
        weekday: 'short', year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit', second: '2-digit',
      }))
    }
    return runs
  } catch {
    return []
  }
})

watch(expression, () => {
  if (!expression.value.trim()) {
    status.value = 'Enter a cron expression.'
    isError.value = false
    return
  }
  explanation.value // trigger evaluation for isError side effect
  if (!isError.value) status.value = `Valid — ${nextRuns.value.length} upcoming runs shown.`
})

const applyPreset = (expr: string) => {
  expression.value = expr
}

const copyExpression = async () => {
  try {
    await navigator.clipboard.writeText(expression.value)
    status.value = 'Expression copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}
</script>

<template>
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <div class="flex-1 p-4 md:p-6 flex flex-col md:flex-row gap-4">
      <aside class="ui-panel shrink-0 md:w-[68px] w-full">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-3">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 rounded-none flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
        </div>
      </aside>

      <main class="flex-1 min-w-0 mx-auto w-full max-w-4xl flex flex-col gap-4">
        <div class="ui-panel p-4 flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="10" cy="10" r="7" /><path stroke-linecap="round" stroke-linejoin="round" d="M10 6v4l3 2" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Cron Expression Explainer</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <section class="ui-panel p-4 flex flex-col gap-3">
          <div class="flex items-center gap-2">
            <input
              v-model="expression"
              type="text"
              spellcheck="false"
              placeholder="* * * * *"
              class="ui-input font-mono text-sm flex-1"
              :class="isError ? 'border-red-500' : ''"
            >
            <button class="ui-button-secondary !py-2 !px-3 text-xs shrink-0" @click="copyExpression">Copy</button>
          </div>

          <div class="flex flex-wrap gap-1.5">
            <button
              v-for="preset in presets"
              :key="preset.expr"
              class="ui-button-secondary !py-1 !px-2.5 text-xs"
              @click="applyPreset(preset.expr)"
            >
              {{ preset.label }}
            </button>
          </div>

          <div v-if="fieldBreakdown.length" class="grid grid-cols-5 gap-2">
            <div v-for="field in fieldBreakdown" :key="field.label" class="p-2 rounded-none border" style="background: var(--surface-2); border-color: var(--border)">
              <div class="text-[9px] uppercase font-semibold tracking-wider text-[var(--ink-3)]">{{ field.label }}</div>
              <div class="text-sm font-mono font-semibold text-[var(--ink-1)]">{{ field.value }}</div>
            </div>
          </div>

          <div v-if="explanation" class="p-3 rounded-none border" style="background: var(--surface-2); border-color: var(--border)">
            <p class="text-sm font-semibold text-[var(--ink-1)]">{{ explanation }}</p>
          </div>
        </section>

        <section class="ui-panel flex flex-col min-h-0">
          <div class="flex items-center justify-between px-4 pt-3 pb-2">
            <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Next {{ NEXT_RUN_COUNT }} runs</span>
          </div>
          <div class="px-4 pb-4">
            <div class="rounded-none overflow-hidden border" style="border-color: var(--border)">
              <div v-if="!nextRuns.length" class="p-4 text-sm text-[var(--ink-3)]" style="background: var(--surface-1)">Enter a valid expression to preview run times.</div>
              <div v-else class="max-h-80 overflow-auto divide-y" style="border-color: var(--border)">
                <div v-for="(run, i) in nextRuns" :key="i" class="flex items-center gap-3 px-3 py-2" style="background: var(--surface-1)">
                  <span class="w-6 shrink-0 text-xs font-mono text-[var(--ink-3)]">{{ i + 1 }}</span>
                  <span class="text-sm font-mono text-[var(--ink-1)]">{{ run }}</span>
                </div>
              </div>
            </div>
          </div>
        </section>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-3 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="status-dot w-2 h-2 rounded-full inline-block shrink-0" :class="isError ? 'err' : 'ok'"></span>
      <span class="truncate" :class="isError ? 'text-red-600' : ''">{{ status }}</span>
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
