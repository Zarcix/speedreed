/* tslint:disable */
/* eslint-disable */
/**
*/
export class SpeedReed {
  free(): void;
/**
*/
  constructor();
/**
* @returns {string}
*/
  get_current_text(): string;
/**
* @returns {bigint}
*/
  get_current_speed(): bigint;
/**
* @param {string} text
*/
  set_current_text(text: string): void;
/**
* @param {string} speed
*/
  set_current_speed(speed: string): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_speedreed_free: (a: number) => void;
  readonly speedreed_new: () => number;
  readonly speedreed_get_current_text: (a: number, b: number) => void;
  readonly speedreed_get_current_speed: (a: number) => number;
  readonly speedreed_set_current_text: (a: number, b: number, c: number) => void;
  readonly speedreed_set_current_speed: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
