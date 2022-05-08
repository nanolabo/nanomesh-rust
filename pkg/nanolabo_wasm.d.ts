/* tslint:disable */
/* eslint-disable */
/**
*/
export function setup_logging(): void;
/**
* @param {Parameters} parameters
* @param {Uint8Array} bytes
* @returns {Uint8Array}
*/
export function read_obj(parameters: Parameters, bytes: Uint8Array): Uint8Array;
/**
*/
export class Parameters {
  free(): void;
/**
*/
  constructor();
/**
*/
  export_format: number;
/**
*/
  polygon_reduction: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_parameters_free: (a: number) => void;
  readonly __wbg_get_parameters_polygon_reduction: (a: number) => number;
  readonly __wbg_set_parameters_polygon_reduction: (a: number, b: number) => void;
  readonly __wbg_get_parameters_export_format: (a: number) => number;
  readonly __wbg_set_parameters_export_format: (a: number, b: number) => void;
  readonly parameters_new: () => number;
  readonly setup_logging: () => void;
  readonly read_obj: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
