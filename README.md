# SysDC

[![check](https://github.com/Yuta1004/SysDC/actions/workflows/check.yml/badge.svg?branch=master)](https://github.com/Yuta1004/SysDC/actions/workflows/check.yml)
![VERSION-Badge](https://img.shields.io/github/v/release/Yuta1004/SysDC?style=flat)

## 概要

システム設計支援言語 ＋ 周辺環境

## 実行方法

`Docker` `docker-compose` が必要です

### 1. セットアップ

```
$ make setup
```

### 2. 実行

#### サーバ (server)

詳細は `server/README.md` を参照してください

```
$ cd server
$ make build
$ make run
```

## ライセンス

### SysDC

Apache License 2.0  
Copyright 2022 Yuta NAKAGAMI

### hugo-theme-learn

Copyright (c) 2014 Grav  
Copyright (c) 2016 MATHIEU CORNIC  
Copyright (c) 2017 Valere JEANTET

[LICENSE.md](https://github.com/matcornic/hugo-theme-learn/blob/master/LICENSE.md)
