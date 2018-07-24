# rust_begginer

## Rustとは
- C++を元に作られたC++よりも安全性の高い関数型言語
- 安全性・速度・並行性にフォーカス

## 開発環境
- `curl https://sh.rustup.rs -sSf | sh`
- `mkdir -p projects/rust_tutorial`
- `cd projects/rust_tutorial`
- `mkdir hello_world`
- `cd hello_world`
- `touch main.rs`

## Uninstall
- `sudo /usr/local/lib/rustlib/uninstall.sh`
- `rustup self uninstall`

## コンパイル・実行
- `rustc main.rs`
- `./main`

## Rustについて
- `println!`: マクロの呼び出し(!がある)

## Cargo
- 以下の3つを行う
  - ソースコードのビルド
  - 依存ライブラリのダウンロード
  - 依存ライブラリのビルド
- プロジェクトの作成
  - `cargo new hello_world --bin`
    - `--bin`はバイナリを作成する
- Cargo.toml: Cargoプロジェクトの設定ファイル
- ビルド・実行
  - `cargo build`
  - `./target/debug/hello_world`
  - もしくは
    - `cargo run`
- Releaseのしかた
  - `cargo build --release`
  - `./target/release/hello_world`

## Reference
1. [春休みだしRust入門しようぜ(1): Qiita](https://qiita.com/musaprg/items/97a72bb1ba85932ad161)
