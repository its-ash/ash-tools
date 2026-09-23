<script setup lang="ts">
definePageMeta({ layout: 'tools' })

useHead({
  title: 'WASM Video Studio | Trim, crop, compress locally',
  meta: [
    { 'http-equiv': 'Cross-Origin-Opener-Policy', content: 'same-origin' },
    { 'http-equiv': 'Cross-Origin-Embedder-Policy', content: 'require-corp' },
    { name: 'description', content: 'Private, in-browser video trimming, cropping, and compression powered by WebAssembly. Process everything locally with no uploads.' },
    { name: 'robots', content: 'index,follow' },
    { property: 'og:url', content: 'https://ash-tools.store/video/' },
    { property: 'og:title', content: 'WASM Video Studio | Trim, crop, compress locally' },
    { property: 'og:description', content: 'Trim, crop, and compress video privately in your browser. WebAssembly powered, no uploads.' },
    { name: 'twitter:card', content: 'summary_large_image' },
  ],
  link: [{ rel: 'canonical', href: 'https://ash-tools.store/video/' }],
})

useToolSchema({
  name: 'WASM Video Studio',
  description: 'Trim, crop, and compress video locally in your browser using WebAssembly.',
  url: 'https://ash-tools.store/video/',
  category: 'MultimediaApplication',
})

let scriptEl: HTMLScriptElement | null = null

onMounted(() => {
  scriptEl = document.createElement('script')
  scriptEl.type = 'module'
  scriptEl.src = `/video/main.js?t=${Date.now()}`
  document.head.appendChild(scriptEl)
})

onUnmounted(() => {
  scriptEl?.remove()
  scriptEl = null
})
</script>

