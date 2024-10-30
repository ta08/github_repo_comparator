# github_repo_comparator

複数のリポジトリの情報を取得し、比較して出力するプログラムです。

## プログラムの実行

コマンドライン引数でリポジトリのURLを指定して実行します。例えば、次のように実行します。

```bash
cargo run -- output.csv rust-lang/rust torvalds/linux facebook/react
```

## 出力

指定したリポジトリの情報が標準出力に表示され、output.csvファイルに保存されます。CSVファイルには、リポジトリ名、URL、説明、言語、スター数が含まれます。

```
Name                           URL                                                Description                    Language        Stars
----------------------------------------------------------------------------------------------------------------------------------
rust                           https://github.com/rust-lang/rust                  Empowering everyone to build reliable and efficient software. Rust            98026   
linux                          https://github.com/torvalds/linux                  Linux kernel source tree       C               181229
react                          https://github.com/facebook/react                  The library for web and native user interfaces. JavaScript      228707
```

## 処理フロー

![処理フロー](/doc/mermaid-diagram-2024-10-30-223631.png)


## 実験的プロジェクト

本リポジトリは実験的な内容を含んでいます。

