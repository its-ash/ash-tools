<script setup lang="ts">
const currentYear = new Date().getFullYear()

// Kept in sync with the tool list on the home page (pages/index.vue).
const toolGroups = [
  {
    label: 'Media & Files',
    tools: [
      { name: 'Video Studio', href: '/video/' },
      { name: 'Image Editor', href: '/image/' },
      { name: 'ZIP Compressor', href: '/zip/' },
      { name: 'PDF Merger', href: '/pdf/' },
      { name: 'PDF Sign', href: '/pdf-sign/' },
    ],
  },
  {
    label: 'Developer',
    tools: [
      { name: 'JSON Formatter', href: '/json/' },
      { name: 'Code Sandbox', href: '/sandbox/' },
      { name: 'Regex Tester', href: '/regex/' },
      { name: 'Diff Checker', href: '/diff/' },
      { name: 'CSV ⇄ JSON', href: '/csv/' },
      { name: 'Encode / Decode / Hash', href: '/encode/' },
      { name: 'Cron Explainer', href: '/cron/' },
    ],
  },
  {
    label: 'Utilities',
    tools: [
      { name: 'Color Picker', href: '/color/' },
      { name: 'QR Code Generator', href: '/qr/' },
      { name: 'Markdown Previewer', href: '/markdown/' },
      { name: 'UUID & Password Generator', href: '/uuid/' },
      { name: 'Unit Converter', href: '/units/' },
      { name: 'Timestamp Converter', href: '/timestamp/' },
      { name: 'Text Case & Lorem Ipsum', href: '/text/' },
      { name: 'Image ⇄ Base64', href: '/base64-image/' },
      { name: 'LinkedIn Post Formatter', href: '/linkedin/' },
    ],
  },
]

const resources = [
  { name: 'GitHub', href: 'https://github.com/its-ash/ash-tools', external: true },
  { name: 'Sitemap', href: '/sitemap.xml' },
  { name: 'Robots', href: '/robots.txt' },
]
</script>

<template>
  <footer class="mt-16 border-t border-[var(--border)] bg-[var(--surface-1)]">
    <div class="mx-auto max-w-7xl px-4 py-12 md:px-6 md:py-14">
      <div class="grid grid-cols-1 gap-10 md:grid-cols-5">
        <div class="flex flex-col gap-3 md:col-span-2">
          <div class="flex items-center gap-2">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" aria-hidden="true">
              <rect x="1" y="1" width="22" height="22" stroke="var(--ink-1)" stroke-width="2" />
              <path d="M7 12L10.5 15.5L17 8" stroke="var(--ink-1)" stroke-width="2" stroke-linecap="square" stroke-linejoin="miter" />
            </svg>
            <h3 class="text-sm font-extrabold text-[var(--ink-1)]">Ash Tools</h3>
          </div>
          <p class="max-w-xs text-sm leading-relaxed text-[var(--ink-2)]">
            Practical file and media tools that run entirely in your browser. No uploads, no accounts, nothing leaves your device.
          </p>
          <div>
            <h4 class="mb-4 text-xs font-bold uppercase tracking-wide text-[var(--ink-3)]">Resources</h4>
            <ul class="space-y-2.5">
              <li v-for="resource in resources" :key="resource.href">
                <a
                  :href="resource.href"
                  :target="resource.external ? '_blank' : undefined"
                  :rel="resource.external ? 'noopener noreferrer' : undefined"
                  class="text-sm text-[var(--ink-2)] transition-colors duration-150 hover:text-[var(--ink-1)]"
                >
                  {{ resource.name }}
                </a>
              </li>
            </ul>
          </div>
        </div>

        <div v-for="group in toolGroups" :key="group.label">
          <h4 class="mb-4 text-xs font-bold uppercase tracking-wide text-[var(--ink-3)]">{{ group.label }}</h4>
          <ul class="space-y-2.5">
            <li v-for="tool in group.tools" :key="tool.href">
              <NuxtLink
                :to="tool.href"
                class="text-sm text-[var(--ink-2)] transition-colors duration-150 hover:text-[var(--ink-1)]"
              >
                {{ tool.name }}
              </NuxtLink>
            </li>
          </ul>
        </div>
      </div>

      <div class="mt-10 flex flex-col gap-3 border-t border-[var(--border)] pt-6 text-xs mono text-[var(--ink-3)] md:flex-row md:items-center md:justify-between">
        <p>© {{ currentYear }} Ash Tools. Built by
          <a href="https://its-ash.github.io/" target="_blank" rel="noopener noreferrer" class="font-medium text-[var(--ink-2)] hover:text-[var(--ink-1)]">Ashvini Jangid</a>.
        </p>
        <p>Nuxt 3 · Rust · WebAssembly</p>
      </div>
    </div>
  </footer>
</template>