<template>
  <div class="min-h-screen bg-[var(--surface-0)] text-[var(--ink-1)] flex flex-col overflow-hidden">
    <!-- Top bar -->
    <div class="relative h-15 shrink-0 flex items-center justify-between px-5 gap-4" style="background: var(--surface-1); border-bottom: 1px solid var(--border)">
      <div class="text-sm font-bold flex items-center gap-2 text-[var(--ink-1)]">
        <svg class="h-4 w-4 text-[var(--ink-3)]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="2.5" y="4.5" width="11" height="11" rx="2" /><path stroke-linecap="round" stroke-linejoin="round" d="M13.5 8.5l4-2.5v8l-4-2.5" /></svg>
        Video Studio
      </div>
      <span class="hidden sm:inline text-xs font-mono text-[var(--ink-3)]">local · offline · no upload</span>
      <input id="videoInput" type="file" accept="video/mp4,video/webm,video/ogg,video/mov,video/avi,video/mkv,video/flv,video/wmv,video/m4v,video/3gp,video/*" multiple class="hidden">
    </div>

    <!-- Main area: rail + editor + sidebar -->
    <div class="relative flex flex-1 overflow-hidden flex-col md:flex-row">

      <!-- Left rail: navigation + open action -->
      <aside class="order-2 md:order-1 shrink-0 border-t md:border-t-0 md:border-r md:w-[76px] w-full" style="background: var(--surface-1); border-color: var(--border)">
        <div class="flex md:flex-col items-center gap-1 p-2 md:py-4 overflow-x-auto md:overflow-visible">
          <NuxtLink to="/" class="rail-btn shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Back to home" aria-label="Back to home">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.6"><path stroke-linecap="round" stroke-linejoin="round" d="M11.5 4L6 9.5l5.5 5.5" /></svg>
            <span class="text-[10px] font-medium">Home</span>
          </NuxtLink>
          <label for="videoInput" class="rail-btn cursor-pointer shrink-0 w-14 h-14 rounded-[var(--radius-md)] flex flex-col items-center justify-center gap-1 text-[var(--ink-3)]" title="Open video" aria-label="Open video">
            <svg class="h-[18px] w-[18px]" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5"><path stroke-linecap="round" stroke-linejoin="round" d="M4 7a2 2 0 012-2h2l1.5 2H14a2 2 0 012 2v5a2 2 0 01-2 2H6a2 2 0 01-2-2V7z" /><path stroke-linecap="round" d="M10 9v5M8.5 10.5L10 9l1.5 1.5" /></svg>
            <span class="text-[10px] font-medium">Open</span>
          </label>
        </div>
      </aside>

      <!-- Editor area (1fr) -->
      <div class="order-1 md:order-2 flex flex-col flex-1 min-w-0 overflow-hidden">
        <!-- Preview section -->
        <div class="flex-1 flex items-center justify-center p-6 relative" style="background: var(--ink-1); border-bottom: 1px solid var(--border)">
          <!-- Empty state -->
          <div id="emptyState" class="text-center flex flex-col items-center gap-4">
            <h2 class="text-2xl font-bold text-white">Start by opening a video</h2>
            <p class="text-xs text-white/70 leading-relaxed">Trim, crop, and compress your videos locally.<br>No uploads · 100% private · WebAssembly powered</p>
            <p class="text-xs text-white/70"><strong>Supported formats:</strong> MP4, WebM, MOV, AVI, MKV, FLV, WMV, M4V, 3GP</p>
            <p class="text-xs text-white/70">You can select multiple files for batch processing</p>
          </div>

          <!-- Video shell (hidden initially) -->
          <div class="video-shell hidden relative w-full h-full max-w-full max-h-full overflow-hidden" id="videoShell" style="border: 1px solid #fff">
            <video id="preview" controls playsinline muted class="block w-full h-full object-contain"></video>
            <div id="cropOverlay" class="crop-overlay hidden absolute cursor-move" style="display:none;">
              <div class="crop-handle tl absolute -top-1.5 -left-1.5 cursor-nw-resize" data-handle="tl"></div>
              <div class="crop-handle tr absolute -top-1.5 -right-1.5 cursor-ne-resize" data-handle="tr"></div>
              <div class="crop-handle bl absolute -bottom-1.5 -left-1.5 cursor-sw-resize" data-handle="bl"></div>
              <div class="crop-handle br absolute -bottom-1.5 -right-1.5 cursor-se-resize" data-handle="br"></div>
            </div>
          </div>
        </div>

        <!-- Timeline section -->
        <div class="h-48 flex flex-col p-4 gap-3" style="background: var(--surface-1); border-top: 1px solid var(--border)">
          <!-- Player controls -->
          <div class="flex items-center gap-4 px-4 py-3" style="background: var(--surface-2); border: 1px solid var(--border)">
            <button id="playBtn" class="control-btn play w-11 h-11 flex items-center justify-center font-bold transition-all">▶</button>
            <div id="timeDisplay" class="font-mono text-xs text-[var(--ink-1)] font-semibold min-w-32 tracking-wide">00:00 / 00:00</div>
            <div class="flex items-center gap-2 ml-auto">
              <button id="volumeBtn" class="control-btn w-9 h-9 flex items-center justify-center transition-all">🔊</button>
              <div class="volume-slider w-24 h-1 cursor-pointer relative" style="background: var(--surface-3); border: 1px solid var(--border)">
                <div id="volumeFill" class="volume-fill h-full" style="width: 100%;"></div>
              </div>
            </div>
          </div>

          <!-- Timeline -->
          <div class="flex flex-col gap-1.5 flex-1">
            <!-- Seek bar -->
            <div id="seekBar" class="seek-bar h-2 cursor-pointer relative overflow-hidden group" style="background: var(--surface-3); border: 1px solid var(--border)">
              <div id="seekProgress" class="seek-progress h-full relative" style="width: 0%;">
                <div class="seek-handle absolute w-3 h-3 -right-1.5 top-1/2 -translate-y-1/2 group-hover:w-4 group-hover:h-4 transition-all" style="background: var(--surface-1); border: 1px solid var(--ink-1)"></div>
              </div>
            </div>

            <!-- Timeline track -->
            <div class="timeline-track flex-1 relative overflow-hidden cursor-pointer" style="background: var(--surface-2); border: 1px solid var(--border)">
              <div class="timeline-content absolute inset-0 bg-repeat-x"></div>
              <div id="trimStart" class="trim-handle start absolute top-0 bottom-0 w-3.5 cursor-ew-resize z-20"></div>
              <div id="trimEnd" class="trim-handle end absolute top-0 bottom-0 w-3.5 cursor-ew-resize z-20"></div>
              <div id="playhead" class="playhead absolute top-0 bottom-0 w-0.5 z-30" style="left: 0%;"></div>
            </div>

            <!-- Timeline labels -->
            <div class="timeline-labels flex justify-between font-mono text-xs text-[var(--ink-3)] px-2">
              <span id="startLabel">00:00</span>
              <span id="endLabel">00:00</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Sidebar (320px) -->
      <div class="order-3 w-80 overflow-y-auto p-5 flex flex-col gap-4" style="background: var(--surface-1); border-left: 1px solid var(--border)">
        <!-- Export Settings -->
        <div>
          <h3 class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)] mb-3">Export Settings</h3>
          <div class="space-y-3">
            <div class="flex flex-col gap-1.5">
              <label for="resolutionSelect" class="text-xs font-medium text-[var(--ink-2)]">Resolution</label>
              <select id="resolutionSelect" class="ui-input font-mono text-xs">
                <option value="source">Keep original</option>
                <option value="1080">1080p (1920×1080)</option>
                <option value="720">720p (1280×720)</option>
                <option value="480">480p (854×480)</option>
              </select>
            </div>

            <div class="flex flex-col gap-1.5">
              <label for="presetSelect" class="text-xs font-medium text-[var(--ink-2)]">Encoding speed</label>
              <select id="presetSelect" class="ui-input font-mono text-xs">
                <option value="ultrafast">Ultrafast (Fastest)</option>
                <option value="superfast">Superfast</option>
                <option value="veryfast" selected>Very fast</option>
                <option value="faster">Faster</option>
                <option value="medium">Balanced</option>
                <option value="slow">Better quality (Slowest)</option>
              </select>
            </div>

            <div class="flex flex-col gap-1.5">
              <label for="crfInput" class="text-xs font-medium text-[var(--ink-2)]">Quality (CRF) <span id="crfValue" class="text-[var(--ink-1)] font-semibold">25</span></label>
              <input id="crfInput" type="range" min="18" max="32" value="25" class="ui-range w-full">
              <p class="text-xs text-[var(--ink-3)]">Lower = better quality, larger file</p>
            </div>
          </div>
        </div>

        <!-- Trim Settings -->
        <div>
          <h3 class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)] mb-3">Trim Settings</h3>
          <div class="grid grid-cols-2 gap-2">
            <div class="flex flex-col gap-1.5">
              <label for="startInput" class="text-xs font-medium text-[var(--ink-2)]">Start time</label>
              <input id="startInput" type="text" placeholder="00:00" class="ui-input text-xs">
            </div>
            <div class="flex flex-col gap-1.5">
              <label for="endInput" class="text-xs font-medium text-[var(--ink-2)]">End time</label>
              <input id="endInput" type="text" placeholder="Full" class="ui-input text-xs">
            </div>
          </div>
        </div>

        <!-- Crop Settings -->
        <div>
          <h3 class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)] mb-3">Crop Settings</h3>
          <div class="grid grid-cols-2 gap-2 mb-3">
            <div class="flex flex-col gap-1.5">
              <label for="cropWidth" class="text-xs font-medium text-[var(--ink-2)]">Width (px)</label>
              <input id="cropWidth" type="number" min="0" placeholder="Auto" class="ui-input text-xs">
            </div>
            <div class="flex flex-col gap-1.5">
              <label for="cropHeight" class="text-xs font-medium text-[var(--ink-2)]">Height (px)</label>
              <input id="cropHeight" type="number" min="0" placeholder="Auto" class="ui-input text-xs">
            </div>
          </div>
          <div class="grid grid-cols-2 gap-2 mb-2">
            <div class="flex flex-col gap-1.5">
              <label for="cropX" class="text-xs font-medium text-[var(--ink-2)]">X offset</label>
              <input id="cropX" type="number" min="0" placeholder="0" class="ui-input text-xs">
            </div>
            <div class="flex flex-col gap-1.5">
              <label for="cropY" class="text-xs font-medium text-[var(--ink-2)]">Y offset</label>
              <input id="cropY" type="number" min="0" placeholder="0" class="ui-input text-xs">
            </div>
          </div>
          <p class="text-xs text-[var(--ink-3)] mb-3">Drag the overlay on video to crop visually</p>
        </div>

        <!-- Action buttons -->
        <div class="flex flex-col gap-2">
          <button id="processBtn" class="ui-button w-full text-xs from-blue-500 to-cyan-500">Export Video</button>
          <button id="processBatchBtn" class="ui-button-secondary hidden w-full text-xs">Process All Files</button>
        </div>

        <!-- Status and progress -->
        <div class="status-badge inline-flex items-center gap-2 px-3 py-1.5 text-xs font-semibold" id="processingBadge" style="background: var(--surface-2); border: 1px solid var(--border); color: var(--ink-3)">
          <span class="status-dot inline-block w-2 h-2" style="background: var(--ink-3)"></span>
          Idle
        </div>
        <div id="status" class="font-mono text-xs text-[var(--ink-3)] min-h-5"></div>

        <!-- Progress bar -->
        <div id="progress" class="w-full h-5 overflow-hidden opacity-0" style="background: var(--surface-2); border: 1px solid var(--border)">
          <div id="progressBar" class="h-full flex items-center justify-center text-[10px] font-semibold whitespace-nowrap mono transition-all tracking-wide" style="width: 0%; background: var(--accent); color: var(--accent-ink)"></div>
        </div>

        <div id="fileCount" class="font-mono text-xs text-[var(--ink-3)] min-h-5"></div>

        <!-- File list -->
        <div id="fileList" class="hidden mt-3 pt-3" style="border-top: 1px solid var(--border)">
          <h3 class="text-[10px] font-bold uppercase tracking-wider text-[var(--ink-3)] mb-2">Files to process</h3>
          <div id="fileListItems" class="text-xs text-[var(--ink-3)]"></div>
        </div>
      </div>
    </div>

    <!-- Bottom status strip -->
    <div class="shrink-0 border-t px-5 py-2 flex items-center gap-4 text-xs font-mono" style="border-color: var(--border); background: var(--surface-1); color: var(--ink-3)">
      <span>local · offline · no upload</span>
    </div>
  </div>
