<script setup lang="ts">
useHead({
  title: 'On-Device WebLLM Studio',
  meta: [
    { name: 'description', content: 'Chat with WebLLM directly in your browser. Provide context, add prompts, and keep the entire conversation offline.' },
  ],
})

let scriptEl: HTMLScriptElement | null = null

function toggleContext() {
  if (typeof document === 'undefined') return
  const contextField = document.querySelector('.context-field') as HTMLElement | null
  if (contextField) {
    contextField.style.display = contextField.style.display === 'none' ? 'block' : 'none'
  }
}

onMounted(() => {
  scriptEl = document.createElement('script')
  scriptEl.type = 'module'
  scriptEl.src = `/webllm/main.js?t=${Date.now()}`
  document.head.appendChild(scriptEl)
  // expose toggleContext globally for inline onclick
  ;(window as never as Record<string, unknown>).toggleContext = toggleContext
})

onUnmounted(() => {
  scriptEl?.remove()
  scriptEl = null
  delete (window as never as Record<string, unknown>).toggleContext
})
</script>

<template>
  <div id="webllm-page" class="min-h-[calc(100vh-48px)] text-black flex flex-col px-3 pb-3 bg-[#FFFDF5]">
    <div class="w-full max-w-4xl mx-auto flex flex-col min-h-[calc(100vh-68px)] pb-4">

      <!-- Header -->
      <header class="sticky top-2 z-20 border-4 border-black rounded-2xl px-4 py-3 mt-2 mb-3 bg-[#FFD93D] shadow-[8px_8px_0px_0px_#000]">
        <div id="status-indicator" data-state="loading" class="inline-flex items-center gap-2 w-fit px-3 py-1.5 rounded-full border-2 border-black bg-white text-[11px] font-medium text-black mt-2">Model warming up…</div>
        <div class="progress-track h-1 bg-white border border-black rounded-full overflow-hidden mt-2" aria-label="Model download progress">
          <div id="model-progress" class="h-full w-0 bg-[#10a37f] transition-all duration-300"></div>
        </div>
        <small id="progress-label" class="text-black text-[11px] mt-1">Initializing WebLLM…</small>
      </header>

      <!-- History Panel -->
      <section class="flex-1 min-h-0 px-1 md:px-2">
        <div id="history-empty" class="text-black text-sm py-10 text-center">Start a conversation with your local model.</div>
        <ul id="history-list" class="flex flex-col gap-3 overflow-y-auto flex-1" hidden></ul>
      </section>

      <!-- Input Panel -->
      <form id="prompt-form" class="mt-3 w-full border-4 border-black rounded-3xl p-3 md:p-4 shadow-[8px_8px_0px_0px_#000] bg-white">
        <!-- Context Field (Toggle) -->
        <div class="context-field mb-3 hidden">
          <label for="context-input" class="text-[11px] uppercase tracking-wide text-black font-semibold mb-2 block">Context</label>
          <textarea id="context-input" name="context" placeholder="e.g. You are a helpful assistant that speaks in concise checklists." class="w-full min-h-20 rounded-2xl border-4 border-black bg-white text-black text-sm p-3 font-mono resize-y outline-none transition-all"></textarea>
        </div>

        <!-- Prompt -->
        <div class="mb-3">
          <label for="prompt-input" class="sr-only">Prompt</label>
          <textarea id="prompt-input" name="prompt" placeholder="Message WebLLM…" class="w-full min-h-24 rounded-2xl border-4 border-black bg-white text-black text-[15px] p-4 font-sans resize-y outline-none transition-all"></textarea>
        </div>

        <!-- Buttons -->
        <div class="flex items-center gap-2 flex-wrap mb-3">
          <button type="button" @click="toggleContext" class="px-3 py-2 rounded-xl border-2 border-black bg-[#C4B5FD] text-black text-xs md:text-sm font-medium transition-colors">Context</button>
          <button type="button" id="reset-btn" class="px-3 py-2 rounded-xl border-2 border-black bg-[#FFD93D] text-black text-xs md:text-sm font-medium transition-colors">Reset</button>
          <button type="submit" id="submit-btn" class="ml-auto inline-flex items-center justify-center px-5 py-2.5 bg-[#FF6B6B] border-2 border-black text-black rounded-xl font-semibold text-sm transition-colors">Send</button>
        </div>

        <!-- Context Preview -->
        <div>
          <label class="text-[11px] uppercase tracking-wide text-black font-semibold mb-2 block">Active context preview</label>
          <div id="context-preview" class="text-xs text-black font-mono p-2.5 bg-[#FFFDF5] border-4 border-black rounded-xl">No context captured yet.</div>
        </div>
      </form>
    </div>
  </div>
