---
title: "手元環境での実行"
date: 2022-09-23T07:09:03Z
weight: 1
---

Mac/Linux 環境での操作を説明します．

### 1. リポジトリダウンロード

```sh
$ git clone https://github.com/Yuta1004/SysDC
$ cd SysDC
```

### 2. ビルド

事前にビルドが必要なプロジェクトのビルドを行います．  

```
$ make build
```

### 3. 設定変更

`run.sh` をテキストエディタで開き，1行目および2行目の内容を適切なものに変更してください．

- SYSDC_BASE_URL : ベースURL
- SYSDC_PORT : 待受ポート

#### 例

```
export SYSDC_BASE_URL="http://localhost:50000"
export SYSDC_PORT=50000
```

### 4. 実行

```
$ ./run.sh
```

コマンド実行後，3. で `SYSDC_BASE_URL` に設定したURLにアクセスしてください．

### 注意事項

ビルド・実行には以下のパッケージ・ソフトウェアが必要です．

- `Docker`
- `docker-compose`
