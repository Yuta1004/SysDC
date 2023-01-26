---
title: "CLI Edition"
date: 2022-09-23T07:09:03Z
weight: 2
---

### 注意事項

ビルド・実行には以下のパッケージ・ソフトウェアが必要です．

- `make`
- `Docker`


### 1. リポジトリダウンロード

```sh
$ git clone --recursive https://github.com/Yuta1004/SysDC
$ cd SysDC/cliapp
```

### 2. ビルド

```
$ make setup
$ make build
```

### 3. 実行

```
$ ./sysdc_cli
```

`--help` オプションをつけることで，使用方法を確認することができます．
