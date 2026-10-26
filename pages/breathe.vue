<script setup lang="ts">
useHead({
  title: 'Free Breathing & Meditation Timer | Box Breathing, 4-7-8',
  meta: [
    { name: 'description', content: 'Guided breathing timer with box breathing, 4-7-8, and calm breathing patterns. Set a session length and follow the circle. Free and offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/breathe/' },
    { property: 'og:title', content: 'Free Breathing & Meditation Timer' },
    { property: 'og:description', content: 'Box breathing, 4-7-8, and calm breathing sessions in your browser.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/breathe/' }],
})

useToolSchema({
  name: 'Breathing Timer',
  description: 'Guided breathing and meditation timer with box breathing and 4-7-8 patterns.',
  url: 'https://ash-tools.store/breathe/',
  category: 'HealthApplication',
})

type Kind = 'in' | 'hold' | 'out' | 'rest'
type Pattern = { id: string, label: string, hint: string, steps: [number, number, number, number] }

const PATTERNS: Pattern[] = [
  { id: 'box', label: 'Box', hint: 'In 4, hold 4, out 4, hold 4', steps: [4, 4, 4, 4] },
  { id: '478', label: '4-7-8', hint: 'In 4, hold 7, out 8', steps: [4, 7, 8, 0] },
  { id: 'calm', label: 'Calm', hint: 'In 5.5, out 5.5', steps: [5.5, 0, 5.5, 0] },
  { id: 'relax', label: 'Relax', hint: 'In 4, out 6', steps: [4, 0, 6, 0] },
  { id: 'custom', label: 'Custom', hint: 'Set your own counts', steps: [4, 2, 6, 0] },
]
const SESSIONS = [1, 3, 5, 10, 0]
const MAX_STEP = 30
const MIN_SCALE = 0.5
const KIND_LABEL: Record<Kind, string> = { in: 'Breathe in', hold: 'Hold', out: 'Breathe out', rest: 'Hold' }
const KINDS: Kind[] = ['in', 'hold', 'out', 'rest']

const patternId = ref('box')
const custom = ref<[number, number, number, number]>([4, 2, 6, 0])
const sessionMin = ref(3)
const sound = ref(false)

const running = ref(false)
const started = ref(false)
const done = ref(false)
const phaseIdx = ref(0)
const phaseMs = ref(0)
const sessionMs = ref(0)
const cycles = ref(0)
const reduceMotion = ref(false)

let raf = 0
let last = 0
let audio: AudioContext | null = null

const steps = computed(() => {
  const raw = patternId.value === 'custom' ? custom.value : PATTERNS.find((p) => p.id === patternId.value)!.steps
  return raw
    .map((s, i) => ({ kind: KINDS[i], ms: Math.min(MAX_STEP, Math.max(0, Number(s) || 0)) * 1000 }))
    .filter((s) => s.ms > 0)
})

const validPattern = computed(() => steps.value.some((s) => s.kind === 'in') && steps.value.some((s) => s.kind === 'out'))
const phase = computed(() => steps.value[phaseIdx.value % Math.max(1, steps.value.length)])
const progress = computed(() => (phase.value ? Math.min(1, phaseMs.value / phase.value.ms) : 0))
const secondsLeft = computed(() => (phase.value ? Math.max(1, Math.ceil((phase.value.ms - phaseMs.value) / 1000)) : 0))
const limitMs = computed(() => sessionMin.value * 60_000)
const remaining = computed(() => (limitMs.value ? Math.max(0, limitMs.value - sessionMs.value) : sessionMs.value))

const ease = (p: number) => 0.5 - 0.5 * Math.cos(Math.PI * p)

const scale = computed(() => {
  if (!started.value || !phase.value) return MIN_SCALE + (1 - MIN_SCALE) * 0.35
  const p = ease(progress.value)
  const span = 1 - MIN_SCALE
  switch (phase.value.kind) {
    case 'in': return MIN_SCALE + span * p
    case 'out': return 1 - span * p
    case 'hold': return 1
    default: return MIN_SCALE
  }
})

const label = computed(() => {
  if (done.value) return 'Session done'
  if (!started.value) return 'Ready'
  if (!running.value) return 'Paused'
  return phase.value ? KIND_LABEL[phase.value.kind] : ''
})

