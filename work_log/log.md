# 作業ログ

## 作業に至るまで

- rustを再学習することを決断
- フレームワークの選定([参考](https://zenn.dev/masaya0521/articles/44ee11c0e266d9))
  - 初心者向けとの評もあるため、始めやすそうという理由で[rocket](https://rocket.rs/)を選ぶ

## 環境準備

- rust用のツールの準備
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  export PATH="$HOME/.cargo/bin:$PATH"
  rustup default stable
  ```
- 既にリポジトリを作成した後だったため、rust用に初期化する
  ```bash
  cd rust-sandbox-rocket
  cargo init
  ```

## アプリケーションの試作

- [公式のGetting started](https://rocket.rs/guide/v0.5/getting-started/)をそのまま実行
- テンプレート機能として、teraを使う
  - [参考（クラスメソッド）](https://dev.classmethod.jp/articles/rust-rocket/)

## 実行

```bash
cargo run
```

## コンテナ化

下記内容は[Rocketの元記事](https://rocket.rs/guide/v0.5/deploying/#deploying)を翻訳したもの。

1. コンフィグ
  Rocketのデプロイ時にDockerを利用する場合、下記環境変数の設定が必要

  - `ROCKET_ADDRESS`: listenするアドレス。`0.0.0.0`
  - `ROCKET_PORT`: listenするポート番号。`80` or `8080`

2. アセットバンドル
  アプリのバイナリが始まるワーキングディレクトリ配下にassetは格納しようね

3. ロードバランサ
  Rocket自体にDDoSへのサポートがないため、ロードバランサの後に配置するなど、L7の負荷分散は自前で用意してね。

4. サービスマネジメント
  運用しているアプリケーションを停止する際の管理方法の提言。現時点ではそこまで考えなくて良い。

記事内に言及がある通り、Dockerfileを作成。ポイントは下記

- `cargo build --release`を実行
- `target/release`配下にrocketのプロジェクト名のファイルができるので、そのファイル名を使ってビルド
  - `docker build --build-arg pkg=rust-sandbox-rocket -t (IMAGE_NAME)  .`
- 動作確認
  - `docker run -it -p 8080:8080 (IMAGE_NAME)`