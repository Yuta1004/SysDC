# SysDC

[![check](https://github.com/Yuta1004/SysDC/actions/workflows/check.yml/badge.svg?branch=master)](https://github.com/Yuta1004/SysDC/actions/workflows/check.yml)
![VERSION-Badge](https://img.shields.io/github/v/release/Yuta1004/SysDC?style=flat)

## 概要

システム設計支援言語

## 実行・起動

`Docker` `docker-compose` が必要です

```
$ ./run.sh
```

## 構成

### core

SysDCのパーサ  
(Rust)

### doc

Hugoを使用してドキュメントを提供するコンテナ  
(Hugo)

### proxy

各コンテナへの接続を管理するコンテナ  
(Envoy)

## ライセンス

### SysDC

Apache License 2.0  
Copyright 2022 Yuta NAKAGAMI

### hugo-theme-learn

Copyright (c) 2014 Grav  
Copyright (c) 2016 MATHIEU CORNIC  
Copyright (c) 2017 Valere JEANTET

[LICENSE.md](https://github.com/matcornic/hugo-theme-learn/blob/master/LICENSE.md)
