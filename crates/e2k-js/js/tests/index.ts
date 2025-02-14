import test from "node:test";
import { C2k, P2k } from "../dist/index.js";

test("C2k", async () => {
  const c2k = await C2k.create(100);
  c2k.setDecodeStrategy({ type: "greedy" });
  console.log(c2k.infer("greedy"));
});

test("P2k", async () => {
  const p2k = await P2k.create(32);
  const pronunciation = ["K", "AA1", "N", "S", "T", "AH0", "N", "T", "S"];
  const dst = p2k.infer(pronunciation);
  console.log(dst);
});
