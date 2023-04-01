import init, { new_url, set_pathname_url, get_host_url, get_origin } from "./pkg/hello_wasm.js";
import { source } from "./pkg/wasm_compressed.js"

await init(source());
const url_id = new_url("http://google.com:80\\\\@yahoo.com/");
set_pathname_url(url_id, "./cool/there/is/indeed a path");
console.log({
  log: get_host_url(url_id) // "http://google.com:80\\\\@yahoo.com/"
})