<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free UUID & Password Generator | Local & Secure in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser UUID and password generator. Generate v4 UUIDs and strong random passwords locally using the Web Crypto API. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/uuid/' },
    { property: 'og:title', content: 'Free UUID & Password Generator | Local & Secure' },
    { property: 'og:description', content: 'Generate UUIDs and secure passwords privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/uuid/' }],
})

useToolSchema({
  name: 'UUID & Password Generator',
  description: 'Generate v4 UUIDs and strong random passwords locally using the Web Crypto API.',
  url: 'https://ash-tools.store/uuid/',
  category: 'DeveloperApplication',
})

const DEFAULT_UUID_COUNT = 5
const DEFAULT_PASSWORD_LENGTH = 20
const MIN_PASSWORD_LENGTH = 4
const MAX_PASSWORD_LENGTH = 128
const MAX_UUID_COUNT = 100

const status = ref('Generate UUIDs or passwords below.')

// --- UUID ---
const uuidCount = ref(DEFAULT_UUID_COUNT)
const uuidUppercase = ref(false)
const uuidHyphens = ref(true)
const uuids = ref<string[]>([])

const formatUuid = (raw: string) => {
  let value = uuidHyphens.value ? raw : raw.replace(/-/g, '')
  if (uuidUppercase.value) value = value.toUpperCase()
  return value
}

const generateUuids = () => {
  const count = Math.min(MAX_UUID_COUNT, Math.max(1, Math.round(uuidCount.value)))
  uuidCount.value = count
  uuids.value = Array.from({ length: count }, () => formatUuid(crypto.randomUUID()))
  status.value = `Generated ${count} UUID${count === 1 ? '' : 's'}.`
}

