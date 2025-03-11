# e2k-py

英単語から読みを推論するライブラリ。
[Patchethium氏のe2k](https://github.com/Patchethium/e2k)をRustに移植した[e2k-rs](https://github.com/sevenc-nanashi/e2k-rs)のPyO3バインディングです。

## 使い方

```py
# 文字列をカタカナに変換する例
import e2k_rs

c2k = e2k_rs.C2k()

word = "constants"
print(c2k(word)) # => コンスタンツ
```

```py
# CMUDictの発音記号をカタカナに変換する例
import e2k_rs

p2k = e2k_rs.P2k()

pronunciation = ["K", "AA1", "N", "S", "T", "AH0", "N", "T", "S"]
print(p2k(pronunciation)) # => コンスタンツ
```

## ライセンス

MIT License にて公開しています。

## 謝辞

モデルはPatchethium氏のものを使用しています。ありがとうございます。
また、Rustで依存しているクレートのライセンスはNOTICE.mdを参照してください。

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
