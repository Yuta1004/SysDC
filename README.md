# SysDC

[![check](https://github.com/Yuta1004/SysDC/actions/workflows/check.yml/badge.svg?branch=master)](https://github.com/Yuta1004/SysDC/actions/workflows/check.yml)
![VERSION-Badge](https://img.shields.io/github/v/release/Yuta1004/SysDC?style=flat)

## 概要

システム設計支援言語 ＋ 周辺環境

## 起動方法

`Docker` `docker-compose` が必要です

### 1. ビルド

```
$ make build
```

### 2. 設定ファイル作成

```
$ make conf
```

### 3. 設定ファイル編集

1，2行目の内容をそれぞれ適切な値に設定してください

- SYSDC_BASE_URL : SysDCを公開するURL
- SYSDC_PORT : SysDCを公開するポート

### 4. 起動

```
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
