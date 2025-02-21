import "./runtime.ts";
import { P2k as BaseP2k, decompressModel, Strategy } from "../e2k_js.js";
import compressed from "../models/model-p2k.safetensors.br.js";
export { type Strategy } from "../e2k_js.js";

/** 英単語 -> カタカナの変換器。 */
export class P2k {
  #inner: BaseP2k;

  /** 初期化処理を行う。 */
  constructor(maxLen: number) {
    const model = decompressModel(compressed);
    this.#inner = new BaseP2k(model, maxLen);
  }

  /**
   * 推論を行う。
   *
   * @param {Array<string>} pronunciation 変換元の発音。CMUdictのフォーマットに従う。
   */
  infer(pronunciation: Array<string>): string {
    return this.#inner.infer(pronunciation);
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
