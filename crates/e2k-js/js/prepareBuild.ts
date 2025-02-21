import fs from "node:fs";
import { gzipSync } from "fflate";

await fs.promises.rm("./dist", { recursive: true, force: true });

const files = [
  { from: "../pkg/e2k_js_bg.wasm", to: "./src/e2k_js_bg.wasm" },
  {
    from: "../../e2k-rs/src/models/model-c2k.safetensors.br",
    to: "./src/models/model-c2k.safetensors.br",
  },
  {
    from: "../../e2k-rs/src/models/model-p2k.safetensors.br",
    to: "./src/models/model-p2k.safetensors.br",
  },
];
for (const { from, to } of files) {
  console.log(`Building ${to}.js and ${to}.d.ts...`);
  const data = await fs.promises.readFile(from);

  const base64 = Buffer.from(
    from.endsWith(".br") ? data : gzipSync(data, { level: 9 }),
  ).toString("base64");
  await fs.promises.writeFile(
    `${to}.js`,
    `export default Uint8Array.from(atob("${base64}"), c => c.charCodeAt(0));`,
  );
  await fs.promises.writeFile(
    `${to}.d.ts`,
    "declare const data: Uint8Array;export default data;",
  );
}

console.log("Copying e2k_js.js and e2k_js.d.ts...");
await fs.promises.copyFile("../pkg/e2k_js.js", "./src/e2k_js.js");
await fs.promises.copyFile("../pkg/e2k_js.d.ts", "./src/e2k_js.d.ts");
await fs.promises.mkdir("./dist");
await fs.promises.copyFile("../pkg/e2k_js.d.ts", "./dist/e2k_js.d.ts");
