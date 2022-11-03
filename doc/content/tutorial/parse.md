---
title: "parse コマンド"
date: 2022-09-23T08:27:10Z
weight: 2
---

プログラムを **.sysdcファイル** に変換するコマンドです．

```sh
./sysdc parse <PROGRAM> -o <OUTPUT>
```

### 引数

#### PROGRAM

変換するプログラムを指定します．  
ワイルドカード **\*** を用いた指定のほか，ファイルパスを列挙することでの指定もできます．

##### サンプル

```sh
./sysdc parse dir/*.def

./sysdc parse program.def

./sysdc parse dir/*.def program.def

./sysdc parse dir/**/*.def
```

#### OUTPUT

`-o` または `--output` を用いて出力ファイル名を指定することができます．  
デフォルトは **out.sysdc** です．