</template>

<style>
@reference "tailwindcss";

/* Preserve tool-specific interactive selectors referenced by /public/video/main.js */

/* Left rail buttons (nav + open action) */
.rail-btn {
  transition: transform 140ms cubic-bezier(0.23, 1, 0.32, 1), background-color 140ms ease, color 140ms ease;
}

.rail-btn:hover {
  background: var(--surface-2);
  color: var(--ink-1);
}

.rail-btn:active {
  transform: scale(0.94);
}

@media (prefers-reduced-motion: reduce) {
  .rail-btn {
    transition: none !important;
  }
}

.video-shell {
  position: relative;
  max-width: 100%;
  max-height: 100%;
  border-radius: 0;
  overflow: hidden;
}

.video-shell video {
  display: block;
  max-width: 100%;
  max-height: calc(100vh - 348px);
  object-fit: contain;
}

/* Crop overlay and handles */
.crop-overlay {
  position: absolute;
  border: 2px dashed #ffffff;
  border-radius: 0;
  background: rgba(0, 0, 0, 0.2);
  cursor: move;
  box-shadow: 0 0 0 2000px rgba(0, 0, 0, 0.6);
}

.crop-handle {
  position: absolute;
  width: 12px;
  height: 12px;
  background: #ffffff;
  border: 2px solid #000;
  border-radius: 0;
}

