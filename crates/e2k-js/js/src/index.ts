import { gunzip } from "fflate";
import { C2k as BaseC2k, P2k as BaseP2k, initSync } from "./e2k_js.js";

let initializePromise: Promise<void> | undefined;

const initialize = async () => {
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

/** 英単語 -> カタカナの変換器。 */
export class C2k extends BaseC2k {
  /** @internal
   * @deprecated C2k.create() を使用してください。もし非async環境で使いたい場合は、`e2k/sync`を使用してください。 */
  constructor(maxLen: number) {
    if (initializePromise === undefined) {
      throw new Error(
        "You must use C2k.create() to create an instance of C2k.",
      );
    }
    super(maxLen);
  }

  /** 初期化処理を行う。 */
  static async create(maxLen: number): Promise<C2k> {
    if (initializePromise === undefined) {
      initializePromise = initialize();
    }
    await initializePromise;
    return new C2k(maxLen);
  }
}

/** 発音 -> カタカナの変換器。 */
export class P2k extends BaseP2k {
  /** @internal
   * @deprecated P2k.create() を使用してください。もし非async環境で使いたい場合は、`e2k/sync`を使用してください。 */
  constructor(maxLen: number) {
    if (initializePromise === undefined) {
      throw new Error(
        "You must use P2k.create() to create an instance of P2k.",
      );
    }
    super(maxLen);
  }

  /** 初期化処理を行う。 */
  static async create(maxLen: number): Promise<P2k> {
    if (initializePromise === undefined) {
      initializePromise = initialize();
    }
    await initializePromise;
    return new P2k(maxLen);
  }
}
