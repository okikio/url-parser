// import { init, compress } from "https://deno.land/x/zstd_wasm/deno/zstd.ts";
import { prettyBytes } from "https://deno.land/x/pretty_bytes@v2.0.0/mod.ts";
import { join, dirname } from "https://deno.land/std/path/mod.ts";
import { encode } from "https://deno.land/std@0.182.0/encoding/base64.ts";

const __dirname = new URL(dirname(import.meta.url)).pathname;
const filePath = join(__dirname, "../pkg/hello_wasm_bg.wasm");

console.log({
  __dirname,
  filePath
})

// await init();

const cs = new CompressionStream('gzip');

const uint8arr = await Deno.readFile(filePath);
const compressedStream = new Blob([uint8arr.buffer]).stream().pipeThrough(cs);
const utf8 = new Uint8Array(await new Response(compressedStream).arrayBuffer()); // compress(uint8arr, 22);
const encoded = encode(utf8);

Deno.writeFile(join(__dirname, "../pkg/wasm_compressed.js"), new TextEncoder().encode(`\
// @ts-nocheck The file is too large to be type checked properly
export const source = async () => {
  const uint8arr = Uint8Array.from(atob("${encoded}"), c => c.charCodeAt(0));
  const ds = new DecompressionStream('gzip');
  const decompressedStream = new Blob([uint8arr.buffer]).stream().pipeThrough(ds);
  return new Uint8Array(await new Response(decompressedStream).arrayBuffer());
};
export default source;
`))

console.log(
  prettyBytes(utf8!.length)
)