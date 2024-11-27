# 作業ログ

## 作業に至るまで

- rustを再学習することを決断
- フレームワークの選定([参考](https://zenn.dev/masaya0521/articles/44ee11c0e266d9))
  - 初心者向けとの評もあるため、始めやすそうという理由で[rocket](https://rocket.rs/)を選ぶ

## 環境準備

- rust用のツールの準備
  ```bash
  $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  $ export PATH="$HOME/.cargo/bin:$PATH"
  $ rustup default stable
  ```
- 既にリポジトリを作成した後だったため、rust用に初期化する
  ```bash
  $ cd rust-sandbox-rocket
  $ cargo init
  ```
- XXX
