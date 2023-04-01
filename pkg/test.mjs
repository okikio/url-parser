import init, { new_url, set_pathname_url, get_host_url } from "./hello_wasm.js";

await init();
const url_id = new_url("http://google.com:80\\\\@yahoo.com/");
set_pathname_url(url_id, "./cool/there/is/indeed a path");
console.log({
  log: get_host_url(url_id) // "http://google.com:80\\\\@yahoo.com/"
})