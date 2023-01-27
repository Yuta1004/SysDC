# SysDC Web-Edition

## 実行方法

`Docker` `docker-compose` が必要です

### 1. ビルド

```
$ make setup
$ make build
```

### 2. 設定ファイル編集

`run.conf` の1，2行目の内容をそれぞれ適切な値に設定してください

- SYSDC_BASE_URL : SysDCを公開するURL
- SYSDC_PORT : SysDCを公開するポート

### 3. 起動

```
$ make run
または
$ SYSDC_OPTS="..." make run     # docker-composeに渡すオプションを設定する場合
```

### その他のコマンド

- `make stop`
- `make down`
