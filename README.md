# Zoo

ZooはZliサークル内製のチャットツールです。

[GitHubプロジェクト](https://github.com/orgs/Zli-UoA/projects/5)

## ドキュメント

必ず目を通してください。

- [ミーティングなどについて](https://zli.esa.io/posts/259)
- [GitHubProjectsの見方・使い方](https://zli.esa.io/posts/258)
- [テストについて](https://zli.esa.io/posts/261)
- [ブランチのルール・開発手順](https://zli.esa.io/posts/260)

## 環境構築

### Prerequirement

- Rustの環境
  - cargo: 1.78.0
- Nodeの環境
  - node: v22.4.0以上
- Dockerの環境
  - docker

### 各種インストール

リポジトリのダウンロード

```sh
git clone git@github.com:Zli-UoA/zoo.git
```

ツールのインストール

```sh
# Sea-ORMのCLI、コード生成に使う
cargo install sea-orm-cli
```

フロントエンドのライブラリインストール

```sh
# frontendの中で
npm install
```

DBスキーマの依存インストール

```sh
# backend/db-schemaの中で
npm install
```

### ORMのコード生成

```sh
# backendの中で
sea-orm-cli generate entity --output-dir ./src/generated
```

### ローカル開発環境のDBのセッティング

初期設定

```sh
# DBのコンテナ起動
docker compose up -d
# スキーマの適用
npx prisma push
```

DBスキーマを編集した場合

```sh
# DBのコンテナ起動
docker compose up -d
# スキーマのマイグレーション
npx prisma migrate dev
```
