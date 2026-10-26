<script setup lang="ts">
useHead({
  title: 'Free Wheel Spinner & Random Name Picker',
  meta: [
    { name: 'description', content: 'Spin a wheel to pick a random name, winner, or option. Fair random picks with the Web Crypto API. Free, private, and offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/picker/' },
    { property: 'og:title', content: 'Free Wheel Spinner & Random Name Picker' },
    { property: 'og:description', content: 'Spin a wheel to pick a random name or option in your browser.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/picker/' }],
})

useToolSchema({
  name: 'Wheel Spinner',
  description: 'Spin a wheel to pick a random name or option, with fair crypto-based randomness.',
  url: 'https://ash-tools.store/picker/',
  category: 'EntertainmentApplication',
})

const STORAGE_KEY = 'wheel-entries'
const MAX_ENTRIES = 200
const MAX_HISTORY = 20
const SPIN_MS = 4500
const SPINS = 6
const LABEL_MAX = 16
const DEFAULT_ENTRIES = ['Priya', 'Mateo', 'Aiko', 'Kwame', 'Sofia', 'Liam', 'Noor', 'Jonas'].join('\n')
const COLORS = ['var(--c-media)', 'var(--c-files)', 'var(--c-code)', 'var(--c-convert)', 'var(--c-text)', 'var(--c-life)', 'var(--accent)']

const text = ref(DEFAULT_ENTRIES)
const rotation = ref(0)
const spinning = ref(false)
const winner = ref('')
const history = ref<string[]>([])
const removeWinner = ref(false)
const reduceMotion = ref(false)
let pending = -1
let timer: ReturnType<typeof setTimeout> | undefined

const entries = computed(() => text.value.split('\n').map((s) => s.trim()).filter(Boolean).slice(0, MAX_ENTRIES))
const tooMany = computed(() => text.value.split('\n').filter((s) => s.trim()).length > MAX_ENTRIES)
const seg = computed(() => 360 / Math.max(1, entries.value.length))

const point = (deg: number, r: number) => {
  const a = (deg * Math.PI) / 180
  return `${(r * Math.sin(a)).toFixed(3)} ${(-r * Math.cos(a)).toFixed(3)}`
}

const slices = computed(() => {
  const n = entries.value.length
  const size = n > 16 ? 6 : n > 8 ? 8 : 11
  return entries.value.map((label, i) => {
    const a0 = i * seg.value
    const a1 = a0 + seg.value
    const mid = a0 + seg.value / 2
    const color = COLORS[(n > 1 && i === n - 1 && i % COLORS.length === 0) ? 1 : i % COLORS.length]
    return {
      label: label.length > LABEL_MAX ? `${label.slice(0, LABEL_MAX - 1)}…` : label,
      path: `M0 0 L${point(a0, 96)} A96 96 0 ${seg.value > 180 ? 1 : 0} 1 ${point(a1, 96)} Z`,
      color,
      transform: `rotate(${mid - 90}) translate(56 0)`,
      size,
    }
  })
})

const randomInt = (max: number) => {
  const limit = Math.floor(0x1_0000_0000 / max) * max
  const buf = new Uint32Array(1)
  do {
    crypto.getRandomValues(buf)
  } while (buf[0] >= limit)
  return buf[0] % max
}

const finish = () => {
  if (pending < 0) return
  const name = entries.value[pending]
  pending = -1
  spinning.value = false
  if (!name) return
  winner.value = name
  history.value = [name, ...history.value].slice(0, MAX_HISTORY)
  if (removeWinner.value) {
    const lines = text.value.split('\n')
    const idx = lines.findIndex((l) => l.trim() === name)
    if (idx >= 0) lines.splice(idx, 1)
    text.value = lines.join('\n')
  }
}

const spin = () => {
  const n = entries.value.length
  if (spinning.value || n < 2) return
  const w = randomInt(n)
  const jitter = (randomInt(1000) / 1000 - 0.5) * 0.7
  const p = (w + 0.5 + jitter) * seg.value
  const current = ((rotation.value % 360) + 360) % 360
  const delta = (((360 - p - current) % 360) + 360) % 360
  pending = w
  winner.value = ''
  spinning.value = true
  rotation.value += (reduceMotion.value ? 0 : SPINS * 360) + delta
  clearTimeout(timer)
  timer = setTimeout(finish, reduceMotion.value ? 50 : SPIN_MS + 100)
}

const shuffle = () => {
  if (spinning.value) return
  const list = [...entries.value]
  for (let i = list.length - 1; i > 0; i--) {
    const j = randomInt(i + 1)
    ;[list[i], list[j]] = [list[j], list[i]]
  }
  text.value = list.join('\n')
}

const sortAz = () => {
  if (spinning.value) return
  text.value = [...entries.value].sort((a, b) => a.localeCompare(b)).join('\n')
}

const clearAll = () => {
  if (spinning.value) return
  text.value = ''
  winner.value = ''
}

watch(text, (v) => localStorage.setItem(STORAGE_KEY, v))

onMounted(() => {
  const saved = localStorage.getItem(STORAGE_KEY)
  if (saved !== null) text.value = saved
  const mq = window.matchMedia('(prefers-reduced-motion: reduce)')
  reduceMotion.value = mq.matches
  mq.addEventListener('change', (e) => { reduceMotion.value = e.matches })
})

