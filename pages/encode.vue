<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'Free Encode/Decode & Hash Tool | Base64, URL, Hex, SHA in your browser',
  meta: [
    { name: 'description', content: 'Free, private, in-browser encode/decode and hashing tool. Base64, URL, hex, and SHA/MD5-style hashes computed locally. No uploads, works offline.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/encode/' },
    { property: 'og:title', content: 'Free Encode/Decode & Hash Tool | Base64, URL, Hex, SHA' },
    { property: 'og:description', content: 'Encode, decode, and hash text privately in your browser. Nothing leaves your device.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/encode/' }],
})

useToolSchema({
  name: 'Encode / Decode / Hash Tool',
  description: 'Base64, URL, hex encoding and SHA/MD5-style hashing, locally in your browser.',
  url: 'https://ash-tools.store/encode/',
  category: 'DeveloperApplication',
})

type EncodingMode = 'base64' | 'url' | 'hex'

const mode = ref<EncodingMode>('base64')
const decoded = ref('')
const encoded = ref('')
const status = ref('Type in either side — the other updates automatically.')
const isError = ref(false)

// Tracks which pane the user is actively typing in, so the watcher pair
// below only pushes the derived value into the *other* pane and never
// fights the field currently under the cursor.
const activeSide = ref<'decoded' | 'encoded' | null>(null)
let suppress = false

const hashes = ref<{ algo: string; value: string }[]>([])
const hashing = ref(false)

const textToBytes = (text: string) => new TextEncoder().encode(text)
const bytesToText = (bytes: Uint8Array) => new TextDecoder().decode(bytes)

const bytesToHex = (bytes: Uint8Array) => Array.from(bytes).map((b) => b.toString(16).padStart(2, '0')).join('')

const hexToBytes = (hex: string) => {
  const clean = hex.trim().replace(/\s+/g, '')
  if (clean.length % 2 !== 0) throw new Error('Hex string must have an even number of digits.')
  if (!/^[0-9a-fA-F]*$/.test(clean)) throw new Error('Invalid hex characters.')
  const bytes = new Uint8Array(clean.length / 2)
  for (let i = 0; i < clean.length; i += 2) bytes[i / 2] = parseInt(clean.slice(i, i + 2), 16)
  return bytes
}

const base64Encode = (bytes: Uint8Array) => {
  let binary = ''
  bytes.forEach((b) => { binary += String.fromCharCode(b) })
  return btoa(binary)
}

const base64Decode = (value: string) => {
  const binary = atob(value.trim())
  const bytes = new Uint8Array(binary.length)
  for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i)
  return bytes
}

const encodeValue = (value: string): string => {
  if (mode.value === 'base64') return base64Encode(textToBytes(value))
  if (mode.value === 'url') return encodeURIComponent(value)
  return bytesToHex(textToBytes(value))
}

const decodeValue = (value: string): string => {
  if (mode.value === 'base64') return bytesToText(base64Decode(value))
  if (mode.value === 'url') return decodeURIComponent(value)
  return bytesToText(hexToBytes(value))
}

// Re-derive the encoded pane from the decoded pane (used when typing into
// "Decoded", and when the format tab changes).
const runEncode = () => {
  if (!decoded.value) {
    suppress = true
    encoded.value = ''
    isError.value = false
    status.value = 'Type in either side — the other updates automatically.'
    return
  }
  try {
    const next = encodeValue(decoded.value)
    suppress = true
    encoded.value = next
    isError.value = false
    status.value = 'Encoded.'
  } catch (error) {
    isError.value = true
    status.value = error instanceof Error ? error.message : 'Could not encode input.'
  }
}

// Re-derive the decoded pane from the encoded pane (used when typing into
// "Encoded").
const runDecode = () => {
  if (!encoded.value) {
    suppress = true
    decoded.value = ''
    isError.value = false
    status.value = 'Type in either side — the other updates automatically.'
    return
  }
  try {
    const next = decodeValue(encoded.value)
    suppress = true
    decoded.value = next
    isError.value = false
    status.value = 'Decoded.'
  } catch (error) {
    isError.value = true
    status.value = error instanceof Error ? error.message : 'Could not decode input — check the format.'
  }
}

watch(decoded, () => {
  if (suppress) { suppress = false; return }
  activeSide.value = 'decoded'
  runEncode()
})

watch(encoded, () => {
  if (suppress) { suppress = false; return }
  activeSide.value = 'encoded'
  runDecode()
})

// Format changed: re-derive from whichever side the user last touched, so
// switching Base64 -> Hex re-encodes the plain text rather than
// re-interpreting stale encoded output.
watch(mode, () => {
  isError.value = false
  if (activeSide.value === 'encoded') runDecode()
  else runEncode()
})

