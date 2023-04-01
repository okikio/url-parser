/* tslint:disable */
/* eslint-disable */
/**
* @param {string} url
* @returns {number}
*/
export function new_url(url: string): number;
/**
* @param {string} url
* @returns {string}
*/
export function parse_url(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function parse(index: number): string;
/**
* @param {string} url
* @returns {string}
*/
export function get_host(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_host_url(index: number): string;
/**
* @param {string} url
* @returns {string}
*/
export function get_hostname(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_hostname_url(index: number): string;
/**
* @param {string} url
* @returns {string}
*/
export function get_port(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_port_url(index: number): string;
/**
* @param {string} url
* @returns {string}
*/
export function get_domain(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_domain_url(index: number): string;
/**
* @param {string} url
* @returns {string}
*/
export function get_scheme(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_scheme_url(index: number): string;
/**
* @param {string} url
* @returns {string}
*/
export function get_origin(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_origin_url(index: number): string;
/**
* @param {string} url
* @returns {string}
*/
export function get_fragment(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_fragment_url(index: number): string;
/**
* @param {string} url
* @param {string} input
* @returns {string}
*/
export function join_url(url: string, input: string): string;
/**
* @param {number} index
* @param {string} input
* @returns {string}
*/
export function join(index: number, input: string): string;
/**
* @param {string} base
* @param {string} url
* @returns {string}
*/
export function make_relative(base: string, url: string): string;
/**
* @param {number} index
* @param {string} url
* @returns {string}
*/
export function relative_partial(index: number, url: string): string;
/**
* @param {number} index
* @param {number} url_index
* @returns {string}
*/
export function relative(index: number, url_index: number): string;
/**
* @param {string} url
* @returns {string}
*/
export function get_password(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_password_url(index: number): string;
/**
* @param {string} url
* @returns {string}
*/
export function get_pathname(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_pathname_url(index: number): string;
/**
* @param {string} url
* @returns {any}
*/
export function get_path_segments(url: string): any;
/**
* @param {number} index
* @returns {any}
*/
export function get_path_segments_url(index: number): any;
/**
* @param {string} url
* @returns {string}
*/
export function get_query(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_query_url(index: number): string;
/**
* @param {string} url
* @returns {string}
*/
export function get_username(url: string): string;
/**
* @param {number} index
* @returns {string}
*/
export function get_username_url(index: number): string;
/**
* @param {string} url
* @returns {any}
*/
export function get_query_pairs(url: string): any;
/**
* @param {number} index
* @returns {any}
*/
export function get_query_pairs_url(index: number): any;
/**
* @param {string} url
* @param {string} host
* @returns {string}
*/
export function set_host(url: string, host: string): string;
/**
* @param {number} index
* @param {string} host
* @returns {number}
*/
export function set_host_url(index: number, host: string): number;
/**
* @param {string} url
* @param {string} host
* @returns {string}
*/
export function set_hostname(url: string, host: string): string;
/**
* @param {number} index
* @param {string} host
* @returns {number}
*/
export function set_hostname_url(index: number, host: string): number;
/**
* @param {string} url
* @param {number} port
* @returns {string}
*/
export function set_port(url: string, port: number): string;
/**
* @param {number} index
* @param {number} port
* @returns {number}
*/
export function set_port_url(index: number, port: number): number;
/**
* @param {string} url
* @param {string} scheme
* @returns {string}
*/
export function set_scheme(url: string, scheme: string): string;
/**
* @param {number} index
* @param {string} scheme
* @returns {number}
*/
export function set_scheme_url(index: number, scheme: string): number;
/**
* @param {string} url
* @param {string} fragment
* @returns {string}
*/
export function set_fragment(url: string, fragment: string): string;
/**
* @param {number} index
* @param {string} fragment
* @returns {number}
*/
export function set_fragment_url(index: number, fragment: string): number;
/**
* @param {string} url
* @param {string} password
* @returns {string}
*/
export function set_password(url: string, password: string): string;
/**
* @param {number} index
* @param {string} password
* @returns {number}
*/
export function set_password_url(index: number, password: string): number;
/**
* @param {string} url
* @param {string} path
* @returns {string}
*/
export function set_pathname(url: string, path: string): string;
/**
* @param {number} index
* @param {string} path
* @returns {number}
*/
export function set_pathname_url(index: number, path: string): number;
/**
* @param {string} url
* @param {string} path
* @returns {any}
*/
export function set_path_segments(url: string, path: string): any;
/**
* @param {number} index
* @param {string} path
* @returns {any}
*/
export function set_path_segments_url(index: number, path: string): any;
/**
* @param {string} url
* @param {string} query
* @returns {string}
*/
export function set_query(url: string, query: string): string;
/**
* @param {number} index
* @param {string} query
* @returns {number}
*/
export function set_query_url(index: number, query: string): number;
/**
* @param {string} url
* @param {string} username
* @returns {string}
*/
export function set_username(url: string, username: string): string;
/**
* @param {number} index
* @param {string} username
* @returns {number}
*/
export function set_username_url(index: number, username: string): number;
/**
* @param {string} url
* @param {string} query
* @returns {any}
*/
export function set_query_pairs(url: string, query: string): any;
/**
* @param {number} index
* @param {string} query
* @returns {any}
*/
export function set_query_pairs_url(index: number, query: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly new_url: (a: number, b: number, c: number) => void;
  readonly parse_url: (a: number, b: number, c: number) => void;
  readonly parse: (a: number, b: number) => void;
  readonly get_host: (a: number, b: number, c: number) => void;
  readonly get_host_url: (a: number, b: number) => void;
  readonly get_port: (a: number, b: number, c: number) => void;
  readonly get_port_url: (a: number, b: number) => void;
  readonly get_domain: (a: number, b: number, c: number) => void;
  readonly get_domain_url: (a: number, b: number) => void;
  readonly get_scheme: (a: number, b: number, c: number) => void;
  readonly get_scheme_url: (a: number, b: number) => void;
  readonly get_origin: (a: number, b: number, c: number) => void;
  readonly get_origin_url: (a: number, b: number) => void;
  readonly get_fragment: (a: number, b: number, c: number) => void;
  readonly get_fragment_url: (a: number, b: number) => void;
  readonly join_url: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly join: (a: number, b: number, c: number, d: number) => void;
  readonly make_relative: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly relative_partial: (a: number, b: number, c: number, d: number) => void;
  readonly relative: (a: number, b: number, c: number) => void;
  readonly get_password: (a: number, b: number, c: number) => void;
  readonly get_password_url: (a: number, b: number) => void;
  readonly get_pathname: (a: number, b: number, c: number) => void;
  readonly get_pathname_url: (a: number, b: number) => void;
  readonly get_path_segments: (a: number, b: number, c: number) => void;
  readonly get_path_segments_url: (a: number, b: number) => void;
  readonly get_query: (a: number, b: number, c: number) => void;
  readonly get_query_url: (a: number, b: number) => void;
  readonly get_username: (a: number, b: number, c: number) => void;
  readonly get_username_url: (a: number, b: number) => void;
  readonly get_query_pairs: (a: number, b: number, c: number) => void;
  readonly get_query_pairs_url: (a: number, b: number) => void;
  readonly set_host: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly set_host_url: (a: number, b: number, c: number, d: number) => void;
  readonly set_port: (a: number, b: number, c: number, d: number) => void;
  readonly set_port_url: (a: number, b: number, c: number) => void;
  readonly set_scheme: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly set_scheme_url: (a: number, b: number, c: number, d: number) => void;
  readonly set_fragment: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly set_fragment_url: (a: number, b: number, c: number, d: number) => void;
  readonly set_password: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly set_password_url: (a: number, b: number, c: number, d: number) => void;
  readonly set_pathname: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly set_pathname_url: (a: number, b: number, c: number, d: number) => void;
  readonly set_path_segments: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly set_path_segments_url: (a: number, b: number, c: number, d: number) => void;
  readonly set_query: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly set_query_url: (a: number, b: number, c: number, d: number) => void;
  readonly set_username: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly set_username_url: (a: number, b: number, c: number, d: number) => void;
  readonly set_query_pairs: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly set_query_pairs_url: (a: number, b: number, c: number, d: number) => void;
  readonly get_hostname_url: (a: number, b: number) => void;
  readonly set_hostname_url: (a: number, b: number, c: number, d: number) => void;
  readonly get_hostname: (a: number, b: number, c: number) => void;
  readonly set_hostname: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
