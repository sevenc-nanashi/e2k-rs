import { gunzipSync } from "fflate";
import { initSync } from "../e2k_js.js";
import wasm from "../e2k_js_bg.wasm.js";

initSync({ module: gunzipSync(wasm) });