const copyUuid = async (value: string) => {
  try {
    await navigator.clipboard.writeText(value)
    status.value = 'UUID copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const copyAllUuids = async () => {
  if (!uuids.value.length) return
  try {
    await navigator.clipboard.writeText(uuids.value.join('\n'))
    status.value = `Copied ${uuids.value.length} UUIDs.`
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

// --- Password ---
const passwordLength = ref(DEFAULT_PASSWORD_LENGTH)
const useUppercase = ref(true)
const useLowercase = ref(true)
const useNumbers = ref(true)
const useSymbols = ref(true)
const excludeAmbiguous = ref(true)
const password = ref('')

const CHAR_SETS = {
  uppercase: 'ABCDEFGHIJKLMNOPQRSTUVWXYZ',
  lowercase: 'abcdefghijklmnopqrstuvwxyz',
  numbers: '0123456789',
  symbols: '!@#$%^&*()-_=+[]{};:,.<>?/',
}
const AMBIGUOUS = new Set('0O1lI'.split(''))

const passwordCharset = computed(() => {
  let charset = ''
  if (useUppercase.value) charset += CHAR_SETS.uppercase
  if (useLowercase.value) charset += CHAR_SETS.lowercase
  if (useNumbers.value) charset += CHAR_SETS.numbers
  if (useSymbols.value) charset += CHAR_SETS.symbols
  if (excludeAmbiguous.value) charset = [...charset].filter((c) => !AMBIGUOUS.has(c)).join('')
  return charset
})

const passwordStrength = computed(() => {
  const charsetSize = passwordCharset.value.length || 1
  const entropy = passwordLength.value * Math.log2(charsetSize)
  if (entropy < 40) return { label: 'Weak', color: '#dc2626' }
  if (entropy < 65) return { label: 'Fair', color: '#d97706' }
  if (entropy < 90) return { label: 'Strong', color: '#16a34a' }
  return { label: 'Very strong', color: '#16a34a' }
})

const generatePassword = () => {
  const charset = passwordCharset.value
  if (!charset) {
    password.value = ''
    status.value = 'Select at least one character type.'
    return
  }
  const length = Math.min(MAX_PASSWORD_LENGTH, Math.max(MIN_PASSWORD_LENGTH, Math.round(passwordLength.value)))
  passwordLength.value = length
  const randomValues = new Uint32Array(length)
  crypto.getRandomValues(randomValues)
  password.value = Array.from(randomValues, (v) => charset[v % charset.length]).join('')
  status.value = 'Generated a new password.'
}

const copyPassword = async () => {
  if (!password.value) return
  try {
    await navigator.clipboard.writeText(password.value)
    status.value = 'Password copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

watch([useUppercase, useLowercase, useNumbers, useSymbols, excludeAmbiguous, passwordLength], generatePassword)

onMounted(() => {
  generateUuids()
  generatePassword()
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
        </div>
      </aside>

      <main class="order-1 md:order-2 flex-1 min-w-0 flex flex-col p-4 md:p-6">
      <div class="mx-auto w-full max-w-6xl flex flex-col gap-4">
        <div class="ui-panel p-4 flex items-center justify-between gap-3">
          <div class="flex items-center gap-2">
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="6" width="14" height="9" rx="1.5" /><path stroke-linecap="round" d="M6.5 6V4.5a3.5 3.5 0 017 0V6" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">UUID &amp; Password Generator</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">UUID v4</span>
              <span class="text-xs font-mono text-[var(--ink-3)]">{{ uuids.length }} generated</span>
            </div>
            <div class="px-4 pb-4 flex flex-col gap-3">
              <div class="flex flex-wrap items-center gap-3">
                <label class="flex items-center gap-1.5 text-xs text-[var(--ink-3)]">
                  Count
                  <input v-model.number="uuidCount" type="number" min="1" :max="MAX_UUID_COUNT" class="ui-input !py-1 !px-2 !w-16 text-xs" @change="generateUuids">
                </label>
                <label class="flex items-center gap-1.5 text-xs text-[var(--ink-3)]">
                  <input v-model="uuidHyphens" type="checkbox" class="ui-range" @change="generateUuids"> Hyphens
                </label>
                <label class="flex items-center gap-1.5 text-xs text-[var(--ink-3)]">
                  <input v-model="uuidUppercase" type="checkbox" class="ui-range" @change="generateUuids"> Uppercase
                </label>
                <button class="ui-button !py-1.5 !px-3 text-xs ml-auto" @click="generateUuids">Regenerate</button>
              </div>

              <div class="overflow-hidden" style="border: 1px solid var(--border)">
                <div v-if="!uuids.length" class="p-4 text-sm text-[var(--ink-3)]" style="background: var(--surface-1)">No UUIDs yet.</div>
                <div v-else class="max-h-72 overflow-auto divide-y" style="border-color: var(--border)">
                  <div v-for="(u, i) in uuids" :key="i" class="flex items-center justify-between gap-2 px-3 py-2" style="background: var(--surface-1)">
                    <span class="text-xs font-mono truncate text-[var(--ink-1)]">{{ u }}</span>
                    <button class="ui-button-secondary !py-1 !px-2.5 text-xs shrink-0" @click="copyUuid(u)">Copy</button>
                  </div>
                </div>
              </div>
              <button class="ui-button-secondary w-full !py-2 text-xs" :disabled="!uuids.length" @click="copyAllUuids">Copy all</button>
            </div>
          </section>

          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Password</span>
              <span class="text-xs font-mono font-semibold" :style="{ color: passwordStrength.color }">{{ passwordStrength.label }}</span>
            </div>
            <div class="px-4 pb-4 flex flex-col gap-3">
              <div class="flex items-center gap-2">
                <span class="ui-input font-mono text-sm flex-1 truncate select-all">{{ password || '—' }}</span>
                <button class="ui-button-secondary !py-2 !px-3 text-xs shrink-0" @click="copyPassword">Copy</button>
                <button class="ui-button !py-2 !px-3 text-xs shrink-0" @click="generatePassword">New</button>
              </div>

              <label class="flex flex-col gap-1">
                <span class="text-xs font-semibold text-[var(--ink-3)]">Length: {{ passwordLength }}</span>
                <input v-model.number="passwordLength" type="range" :min="MIN_PASSWORD_LENGTH" :max="MAX_PASSWORD_LENGTH" class="ui-range w-full">
              </label>

              <div class="grid grid-cols-2 gap-2">
                <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
                  <input v-model="useUppercase" type="checkbox" class="ui-range"> Uppercase (A-Z)
                </label>
                <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
                  <input v-model="useLowercase" type="checkbox" class="ui-range"> Lowercase (a-z)
                </label>
                <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
                  <input v-model="useNumbers" type="checkbox" class="ui-range"> Numbers (0-9)
                </label>
                <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)]">
                  <input v-model="useSymbols" type="checkbox" class="ui-range"> Symbols (!@#$)
                </label>
                <label class="flex items-center gap-1.5 text-xs text-[var(--ink-2)] col-span-2">
                  <input v-model="excludeAmbiguous" type="checkbox" class="ui-range"> Exclude ambiguous (0, O, 1, l, I)
                </label>
              </div>
            </div>
          </section>
        </div>
      </div>
      </main>
    </div>

    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-3 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span class="status-dot ok w-2 h-2 shrink-0 inline-block"></span>
      <span class="truncate">{{ status }}</span>
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