.crop-handle.br { bottom: -7px; right: -7px; cursor: se-resize; }
.crop-handle.tr { top: -7px; right: -7px; cursor: ne-resize; }
.crop-handle.bl { bottom: -7px; left: -7px; cursor: sw-resize; }
.crop-handle.tl { top: -7px; left: -7px; cursor: nw-resize; }

/* Timeline player controls */
.control-btn {
  border-radius: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  padding: 0;
  background: var(--surface-1);
  border: 1px solid var(--border-strong);
  color: var(--ink-1);
  cursor: pointer;
  transition: all 0.2s ease;
}

.control-btn:hover {
  background: var(--accent-soft);
  border-color: var(--accent);
}

.control-btn.play {
  width: 42px;
  height: 42px;
  background: var(--accent);
  border: 1px solid var(--border);
  color: var(--accent-ink);
  font-size: 1.125rem;
}

.control-btn.play:hover {
  background: var(--accent-hover);
}

/* Seek bar and progress */
.seek-bar {
  height: 0.5rem;
  border-radius: 0;
  cursor: pointer;
  position: relative;
  overflow: hidden;
}

.seek-progress {
  height: 100%;
  background: var(--accent);
  border-radius: 0;
  position: relative;
}

.seek-handle {
  position: absolute;
  right: -6px;
  top: 50%;
  transform: translateY(-50%);
  width: 12px;
  height: 12px;
  background: var(--surface-1);
  border: 1px solid var(--ink-1);
  border-radius: 0;
}

