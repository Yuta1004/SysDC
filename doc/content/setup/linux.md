---
title: "Linux"
date: 2022-09-23T07:10:21Z
weight: 3
---

### 1. リポジトリダウンロード

```sh
$ git clone https://github.com/Yuta1004/SysDC
```

### 2. ビルド

```
$ make build
```

### 依存関係

ビルドには以下のパッケージ・ソフトウェアが必要です．

- `cargo`
- `node`
- `npm`

以下の環境でビルドできることを確認しています．

#### 実行環境

```text
Linux ArchLinux 5.19.9-arch1-1 #1 SMP PREEMPT_DYNAMIC Thu, 15 Sep 2022 16:08:26 +0000 x86_64 GNU/Linux
```

#### バージョン

```sh
$ cargo --version
cargo 1.63.0 (fd9c4297c 2022-07-01)

$ rustc --version
rustc 1.63.0 (4b91a6ea7 2022-08-08)

$ node --version
v18.9.0

$ npm --version
8.19.2
```
