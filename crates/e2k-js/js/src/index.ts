import { gunzip } from "fflate";
import {
  C2k as BaseC2k,
  P2k as BaseP2k,
  type Strategy,
  decompressModel,
  initSync,
} from "./e2k_js.js";
export type { Strategy } from "./e2k_js.js";

let initializePromise: Promise<void> | undefined;
let c2kModelPromise: Promise<Uint8Array> | undefined;
let p2kModelPromise: Promise<Uint8Array> | undefined;

const initialize = () => {
  if (initializePromise === undefined) {
    initializePromise = initializeInner();
  }
  return initializePromise;
};
const initializeInner = async () => {
  if (initializePromise === undefined) {
    const wasm = await import("./e2k_js_bg.wasm.js");
    const unzipped = await new Promise<Uint8Array>((resolve) => {
      gunzip(wasm.default, (err, result) => {
        if (err) {
          throw err;
        }
        resolve(result);
      });
    });
    initSync({ module: await WebAssembly.compile(unzipped) });
  }
};

const loadC2kModel = async () => {
  const data = await import("./models/model-c2k.safetensors.br.js");
  await initialize();
  return decompressModel(data.default);
};

const loadP2kModel = async () => {
  const data = await import("./models/model-p2k.safetensors.br.js");
  await initialize();
  return decompressModel(data.default);
};

/** 英単語 -> カタカナの変換器。 */
export class C2k {
  #inner: BaseC2k;

  /** @internal
   * @deprecated C2k.create() を使用してください。もし非async環境で使いたい場合は、`e2k/sync`を使用してください。 */
  constructor(model: Uint8Array, maxLen: number) {
    if (initializePromise === undefined) {
      throw new Error(
        "You must use C2k.create() to create an instance of C2k.",
      );
    }

    this.#inner = new BaseC2k(model, maxLen);
  }

  /** 初期化処理を行う。 */
  static async create(maxLen: number): Promise<C2k> {
    await initialize();
    if (c2kModelPromise === undefined) {
      c2kModelPromise = loadC2kModel();
    }
    await initializePromise;
    return new C2k(await c2kModelPromise, maxLen);
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

/** 発音 -> カタカナの変換器。 */
export class P2k {
  #inner: BaseP2k;

  /** @internal
   * @deprecated P2k.create() を使用してください。もし非async環境で使いたい場合は、`e2k/sync`を使用してください。 */
  constructor(model: Uint8Array, maxLen: number) {
    if (initializePromise === undefined) {
      throw new Error(
        "You must use P2k.create() to create an instance of P2k.",
      );
    }

    this.#inner = new BaseP2k(model, maxLen);
  }

  /** 初期化処理を行う。 */
  static async create(maxLen: number): Promise<P2k> {
    await initialize();
    if (p2kModelPromise === undefined) {
      p2kModelPromise = loadP2kModel();
    }
    await initializePromise;
    return new P2k(await p2kModelPromise, maxLen);
  }

  /**
   * 推論を行う。
   *
   * @param {Array<string>} pronunciation 変換元の発音。CMUdictのフォーマットに従う。
   */
  infer(pronunciation: string[]): string {
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
