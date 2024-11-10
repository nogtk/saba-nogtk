## 2章
- Rust はバイナリクレートとライブラリクレートの2種類がある
    - バイナリクレートは実行可能ファイルを生成する
    - ライブラリクレートは再利用可能なコードのコレクション
        - モジュール化したコードを他のプログラムやクレートに提供できる

今回は saba-nogtk 内部で、URLのパース部分をライブラリクレートとして切り出す。