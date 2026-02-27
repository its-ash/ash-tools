/* tslint:disable */
/* eslint-disable */

/**
 * Get the number of pages in a PDF document.
 */
export function get_page_count(pdf_bytes: Uint8Array): number;

/**
 * Convert an image into a single-page PDF document.
 * image_bytes: RAW JPEG/PNG bytes (it's best to pass JPEG for PDF)
 * width: input image width in pixels
 * height: input image height in pixels
 */
export function img_to_pdf(image_bytes: Uint8Array, width: number, height: number): Uint8Array;

export function init(): void;

/**
 * Merge multiple PDFs into one and compress based on quality.
 * quality: 0–100 (100 = minimal compression, lower = more aggressive)
 */
export function merge_and_compress(pdf_arrays: any, quality: number): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly get_page_count: (a: number, b: number) => [number, number, number];
    readonly img_to_pdf: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly init: () => void;
    readonly merge_and_compress: (a: any, b: number) => [number, number, number];
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
