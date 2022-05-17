# web-server

### 実装すること
- mysql接続
- httpリクエスト
- actor

## 準備
diesel cliをインストール
migrationなどdiesel cli経由で行う
※ せっかくなので使ってみる
```shell
$ cargo install diesel_cli --no-default-features --features mysql
```
### database作成
```shell
$ DATABASE_URL=mysql://root:web-server-pw@127.0.0.1:3306/web-server
$ diesel setup --database-url=$DATABASE_URL
```

### マイグレーションファイルの作成
```shell
$ diesel migration generate {migration file name sufix}
ex) diesel migration generate create_projects
```
up.sqlファイルが作成されるので、SQLを書く。

### マイグレーション実行
```shell
$ diesel migration run --database-url=$DATABASE_URL
```

## サーバ起動

```shell
$ RUN_ENV=development cargo run
```