onUnmounted(() => clearTimeout(timer))
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
          <button class="rail-btn text-[var(--ink-3)]" title="Shuffle entries" aria-label="Shuffle entries" @click="shuffle">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 6h3l8 8h3M3 14h3l2-2M12 8l2-2h3M15 4l2 2-2 2M15 12l2 2-2 2" /></svg>
            <span>Shuffle</span>
          </button>
          <button class="rail-btn text-[var(--ink-3)]" title="Sort A to Z" aria-label="Sort A to Z" @click="sortAz">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 5h8M4 10h6M4 15h4M15 4v12M12.5 13.5L15 16l2.5-2.5" /></svg>
            <span>Sort</span>
          </button>
          <button class="rail-btn text-[var(--ink-3)]" title="Clear" aria-label="Clear" @click="clearAll">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M5 6h10M8 6V4h4v2m-6 0l1 10h6l1-10" /></svg>
            <span>Clear</span>
          </button>
        </div>
      </Teleport>
    </ClientOnly>

    <main class="flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel px-4 flex items-center justify-between gap-3">
          <div id="page-actions" class="order-first flex flex-wrap items-center gap-1"></div>
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="10" cy="11" r="6.5" /><path stroke-linecap="round" d="M10 11V4.5M10 11l5.6 3.2M10 11l-5.6 3.2M8 1.5h4L10 4z" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Wheel Spinner</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-[minmax(0,1fr)_320px] gap-4 items-start">
          <section class="ui-panel p-5 flex flex-col items-center gap-5">
            <div class="relative w-full max-w-[440px] aspect-square">
              <svg class="absolute left-1/2 -top-1 z-10 h-7 w-7 -translate-x-1/2 drop-shadow" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M12 22L3 4h18z" fill="var(--ink-1)" stroke="var(--surface-1)" stroke-width="1.5" stroke-linejoin="round" />
              </svg>
              <div v-if="entries.length < 2" class="absolute inset-0 grid place-items-center rounded-full border-2 border-dashed border-[var(--border-strong)] p-10 text-center text-sm text-[var(--ink-3)]">
                Add at least two entries to spin the wheel.
              </div>
              <svg
                v-else
                class="wheel h-full w-full"
                :class="spinning && !reduceMotion && 'is-spinning'"
                viewBox="-100 -100 200 200"
                role="img"
                :aria-label="`Wheel with ${entries.length} entries`"
                :style="{ transform: `rotate(${rotation}deg)` }"
              >
                <circle r="99" fill="var(--surface-1)" stroke="var(--border)" stroke-width="2" />
                <g v-for="(s, i) in slices" :key="i">
                  <path :d="s.path" :fill="s.color" stroke="var(--surface-1)" stroke-width="0.8" />
                  <text
                    :transform="s.transform"
                    text-anchor="middle"
                    dominant-baseline="middle"
                    fill="var(--surface-1)"
                    :font-size="s.size"
                    font-weight="700"
                  >{{ s.label }}</text>
                </g>
                <circle r="11" fill="var(--surface-1)" stroke="var(--border)" stroke-width="1.5" />
              </svg>
            </div>

            <button class="ui-button !px-8 !py-3 text-base" :disabled="spinning || entries.length < 2" @click="spin">
              {{ spinning ? 'Spinning...' : 'Spin' }}
            </button>

            <div class="min-h-20 w-full flex items-center justify-center" aria-live="polite">
              <div v-if="winner" :key="winner + history.length" class="winner rounded-[var(--radius-lg)] bg-[color-mix(in_oklab,var(--c-life)_14%,var(--surface-1))] px-8 py-4 text-center">
                <span class="block text-xs font-semibold text-[var(--ink-3)]">The wheel picked</span>
                <span class="block text-3xl font-extrabold tracking-tight text-[var(--ink-1)] break-all">{{ winner }}</span>
              </div>
            </div>
          </section>

          <aside class="flex flex-col gap-4">
            <section class="ui-panel p-4 flex flex-col gap-2">
              <div class="flex items-center justify-between">
                <label for="wheel-entries" class="text-sm font-bold">Entries</label>
                <span class="text-xs font-mono text-[var(--ink-3)]">{{ entries.length }} / {{ MAX_ENTRIES }}</span>
              </div>
              <textarea
                id="wheel-entries"
                v-model="text"
                :disabled="spinning"
                spellcheck="false"
                placeholder="One name or option per line"
                class="ui-input min-h-[260px] resize-y text-sm leading-relaxed"
              />
              <span class="text-xs text-[var(--ink-3)]">One entry per line.</span>
              <p v-if="tooMany" class="text-xs text-[var(--danger)]" role="alert">Only the first {{ MAX_ENTRIES }} entries are used.</p>
              <label class="mt-1 flex items-center gap-2 text-sm text-[var(--ink-2)]">
                <input v-model="removeWinner" type="checkbox"> Remove the winner after each spin
              </label>
            </section>

            <section class="ui-panel p-4">
              <div class="flex items-center justify-between">
                <h2 class="text-sm font-bold">Past picks</h2>
                <button v-if="history.length" class="text-xs font-semibold text-[var(--accent)] hover:underline" @click="history = []">Clear</button>
              </div>
              <p v-if="!history.length" class="mt-2 text-sm text-[var(--ink-3)]">Your picks will show here.</p>
              <ol v-else class="mt-2 flex flex-col gap-1 text-sm">
                <li v-for="(h, i) in history" :key="i" class="flex items-center gap-2">
                  <span class="w-5 shrink-0 text-right font-mono text-xs text-[var(--ink-3)]">{{ history.length - i }}</span>
                  <span class="truncate" :class="i === 0 ? 'font-bold text-[var(--ink-1)]' : 'text-[var(--ink-2)]'">{{ h }}</span>
                </li>
              </ol>
            </section>
          </aside>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.wheel {
  transition: transform 300ms var(--ease-out);
}

.wheel.is-spinning {
  transition: transform 4500ms cubic-bezier(0.15, 0.7, 0.1, 1);
}

.winner {
  transition: opacity 240ms var(--ease-out), transform 240ms var(--ease-out);

  @starting-style {
    opacity: 0;
    transform: scale(0.95);
  }
}
</style>
