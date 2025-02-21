# e2k-js [![NPM Version](https://img.shields.io/npm/v/%40sevenc-nanashi%2Fe2k)](https://npmjs.org/package/@sevenc-nanashi/e2k) [![jsDocs.io](https://img.shields.io/badge/jsDocs.io-reference-blue)](https://www.jsdocs.io/package/@sevenc-nanashi/e2k)

英単語から読みを推論するライブラリ。
[Patchethium氏のe2k](https://github.com/Patchethium/e2k)をRustに移植した[e2k-rs](https://github.com/sevenc-nanashi/e2k-rs)をwasmにしたものです。

## 使い方

### ロード

以下のエントリーポイントがあります：

- `e2k`：非同期環境用。`C2k`、`P2k`のインスタンス生成までwasmやモデルの読み込みを遅延させます。
- `e2k/sync`：同期環境用。`C2k`、`P2k`のインスタンス生成をする前にwasmやモデルを読み込みます。
- `e2k/sync/c2k`：同期環境用。`C2k`のみを提供します。
- `e2k/sync/p2k`：同期環境用。`P2k`のみを提供します。

```ts
// 文字列をカタカナに変換する例
import { C2k } from "@sevenc-nanashi/e2k";

const src = "constants";
// e2k/syncを使う場合：
// const c2k = new C2k(32);
const c2k = await C2k.create(32);
const dst = c2k.infer(src);

console.log(dst); // "コンスタンツ"
```

```ts
// CMUDictの発音記号をカタカナに変換する例
import { P2k } from "@sevenc-nanashi/e2k";

const pronunciation = ["K", "AA1", "N", "S", "T", "AH0", "N", "T", "S"];
// e2k/syncを使う場合：
// const p2k = new P2k(32);
const p2k = await P2k.create(32);
const dst = p2k.infer(pronunciation);
console.log(dst); // "コンスタンツ"
```

## ライセンス

MIT License にて公開しています。

## 謝辞

モデルはPatchethium氏のものを使用しています。ありがとうございます。

### e2k のライセンス

e2k は WTFPL にて公開されています。

```
           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
                   Version 2, December 2004

Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>

Everyone is permitted to copy and distribute verbatim or modified
copies of this license document, and changing it is allowed as long
as the name is changed.

           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
  TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

 0. You just DO WHAT THE FUCK YOU WANT TO.
```