const copyDecoded = async () => {
  if (!decoded.value) return
  try {
    await navigator.clipboard.writeText(decoded.value)
    status.value = 'Decoded value copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const copyEncoded = async () => {
  if (!encoded.value) return
  try {
    await navigator.clipboard.writeText(encoded.value)
    status.value = 'Encoded value copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
}

const clearAll = () => {
  decoded.value = ''
  encoded.value = ''
  activeSide.value = null
  isError.value = false
  hashes.value = []
  status.value = 'Type in either side — the other updates automatically.'
}

const hashAlgorithms = ['SHA-1', 'SHA-256', 'SHA-384', 'SHA-512'] as const

const computeHashes = async () => {
  if (!decoded.value) {
    hashes.value = []
    return
  }
  hashing.value = true
  try {
    const bytes = textToBytes(decoded.value)
    const results: { algo: string; value: string }[] = []
    for (const algo of hashAlgorithms) {
      const digest = await crypto.subtle.digest(algo, bytes)
      results.push({ algo, value: bytesToHex(new Uint8Array(digest)) })
    }
    hashes.value = results
  } catch (error) {
    status.value = error instanceof Error ? error.message : 'Could not compute hashes.'
  } finally {
    hashing.value = false
  }
}

const copyHash = async (value: string) => {
  try {
    await navigator.clipboard.writeText(value)
    status.value = 'Hash copied to clipboard.'
  } catch {
    status.value = 'Could not copy — select and copy manually.'
  }
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
            <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 10h12M4 10l3-3M4 10l3 3M16 10l-3-3M16 10l-3 3" /></svg>
            <h1 class="text-sm font-bold text-[var(--ink-1)]">Encode / Decode / Hash</h1>
          </div>
          <span class="text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
        </div>

        <div class="ui-panel p-3 flex flex-wrap items-center gap-2">
          <div class="flex items-center gap-1 p-1" style="background: var(--surface-2); border: 1px solid var(--border)">
            <button
              v-for="option in (['base64', 'url', 'hex'] as EncodingMode[])"
              :key="option"
              class="px-3 py-1.5 text-xs font-semibold transition-colors duration-150"
              :class="mode === option ? 'ui-button !py-1.5 !px-3' : 'text-[var(--ink-3)] hover:text-[var(--ink-1)]'"
              @click="mode = option"
            >
              {{ option === 'base64' ? 'Base64' : option === 'url' ? 'URL' : 'Hex' }}
            </button>
          </div>
          <span class="text-xs text-[var(--ink-3)]">Type in either pane — the other updates as you go.</span>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Decoded</span>
              <div class="flex items-center gap-2">
                <span class="text-xs font-mono text-[var(--ink-3)]">{{ decoded.length }} chars</span>
                <button class="ui-button-secondary !py-1 !px-2.5 text-xs" :disabled="!decoded" @click="copyDecoded">Copy</button>
              </div>
            </div>
            <div class="px-4 pb-4 flex-1 min-h-0">
              <textarea
                v-model="decoded"
                spellcheck="false"
                placeholder="Type or paste plain text here..."
                class="ui-input font-mono text-xs w-full h-full min-h-[220px] resize-none"
                :class="isError && activeSide === 'encoded' ? 'text-red-600' : ''"
              />
            </div>
          </section>

          <section class="ui-panel flex flex-col min-h-0">
            <div class="flex items-center justify-between px-4 pt-3 pb-2">
              <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Encoded</span>
              <div class="flex items-center gap-2">
                <span class="text-xs font-mono text-[var(--ink-3)]">{{ encoded.length }} chars</span>
                <button class="ui-button-secondary !py-1 !px-2.5 text-xs" :disabled="!encoded" @click="copyEncoded">Copy</button>
              </div>
            </div>
            <div class="px-4 pb-4 flex-1 min-h-0">
              <textarea
                v-model="encoded"
                spellcheck="false"
                :placeholder="mode === 'base64' ? 'Type or paste Base64 here...' : mode === 'url' ? 'Type or paste a URL-encoded string here...' : 'Type or paste hex here...'"
                class="ui-input font-mono text-xs w-full h-full min-h-[220px] resize-none"
                :class="isError && activeSide === 'decoded' ? 'text-red-600' : ''"
              />
            </div>
          </section>
        </div>

        <section class="ui-panel flex flex-col min-h-0">
          <div class="flex items-center justify-between px-4 pt-3 pb-2">
            <span class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)]">Hash digest (of decoded text)</span>
            <button class="ui-button-secondary !py-1 !px-2.5 text-xs" :disabled="!decoded || hashing" @click="computeHashes">
              {{ hashing ? 'Computing...' : 'Compute hashes' }}
            </button>
          </div>
          <div class="px-4 pb-4">
            <div v-if="!hashes.length" class="p-3 text-sm text-[var(--ink-3)]" style="background: var(--surface-1); border: 1px solid var(--border)">
              No hashes computed yet.
            </div>
            <div v-else class="overflow-hidden divide-y" style="border: 1px solid var(--border); border-color: var(--border)">
              <div v-for="h in hashes" :key="h.algo" class="flex items-center justify-between gap-3 px-3 py-2" style="background: var(--surface-1)">
                <span class="text-xs font-mono font-semibold w-20 shrink-0 text-[var(--ink-2)]">{{ h.algo }}</span>
                <span class="text-xs font-mono truncate flex-1 text-[var(--ink-1)]">{{ h.value }}</span>
                <button class="ui-button-secondary !py-1 !px-2.5 text-xs shrink-0" @click="copyHash(h.value)">Copy</button>
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