</template>

<style>
#webllm-page {
  background:
    radial-gradient(1200px 450px at 20% -10%, rgba(255, 107, 107, 0.2), transparent 60%),
    radial-gradient(900px 350px at 85% 0%, rgba(196, 181, 253, 0.2), transparent 60%),
    #fffdf5;
}

#webllm-page .message {
  max-width: min(92%, 760px);
  width: fit-content;
  border-radius: 18px;
  padding: 12px 14px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  background: #2f2f2f;
  color: #ececec;
  align-self: flex-start;
}

#webllm-page .message[data-role='user'] {
  margin-left: auto;
  background: #343541;
  border-color: rgba(255, 255, 255, 0.12);
}

#webllm-page .message-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  color: #9ca3af;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 6px;
}

#webllm-page .message-content {
  font-size: 14px;
  line-height: 1.55;
  color: #ececec;
}

#webllm-page .message-content p {
  margin: 0 0 8px;
}

#webllm-page .message-content p:last-child {
  margin-bottom: 0;
}

#webllm-page .code-block {
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 12px;
  overflow: hidden;
  background: #1e1e1e;
  margin-top: 6px;
}

#webllm-page #history-list {
  padding-bottom: 18px;
}

#webllm-page #status-indicator[data-state='ready'] {
  color: #c7f2e6;
  border-color: rgba(16, 163, 127, 0.5);
  background: rgba(16, 163, 127, 0.14);
}

#webllm-page #status-indicator[data-state='loading'] {
  color: #dbeafe;
  border-color: rgba(56, 189, 248, 0.4);
  background: rgba(56, 189, 248, 0.12);
  animation: thinkingPulse 1.6s ease-in-out infinite;
  position: relative;
}

#webllm-page #status-indicator[data-state='loading']::after {
  content: '...';
  margin-left: 2px;
  letter-spacing: 1px;
  animation: thinkingDots 1.2s steps(4, end) infinite;
}

#webllm-page #status-indicator[data-state='error'] {
  color: #fecaca;
  border-color: rgba(248, 113, 113, 0.45);
  background: rgba(127, 29, 29, 0.35);
}

#webllm-page #model-progress {
  background: linear-gradient(90deg, #0ea5e9, #10b981, #14b8a6);
  background-size: 200% 100%;
  animation: progressShimmer 1.8s linear infinite;
}

@keyframes thinkingPulse {
  0%,
  100% {
    box-shadow: 0 0 0 0 rgba(56, 189, 248, 0.28);
    transform: translateY(0);
  }

  50% {
    box-shadow: 0 0 0 6px rgba(56, 189, 248, 0.05);
    transform: translateY(-1px);
  }
}

@keyframes thinkingDots {
  0% {
    content: '.';
  }

  33% {
    content: '..';
  }

  66% {
    content: '...';
  }

  100% {
    content: '';
  }
}

@keyframes progressShimmer {
  from {
    background-position: 0% 0;
  }

  to {
    background-position: 200% 0;
  }
}

@media (max-width: 768px) {
  #webllm-page .message {
    max-width: 97%;
  }
}
</style>

