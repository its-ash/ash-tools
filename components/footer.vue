<script setup lang="ts">
const currentYear = new Date().getFullYear()

const tools = [
  { name: 'ZIP Compressor', href: '/zip/' },
  { name: 'Image Editor', href: '/image/' },
  { name: 'Video Studio', href: '/video/' },
  { name: 'Regex Generator', href: '/regexp/' },
  { name: 'Speed Runner', href: '/speed/' },
  { name: 'Code Sandbox', href: '/sandbox/' },
  { name: 'WebLLM Chat', href: '/webllm/' },
  { name: 'Resume Optimizer', href: '/resume/' },
]

const resources = [
  { name: 'GitHub', href: 'https://github.com/its-ash/ash-tools', external: true },
  { name: 'Sitemap', href: '/sitemap.xml' },
  { name: 'Robots', href: '/robots.txt' },
]
</script>

<template>
  <footer class="mt-8 border-4 border-black bg-[#FFD93D] text-black shadow-[8px_8px_0px_0px_#000]">
    <div class="max-w-7xl mx-auto px-4 md:px-6">
      <div class="grid grid-cols-1 gap-6 py-8 md:grid-cols-3 md:py-10">
        <div class="flex flex-col gap-4">
          <div class="flex items-center gap-2">
            <span class="inline-block h-3 w-3 border-2 border-black rounded-full bg-[#FF6B6B]"></span>
            <h3 class="border-4 border-black bg-white px-3 py-1 text-lg font-black uppercase tracking-wide shadow-[4px_4px_0px_0px_#000]">Ash Tools</h3>
          </div>
          <p class="border-4 border-black bg-white p-4 text-sm leading-relaxed shadow-[4px_4px_0px_0px_#000]">
            A collection of powerful, privacy-focused web tools built with WebAssembly. Process everything locally in your browser. No uploads, 100% private.
          </p>
        </div>

        <div>
          <h4 class="mb-4 inline-block border-4 border-black bg-[#FF6B6B] px-3 py-1 text-sm font-black uppercase tracking-[0.2em] shadow-[4px_4px_0px_0px_#000]">Tools</h4>
          <ul class="space-y-2">
            <li v-for="tool in tools" :key="tool.href">
              <NuxtLink
                :to="tool.href"
                class="inline-block border-4 border-transparent px-2 py-1 text-sm font-bold uppercase tracking-wide transition-all duration-100 ease-linear hover:border-black hover:bg-white hover:shadow-[4px_4px_0px_0px_#000]"
              >
                {{ tool.name }}
              </NuxtLink>
            </li>
          </ul>
        </div>

        <div>
          <h4 class="mb-4 inline-block border-4 border-black bg-[#C4B5FD] px-3 py-1 text-sm font-black uppercase tracking-[0.2em] shadow-[4px_4px_0px_0px_#000]">Resources</h4>
          <ul class="space-y-2">
            <li v-for="resource in resources" :key="resource.href">
              <a
                :href="resource.href"
                :target="resource.external ? '_blank' : undefined"
                :rel="resource.external ? 'noopener noreferrer' : undefined"
                class="inline-block border-4 border-transparent px-2 py-1 text-sm font-bold uppercase tracking-wide transition-all duration-100 ease-linear hover:border-black hover:bg-white hover:shadow-[4px_4px_0px_0px_#000]"
              >
                {{ resource.name }}
                <span v-if="resource.external" class="ml-1">↗</span>
              </a>
            </li>
          </ul>
        </div>
      </div>

      <div class="mb-6 border-4 border-black bg-white p-4 shadow-[4px_4px_0px_0px_#000]">
        <p class="text-xs font-bold text-center md:text-left">
          © {{ currentYear }} Ash Tools. All rights reserved. Built with 
          <span class="font-black">WebAssembly</span> • 
          <span class="font-black">Privacy First</span>
        </p>

        <div class="mt-4 flex flex-wrap items-center justify-center gap-3 text-xs md:justify-start">
          <span class="border-4 border-black bg-[#FFFDF5] px-2 py-1 font-black uppercase tracking-wide shadow-[4px_4px_0px_0px_#000]">Nuxt 3</span>
          <span class="border-4 border-black bg-[#FFFDF5] px-2 py-1 font-black uppercase tracking-wide shadow-[4px_4px_0px_0px_#000]">Tailwind</span>
          <span class="border-4 border-black bg-[#FFFDF5] px-2 py-1 font-black uppercase tracking-wide shadow-[4px_4px_0px_0px_#000]">WASM</span>
          <span class="border-4 border-black bg-[#FFFDF5] px-2 py-1 font-black uppercase tracking-wide shadow-[4px_4px_0px_0px_#000]">VueJS</span>
        </div>
      </div>

      <div class="mb-8 border-4 border-black bg-white p-4 text-center text-xs font-bold leading-relaxed shadow-[4px_4px_0px_0px_#000] md:text-left">
        <p>
          Built with ❤️ using WebAssembly, Rust, and modern web technologies.<br>
          Created by <a href="https://its-ash.github.io/" target="_blank" rel="noopener noreferrer" class="border-b-4 border-black bg-[#C4B5FD] px-1 py-0.5 font-black uppercase tracking-wide">Ashvini Jangid</a> •
          <a href="https://github.com/its-ash/ash-tools" target="_blank" rel="noopener noreferrer" class="border-b-4 border-black bg-[#FF6B6B] px-1 py-0.5 font-black uppercase tracking-wide">View on GitHub</a> •
          <a href="https://ash-tools.store/" target="_blank" rel="noopener noreferrer" class="border-b-4 border-black bg-[#FFD93D] px-1 py-0.5 font-black uppercase tracking-wide">Learn More</a>
        </p>
      </div>
    </div>
  </footer>
</template>

<style scoped>
/* Global Tailwind CSS is loaded from assets/css/main.css */
</style>
