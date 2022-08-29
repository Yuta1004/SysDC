# SysDC

## 概要

システム設計支援言語

## 構成

### cli (bin)

CLI

### parser (lib)

プログラムを内部表現に変換する

### tools

#### tools/debug (lib)

内部表現を標準出力

#### tools/json (lib)

内部表現をJSON形式で標準出力する  
`--args` オプションを指定することでファイルに出力することも可能

```
    out.sysdc
       ↑ ↓
+---------------+
|      cli      |
+---------------+
        ↑
        +------------------+
  parse |             exec |
+---------------+   +---------------+
|     parser    |   |   tools/...   |
+---------------+   +---------------+
```

## サンプルコードの実行

### Box

```
$ cargo run parse example/box/box.def
Load: box.def
1 units loaded!

$ cargo run exec debug
```

### Compiler

```
$ cargo run parse example/compiler/*.def
Load: compiler.def
Load: parser.def
Load: std.def
Load: structure.def
Load: tokenizer.def
5 units loaded!

$ cargo run exec debug
```
