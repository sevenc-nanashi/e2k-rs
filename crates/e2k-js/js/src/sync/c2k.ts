import "./runtime.ts";
import { C2k as BaseC2k, type Strategy, decompressModel } from "../e2k_js.js";
import compressed from "../models/model-c2k.safetensors.br.js";
export type { Strategy } from "../e2k_js.js";

/** 英単語 -> カタカナの変換器。 */
export class C2k {
  #inner: BaseC2k;

  /** 初期化処理を行う。 */
  constructor(maxLen: number) {
    const model = decompressModel(compressed);
    this.#inner = new BaseC2k(model, maxLen);
  }

  /**
   * 推論を行う。
   *
   * @param {string} src 変換元の文字列
   */
  infer(src: string): string {
    return this.#inner.infer(src);
  }

  /**
   * アルゴリズムを設定する。
   *
   * @param {Strategy} strategy アルゴリズム
   */
  setDecodeStrategy(strategy: Strategy): void {
    this.#inner.setDecodeStrategy(strategy);
  }
}
