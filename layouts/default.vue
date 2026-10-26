<template>
  <div class="min-h-[100dvh] bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col">
    <Header />
    <main class="flex-1 flex flex-col">
      <slot />
    </main>
  </div>
</template>
