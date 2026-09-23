<script setup lang="ts">
</script>

<template>
  <header class="sticky top-0 z-50 border-b border-[var(--border)] bg-[var(--surface-0)]/95 backdrop-blur-sm">
    <div class="mx-auto flex h-16 max-w-7xl items-center justify-between gap-3 px-4 md:px-6">
      <NuxtLink to="/" class="flex shrink-0 items-center gap-2">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" aria-hidden="true">
          <rect x="1" y="1" width="22" height="22" stroke="var(--ink-1)" stroke-width="2" />
          <path d="M7 12L10.5 15.5L17 8" stroke="var(--ink-1)" stroke-width="2" stroke-linecap="square" stroke-linejoin="miter" />
        </svg>
        <span class="hidden text-[15px] font-extrabold tracking-tight text-[var(--ink-1)] sm:inline">Ash Tools</span>
      </NuxtLink>

      <div class="flex min-w-0 flex-1 justify-center">
        <ToolSearch class="w-full max-w-xs" />
      </div>

      <a
        href="https://github.com/its-ash/ash-tools"
        target="_blank"
        rel="noopener noreferrer"
        class="ui-button-secondary hidden shrink-0 !py-2 md:inline-flex"
      >
        GitHub
      </a>
    </div>
  </header>
</template>
