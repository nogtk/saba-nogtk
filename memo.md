## 2章
- Rust はバイナリクレートとライブラリクレートの2種類がある
    - バイナリクレートは実行可能ファイルを生成する
    - ライブラリクレートは再利用可能なコードのコレクション
        - モジュール化したコードを他のプログラムやクレートに提供できる

今回は saba-nogtk 内部で、URLのパース部分をライブラリクレートとして切り出す。

## 3章
### Cargo.toml の表記について
- Features
    - 条件付きコンパイルや依存の切り替えを行うための機能
    - cargo --features=xxx で指定できる。
    - optional もこの feature に対して optional かどうかを指定するもの
- bin
    - バイナリファイルの設定を行う
    - required-features で、生成されるバイナリの条件を設定できる

- compile error について
    - cannot find macro `format` in this scope
        - `use alloc::format;` を追加して直った
    - no method named `to_string` found for reference `&str` in the current scope
        - `use alloc::string::{String, ToString};` を追加して直った
    - unresolved import `crate::entry_point`
    - unresolved import `sys::os`
        - 両方 saba-nogtk_core でテスト実行したら直った

## 4章
- Rc
    - strong pointer
- Weak
    - weak pointer

らしい。全然分かってない。

- if let 構文慣れが必要そう
    - https://qiita.com/plotter/items/0d8dc2782f21178d64f1


## 5章
- Rust の mod についてちゃんと調べる
- Rust の Iterator の使い方について