/* Timeline track and playhead */
.timeline-track {
  height: 3rem;
  border-radius: 0;
  position: relative;
  overflow: hidden;
  cursor: pointer;
}

.timeline-content {
  position: absolute;
  inset: 0;
  background: repeating-linear-gradient(90deg, transparent 0, transparent 9px, var(--border) 9px, var(--border) 10px);
}

.trim-handle {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 14px;
  background: var(--accent);
  cursor: ew-resize;
  z-index: 20;
  border-radius: 0;
  opacity: 0.9;
}

.trim-handle.start { left: 0; }
.trim-handle.end { right: 0; }

.playhead {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 2px;
  background: var(--ink-1);
  z-index: 30;
}

.timeline-labels {
  display: flex;
  justify-content: space-between;
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--ink-3);
}

.timeline-label {
  font-size: 11px;
  color: var(--ink-3);
}

/* Status badge */
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border-radius: 0;
  width: fit-content;
}

.status-badge.active {
  color: var(--ink-1);
  border-color: var(--accent);
  background: var(--accent-soft);
}

.status-badge.active::before {
  background: var(--accent);
}

/* Volume slider */
.volume-slider {
  width: 5rem;
  height: 4px;
  border-radius: 0;
  cursor: pointer;
  position: relative;
}

.volume-fill {
  height: 100%;
  background: var(--accent);
  border-radius: 0;
  width: 100%;
}

/* Progress bar */
.progress {
  width: 100%;
  height: 10px;
  border-radius: 0;
  overflow: hidden;
  opacity: 0;
}

.progress-bar {
  height: 100%;
  width: 0%;
  background: var(--accent);
  transition: width 0.15s ease;
}

/* #processBtn keeps its from- and to- gradient-stop classes so main.js's
   classList.replace("from-blue-500","from-red-500") toggle keeps working;
   these classes now just act as state flags mapped to the new palette. */
#processBtn.from-red-500,
#processBtn.to-orange-500 {
  background: var(--danger);
}

#processBtn.from-red-500:hover,
#processBtn.to-orange-500:hover {
  background: var(--danger);
  opacity: 0.9;
}

/* Responsive */
@media (max-width: 1024px) {
  .video-shell {
    max-height: calc(100vh - 300px);
  }
}
</style>
