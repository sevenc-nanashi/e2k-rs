# メンテナンスメモ

## アプデ手順

1. コミット

- 全体：`git commit -am "release: vx.x.x"`
- 個別：`git commit -am "release(js): vx.x.x"`

2. `git tag vx.x.x`

- js：package.jsonをいじってから`pnpm publish`
- rust：Cargo.tomlをいじってから`cargo publish --no-default-features`