const clock = (ms: number) => {
  const s = Math.round(ms / 1000)
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, '0')}`
}

const chime = (kind: Kind) => {
  if (!sound.value) return
  try {
    audio ??= new AudioContext()
    const osc = audio.createOscillator()
    const gain = audio.createGain()
    osc.frequency.value = kind === 'in' ? 528 : kind === 'out' ? 396 : 440
    gain.gain.setValueAtTime(0.0001, audio.currentTime)
    gain.gain.exponentialRampToValueAtTime(0.15, audio.currentTime + 0.03)
    gain.gain.exponentialRampToValueAtTime(0.0001, audio.currentTime + 0.6)
    osc.connect(gain).connect(audio.destination)
    osc.start()
    osc.stop(audio.currentTime + 0.65)
  } catch {
    sound.value = false
  }
}

const stopLoop = () => {
  cancelAnimationFrame(raf)
  raf = 0
}

const tick = (now: number) => {
  const dt = Math.min(1000, now - last)
  last = now
  phaseMs.value += dt
  sessionMs.value += dt
  while (phase.value && phaseMs.value >= phase.value.ms) {
    phaseMs.value -= phase.value.ms
    phaseIdx.value = (phaseIdx.value + 1) % steps.value.length
    if (phaseIdx.value === 0) cycles.value++
    chime(phase.value.kind)
  }
  if (limitMs.value && sessionMs.value >= limitMs.value) {
    running.value = false
    done.value = true
    stopLoop()
    return
  }
  raf = requestAnimationFrame(tick)
}

const start = () => {
  if (!validPattern.value) return
  if (done.value) reset()
  if (!started.value) chime('in')
  started.value = true
  running.value = true
  last = performance.now()
  stopLoop()
  raf = requestAnimationFrame(tick)
}

const pause = () => {
  running.value = false
  stopLoop()
}

const reset = () => {
  pause()
  started.value = false
  done.value = false
  phaseIdx.value = 0
  phaseMs.value = 0
  sessionMs.value = 0
  cycles.value = 0
}

const toggle = () => (running.value ? pause() : start())

const onKey = (e: KeyboardEvent) => {
  const tag = (e.target as HTMLElement | null)?.tagName
  if (e.code !== 'Space' || tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' || tag === 'BUTTON') return
  e.preventDefault()
  toggle()
}

watch([patternId, custom], reset, { deep: true })

onMounted(() => {
  const mq = window.matchMedia('(prefers-reduced-motion: reduce)')
  reduceMotion.value = mq.matches
  mq.addEventListener('change', (e) => { reduceMotion.value = e.matches })
  window.addEventListener('keydown', onKey)
})

onUnmounted(() => {
  stopLoop()
  window.removeEventListener('keydown', onKey)
  audio?.close()
})
</script>

<template>
  <div class="flex-1 flex flex-col">
    <ClientOnly>
      <Teleport to="#page-actions">
        <div class="flex items-center gap-1">
          <NuxtLink to="/" class="rail-btn text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span>Home</span>
          </NuxtLink>
          <button class="rail-btn text-[var(--ink-3)]" title="Reset" aria-label="Reset" @click="reset">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 10a6 6 0 1 0 1.8-4.3M4 4v3h3" /></svg>
            <span>Reset</span>
          </button>
        </div>
      </Teleport>
    </ClientOnly>

    <main class="flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel px-4 flex items-center justify-between gap-3">
          <div id="page-actions" class="order-first flex flex-wrap items-center gap-1"></div>
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="10" cy="10" r="7" /><circle cx="10" cy="10" r="3.5" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Breathing Timer</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-[minmax(0,1fr)_320px] gap-4 items-start">
          <section class="ui-panel p-6 flex flex-col items-center gap-6">
            <div class="relative grid w-full max-w-[380px] aspect-square place-items-center">
              <div class="absolute inset-0 rounded-full border-2 border-dashed border-[color-mix(in_oklab,var(--c-life)_35%,var(--border))]" aria-hidden="true" />
              <div
                class="breath absolute inset-0 rounded-full bg-[radial-gradient(circle_at_35%_30%,color-mix(in_oklab,var(--c-life)_55%,var(--surface-1)),var(--c-life))]"
                :style="reduceMotion ? { opacity: 0.35 + 0.65 * scale, transform: 'scale(0.85)' } : { transform: `scale(${scale})` }"
                aria-hidden="true"
              />
              <div class="relative z-10 text-center" aria-live="polite">
                <span class="block text-2xl font-extrabold tracking-tight text-[var(--surface-1)] drop-shadow-sm">{{ label }}</span>
                <span v-if="running && phase" class="block text-5xl font-extrabold font-mono text-[var(--surface-1)] drop-shadow-sm">{{ secondsLeft }}</span>
              </div>
            </div>

            <div class="flex items-center gap-2">
              <button class="ui-button !px-8 !py-3 text-base" :disabled="!validPattern" @click="toggle">
                {{ running ? 'Pause' : done ? 'Start again' : started ? 'Resume' : 'Start' }}
              </button>
              <button v-if="started" class="ui-button-secondary !px-5 !py-3 text-base" @click="reset">Reset</button>
            </div>

            <dl class="grid w-full max-w-md grid-cols-2 gap-3 text-center">
              <div class="rounded-[var(--radius-md)] bg-[var(--surface-2)] p-3">
                <dt class="text-xs text-[var(--ink-3)]">{{ limitMs ? 'Time left' : 'Time' }}</dt>
                <dd class="text-xl font-bold font-mono">{{ clock(remaining) }}</dd>
              </div>
              <div class="rounded-[var(--radius-md)] bg-[var(--surface-2)] p-3">
                <dt class="text-xs text-[var(--ink-3)]">Breaths</dt>
                <dd class="text-xl font-bold font-mono">{{ cycles }}</dd>
              </div>
            </dl>
            <p class="text-xs text-[var(--ink-3)]">Press Space to start or pause.</p>
          </section>

          <aside class="flex flex-col gap-4">
            <section class="ui-panel p-4 flex flex-col gap-3">
              <h2 class="text-sm font-bold">Pattern</h2>
              <div class="flex flex-col gap-1.5" role="radiogroup" aria-label="Breathing pattern">
                <button
                  v-for="p in PATTERNS"
                  :key="p.id"
                  type="button"
                  role="radio"
                  :aria-checked="patternId === p.id"
                  class="flex items-center justify-between gap-3 rounded-[var(--radius-md)] border px-3 py-2.5 text-left transition-colors duration-150"
                  :class="patternId === p.id ? 'border-[var(--c-life)] bg-[color-mix(in_oklab,var(--c-life)_10%,var(--surface-1))]' : 'border-[var(--border)] hover:bg-[var(--surface-2)]'"
                  @click="patternId = p.id"
                >
                  <span class="text-sm font-semibold">{{ p.label }}</span>
                  <span class="text-xs text-[var(--ink-3)]">{{ p.hint }}</span>
                </button>
              </div>

              <div v-if="patternId === 'custom'" class="grid grid-cols-4 gap-2">
                <label v-for="(name, i) in ['In', 'Hold', 'Out', 'Hold']" :key="i" class="flex flex-col gap-1">
                  <span class="text-xs font-semibold text-[var(--ink-2)]">{{ name }}</span>
                  <input v-model.number="custom[i]" type="number" min="0" :max="MAX_STEP" step="0.5" inputmode="decimal" class="ui-input !px-2 font-mono text-center">
                </label>
                <p class="col-span-4 text-xs text-[var(--ink-3)]">Seconds, 0 to {{ MAX_STEP }}. Use 0 to skip a hold.</p>
              </div>
              <p v-if="!validPattern" class="text-xs text-[var(--danger)]" role="alert">A pattern needs a breath in and a breath out.</p>
            </section>

            <section class="ui-panel p-4 flex flex-col gap-3">
              <h2 class="text-sm font-bold">Session</h2>
              <div class="flex flex-wrap gap-1.5">
                <button
                  v-for="m in SESSIONS"
                  :key="m"
                  type="button"
                  class="h-9 rounded-full border px-3.5 text-sm font-semibold transition-colors duration-150"
                  :class="sessionMin === m ? 'border-[var(--c-life)] bg-[color-mix(in_oklab,var(--c-life)_12%,var(--surface-1))] text-[var(--ink-1)]' : 'border-[var(--border-strong)] text-[var(--ink-2)] hover:bg-[var(--surface-2)]'"
                  :aria-pressed="sessionMin === m"
                  @click="sessionMin = m"
                >
                  {{ m ? `${m} min` : 'No limit' }}
                </button>
              </div>
              <label class="flex items-center gap-2 text-sm text-[var(--ink-2)]">
                <input v-model="sound" type="checkbox"> Soft chime on each step
              </label>
            </section>
          </aside>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.breath {
  transition: transform 120ms linear, opacity 300ms ease;
  will-change: transform;
}
</style>
