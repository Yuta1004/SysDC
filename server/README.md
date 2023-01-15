# SysDC-Server

Webアプリケーション

## 実行方法

### 1. ビルド

```
$ make build
```

### 2. 設定ファイル編集

`run.conf` の1，2行目の内容をそれぞれ適切な値に設定してください

- SYSDC_BASE_URL : SysDCを公開するURL
- SYSDC_PORT : SysDCを公開するポート

### 3. 起動

```
$ make run-server
または
$ SYSDC_OPTS="..." make run-server      # docker-composeに渡すオプションを設定する場合
```
