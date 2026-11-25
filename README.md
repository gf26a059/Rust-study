# 🦀 Rust Study — 手を動かして学ぶ Rust

壊れたコードを直して、テストを通しながら Rust を学ぶための練習問題集です。
全 15 問。各問題ファイルの先頭に解説があるので、**読む → 直す → チェック** をくり返すだけで基礎が身につきます。

## はじめかた

```bash
cargo run
```

最初に解く問題とエラー内容が表示されます。表示されたファイル (`exercises/01_variables.rs` など) をエディタで開いて直しましょう。

おすすめは **watch モード** です。ファイルを保存するたびに自動でチェックし、合格すると次の問題へ進みます。

```bash
cargo run -- watch
```

## コマンド一覧

| コマンド | 内容 |
| --- | --- |
| `cargo run` | 次に解く問題をチェックして、進捗を表示 |
| `cargo run -- watch` | 保存するたびに自動チェック (Ctrl+C で終了) |
| `cargo run -- hint 3` | 3 問目のヒントを表示 |
| `cargo run -- check 3` | 3 問目だけチェック |
| `cargo run -- all` | 全問をチェックして一覧表示 |
| `cargo run -- list` | 問題の一覧 (コンパイルせずに表示) |
| `cargo run -- run 15` | 問題の `main` 関数を実行 (15 問目は ToDo アプリが動きます) |
| `cargo run --bin playground` | 自由に実験できる遊び場 (`src/bin/playground.rs`) |

番号の代わりに `variables` や `03_control_flow` のような名前でも指定できます。

## 学習の流れ

| # | テーマ | ここで学ぶこと |
| --- | --- | --- |
| 01 | 変数と可変性 | `let` / `mut` / シャドーイング / `const` |
| 02 | 関数と基本の型 | 整数・小数・タプル・配列、式と文、`as` による型変換 |
| 03 | 制御フロー | `if` 式、`loop` / `while` / `for`、範囲 |
| 04 | 所有権とムーブ | ★Rust の核心。ムーブ・コピー・クローン・スコープ |
| 05 | 参照と借用 | `&` と `&mut`、借用ルール、スライス (`&str`, `&[T]`) |
| 06 | 構造体 | `struct`、`impl`、メソッドと関連関数、タプル構造体 |
| 07 | 列挙型と match | データを持つ `enum`、網羅的な `match`、`Option`、`if let` |
| 08 | コレクション | `Vec`、`String`、`HashMap` と entry API |
| 09 | エラー処理 | `Result`、`?` 演算子、独自エラー型、`map_err` |
| 10 | トレイト | トレイト定義・実装、デフォルト実装、`dyn`、`Display`、演算子オーバーロード |
| 11 | ジェネリクス | 型パラメータ、トレイト境界、ジェネリックな構造体 |
| 12 | クロージャとイテレータ | `map` / `filter` / `zip` / `collect`、`move`、`Fn` / `FnMut` |
| 13 | ライフタイム | `'a` 注釈、省略規則、参照を持つ構造体 |
| 14 | モジュール | `mod`、`pub`、カプセル化、`use` |
| 15 | 総まとめ | ToDo アプリを完成させて実際に動かす |

## 解くときのコツ

- **エラーメッセージをじっくり読もう。** Rust のコンパイラはとても親切で、`help:` に直し方が書いてあることも多いです。
- `rustc --explain E0382` のようにすると、エラー番号ごとの詳しい解説 (英語) が読めます。
- 行き詰まったら `cargo run -- hint <番号>`。それでもダメなら `solutions/` に解答例があります。
- テスト (`#[cfg(test)]` 以下) は変更しないでください。テストを読むと「何を求められているか」が分かります。
- 問題を最初からやり直したいときは:

  ```bash
  git checkout -- exercises/01_variables.rs
  ```

## エディタの設定

VS Code なら拡張機能 **rust-analyzer** を入れておくと、`exercises/` のファイルでも補完やエラー表示が効きます
(各問題は `Cargo.toml` にテストターゲットとして登録されています)。

## もっと学ぶには

- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/) — 公式の入門書。この問題集の順番もおおむねこれに沿っています
- [Rust by Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/) — サンプルコードで学ぶ
- [Rust Playground](https://play.rust-lang.org/) — ブラウザで Rust を試せる
- [標準ライブラリのドキュメント](https://doc.rust-lang.org/std/) — `Vec` や `String` のメソッドを調べるときに

## ディレクトリ構成

```
exercises/        練習問題 (ここを編集する)
solutions/        解答例
src/main.rs       チェッカー本体 (cargo run で起動)
src/exercises.rs  問題の一覧とヒント
src/bin/playground.rs  自由に試せる遊び場
```
