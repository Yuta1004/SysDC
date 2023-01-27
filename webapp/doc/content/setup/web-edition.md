---
title: "Web Edition"
date: 2022-09-23T07:09:03Z
weight: 1
---

### 注意事項

ビルド・実行には以下のパッケージ・ソフトウェアが必要です．

- `make`
- `Docker`
- `docker-compose`


### 1. リポジトリダウンロード

```sh
$ git clone --recursive https://github.com/Yuta1004/SysDC
$ cd SysDC/webapp
```

### 2. ビルド

事前にビルドが必要なプロジェクトのビルドを行います．  

```
$ make setup
$ make build
```

### 3. 設定変更

`run.conf` をテキストエディタで開き，1行目および2行目の内容を適切なものに変更してください．

- SYSDC_BASE_URL : ベースURL
- SYSDC_PORT : 待受ポート

#### 例

```
export SYSDC_BASE_URL="http://localhost:50000"
export SYSDC_PORT=50000
```

### 4. 実行

```
$ make run
```

コマンド実行後，3. で `SYSDC_BASE_URL` に設定したURLにアクセスしてください．
