# e2k-rs [![Crates.io Version](https://img.shields.io/crates/v/e2k)](https://crates.io/crates/e2k) [![docs.rs](https://img.shields.io/docsrs/e2k)](https://docs.rs/e2k)

英単語から読みを推論するライブラリ。
[Patchethium氏のe2k](https://github.com/Patchethium/e2k)をRustに移植したものです。

## 使い方

```rust
// 文字列をカタカナに変換する例
let src = "constants";
let c2k = e2k::C2k::new(32);
let dst = c2k.infer(src);

dbg!(dst); // "コンスタンツ"
```

```rust
// CMUDictの発音記号をカタカナに変換する例
let pronunciation = ["K", "AA1", "N", "S", "T", "AH0", "N", "T", "S"];
let p2k = e2k::P2k::new(32);
let dst = p2k.infer(&pronunciation);
dbg!(dst); // "コンスタンツ"
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
