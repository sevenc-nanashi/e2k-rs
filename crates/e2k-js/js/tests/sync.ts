import test from "node:test";
import { C2k, P2k } from "../dist/sync.js";

test("C2k", async () => {
  const c2k = new C2k(100);
  c2k.setDecodeStrategy({ type: "greedy" });
  console.log(c2k.infer("greedy"));
});

test("P2k", async () => {
  const p2k = new P2k(32);
  const pronunciation = ["K", "AA1", "N", "S", "T", "AH0", "N", "T", "S"];
  const dst = p2k.infer(pronunciation);
  console.log(dst);
});
