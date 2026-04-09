<script setup lang="ts">
useHead({
  title: 'Ash Tools | Private Browser Tools for Media, Docs, and Dev',
  meta: [
    { name: 'description', content: 'Ash Tools is a privacy-first toolkit for media editing, PDF and ZIP workflows, AI chat, and developer utilities. Everything runs locally in your browser with no uploads.' },
    { name: 'keywords', content: 'free tools, video compressor, file zipper, online tools, web tools, compression, zip files, video editor, AI chat, code sandbox, rust execution, javascript runner, local processing, offline tools, privacy tools' },
    { property: 'og:type', content: 'website' },
    { property: 'og:url', content: 'https://ash-tools.store/' },
    { property: 'og:title', content: 'Ash Tools | Private Browser Tools for Media, Docs, and Dev' },
    { property: 'og:description', content: 'Privacy-first browser tools for media, docs, AI, and developer workflows. Local processing, no uploads, offline-ready.' },
    { name: 'twitter:card', content: 'summary_large_image' },
    { name: 'robots', content: 'index,follow,max-snippet:-1,max-image-preview:large,max-video-preview:-1' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/' }],
})

const mediaTools = [
  {
    title: 'Video Studio',
    route: '/video',
    icon: '🎬',
    color: '#FF6B6B',
    description: 'Trim, crop, compress, and process videos locally with WebAssembly + FFmpeg.',
    features: ['Trim and crop videos', 'Multiple format support', 'Batch processing', 'Local-first workflow'],
    cta: 'Open Video Studio',
  },
  {
    title: 'Image Editor',
    route: '/image',
    icon: '🖼️',
    color: '#FFD93D',
    description: 'Edit, warp, and optimize images in-browser without file uploads.',
    features: ['Perspective crop tools', 'PNG, JPEG, WebP, HEIC', 'Compression previews', 'Offline-ready'],
    cta: 'Open Image Editor',
  },
  {
    title: 'View and Zip Files',
    route: '/zip',
    icon: '📦',
    color: '#C4B5FD',
    description: 'Create ZIP archives from files and folders with drag-and-drop convenience.',
    features: ['ZIP archive creation', 'Folder compression', 'Drag and drop interface', 'Batch packaging'],
    cta: 'Open ZIP Tool',
  },
  {
    title: 'PDF Merger',
    route: '/pdf',
    icon: '📄',
    color: '#FF6B6B',
    description: 'Merge, reorder, and compress PDF documents securely offline.',
    features: ['Multi-file merging', 'Drag reordering', 'Compression support', 'Thumbnail previews'],
    cta: 'Open PDF Merger',
  },
  {
    title: 'WebLLM Studio',
    route: '/webllm',
    icon: '🧠',
    color: '#C4B5FD',
    description: 'Run local AI chat sessions with custom context and prompts on-device.',
    features: ['Context + prompt workflow', 'Local WebGPU inference', 'Private chat history', 'PWA support'],
    cta: 'Open WebLLM Studio',
  },
]

const devTools = [
  {
    title: 'Speed Runner',
    route: '/speed',
    icon: '⚡',
    color: '#FFD93D',
    description: 'Benchmark JavaScript execution and memory with visual performance output.',
    features: ['Execution time analysis', 'Heap tracking', 'Visual charts', 'Worker isolation'],
    cta: 'Open Speed Runner',
  },
  {
    title: 'Regex Generator',
    route: '/regexp',
    icon: '.*',
    color: '#FF6B6B',
    description: 'Generate optimized regex patterns from examples using Rust-powered logic.',
    features: ['Example-based generation', 'Rust/Grex engine', 'Instant preview', 'Offline processing'],
    cta: 'Open Regex Tool',
  },
  {
    title: 'Code Sandbox',
    route: '/sandbox',
    icon: '🧪',
    color: '#C4B5FD',
    description: 'Run and test multiple languages in an isolated browser execution environment.',
    features: ['Multi-language support', 'WASI runtime', 'stdin support', 'Copy output tools'],
    cta: 'Open Code Sandbox',
  },
]
</script>

<template>
  <div class="mx-auto max-w-7xl px-2 py-10 sm:px-4 md:px-6 md:py-14">
    <header class="neo-shell mb-12 px-5 py-10 text-center md:px-10 md:py-14">
      <p class="inline-block -rotate-2 border-4 border-black bg-[#FFD93D] px-3 py-1 text-xs font-black uppercase tracking-[0.2em] shadow-[4px_4px_0px_0px_#000]">
        Local-First Browser Toolkit
      </p>

      <div class="mt-6">
        <h1 class="neo-outline-title text-5xl leading-[0.9] tracking-tight sm:text-7xl md:text-8xl">ASH TOOLS</h1>
        <h1 class="-mt-7 rotate-1 text-5xl font-black uppercase tracking-tight sm:text-7xl md:text-8xl">ASH TOOLS</h1>
      </div>

      <p class="mx-auto mt-6 max-w-3xl border-4 border-black bg-white p-4 text-lg font-bold leading-relaxed shadow-[8px_8px_0px_0px_#000]">
        A single workspace for everyday file tasks and developer utilities. Edit media, merge documents, run code, and chat with local AI models directly in your browser.
      </p>

      <div class="mt-6 flex flex-wrap justify-center gap-3">
        <span class="neo-badge bg-[#FF6B6B]">100% Local Processing</span>
        <span class="neo-badge bg-[#FFD93D]">No Uploads Required</span>
        <span class="neo-badge bg-[#C4B5FD]">Works Offline</span>
        <span class="neo-badge bg-white">WebAssembly Powered</span>
      </div>
    </header>

    <section class="neo-shell mb-14 bg-[#FFD93D] p-6 md:p-8">
      <h2 class="mb-5 inline-block border-4 border-black bg-white px-3 py-1 text-3xl uppercase">Start in 3 Steps</h2>
      <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
        <div class="border-4 border-black bg-white p-4 shadow-[4px_4px_0px_0px_#000]">
          <p class="text-sm font-black uppercase tracking-wide">1. Pick a tool</p>
          <p class="mt-2 text-sm">Choose from media, docs, AI, or developer utilities below.</p>
        </div>
        <div class="border-4 border-black bg-white p-4 shadow-[4px_4px_0px_0px_#000]">
          <p class="text-sm font-black uppercase tracking-wide">2. Add files or code</p>
          <p class="mt-2 text-sm">Drag files or paste snippets. Processing starts locally.</p>
        </div>
        <div class="border-4 border-black bg-white p-4 shadow-[4px_4px_0px_0px_#000]">
          <p class="text-sm font-black uppercase tracking-wide">3. Export results</p>
          <p class="mt-2 text-sm">Download optimized outputs and keep working in-browser.</p>
        </div>
      </div>
    </section>

    <section class="mb-14">
      <h2 class="inline-block -rotate-1 border-4 border-black bg-[#FF6B6B] px-4 py-2 text-3xl uppercase shadow-[4px_4px_0px_0px_#000]">
        Media and Document Tools
      </h2>
      <p class="mt-4 max-w-3xl border-4 border-black bg-white p-4 text-base shadow-[4px_4px_0px_0px_#000]">
        Use these tools when you need to transform files quickly without cloud uploads.
      </p>

      <div class="mt-6 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
        <article
          v-for="tool in mediaTools"
          :key="tool.route"
          class="neo-card flex h-full flex-col p-5"
        >
          <div class="mb-3 flex items-center gap-3">
            <div
              class="flex h-14 w-14 items-center justify-center border-4 border-black text-2xl shadow-[4px_4px_0px_0px_#000]"
              :style="{ backgroundColor: tool.color }"
            >
              {{ tool.icon }}
            </div>
            <h3 class="text-xl uppercase">{{ tool.title }}</h3>
          </div>

          <p class="mb-4 text-sm leading-relaxed">{{ tool.description }}</p>
          <ul class="mb-5 space-y-1 text-xs font-bold uppercase tracking-wide">
            <li v-for="feature in tool.features" :key="feature">✓ {{ feature }}</li>
          </ul>

          <NuxtLink
            :to="tool.route"
            class="neo-button mt-auto"
            :style="{ backgroundColor: tool.color }"
          >
            {{ tool.cta }}
          </NuxtLink>
        </article>
      </div>
    </section>

    <section>
      <h2 class="inline-block rotate-1 border-4 border-black bg-[#C4B5FD] px-4 py-2 text-3xl uppercase shadow-[4px_4px_0px_0px_#000]">
        Developer Tools
      </h2>
      <p class="mt-4 max-w-3xl border-4 border-black bg-white p-4 text-base shadow-[4px_4px_0px_0px_#000]">
        Build, test, and debug faster with in-browser utilities powered by workers and WASM.
      </p>

      <div class="mt-6 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
        <article
          v-for="tool in devTools"
          :key="tool.route"
          class="neo-card flex h-full flex-col p-5"
        >
          <div class="mb-3 flex items-center gap-3">
            <div
              class="flex h-14 w-14 items-center justify-center border-4 border-black text-2xl font-black shadow-[4px_4px_0px_0px_#000]"
              :style="{ backgroundColor: tool.color }"
            >
              {{ tool.icon }}
            </div>
            <h3 class="text-xl uppercase">{{ tool.title }}</h3>
          </div>

          <p class="mb-4 text-sm leading-relaxed">{{ tool.description }}</p>
          <ul class="mb-5 space-y-1 text-xs font-bold uppercase tracking-wide">
            <li v-for="feature in tool.features" :key="feature">✓ {{ feature }}</li>
          </ul>

          <NuxtLink
            :to="tool.route"
            class="neo-button mt-auto"
            :style="{ backgroundColor: tool.color }"
          >
            {{ tool.cta }}
          </NuxtLink>
        </article>
      </div>
    </section>
  </div>
</template>
