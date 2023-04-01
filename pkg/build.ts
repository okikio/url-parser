import { init, compress } from "https://deno.land/x/zstd_wasm/deno/zstd.ts";
import { prettyBytes } from "https://deno.land/x/pretty_bytes@v2.0.0/mod.ts";
import { join, dirname } from "https://deno.land/std/path/mod.ts";

const __dirname = new URL(dirname(import.meta.url)).pathname;

console.log({
  __dirname,
  log: join(__dirname, "./hello_wasm_bg.wasm")
})
await init();
const utf8 = compress(await Deno.readFile(join(__dirname, "./hello_wasm_bg.wasm")), 22);

console.log(
  prettyBytes(utf8!.length)
)