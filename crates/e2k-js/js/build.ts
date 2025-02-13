import fs from "node:fs";
import { gzipSync } from "fflate";
import * as tsdown from "tsdown";

await fs.promises.rm("./dist", { recursive: true, force: true });
const wasm = await fs.promises.readFile("../pkg/e2k_js_bg.wasm");
const compressed = gzipSync(wasm, {});
const base64 = Buffer.from(compressed).toString("base64");
await fs.promises.writeFile(
  "./src/e2k_js_bg.wasm.js",
  `export default Uint8Array.from(atob("${base64}"), c => c.charCodeAt(0));`,
);
await fs.promises.writeFile(
  "./src/e2k_js_bg.wasm.d.ts",
  `declare const wasm: Uint8Array;export default wasm;`,
);

await fs.promises.rm("./dist", { recursive: true, force: true });
await tsdown.build({
  entry: {
    "index": "./src/index.ts",
    "sync": "./src/sync.ts",
  },
  outDir: "./dist",
  sourcemap: true,
});
