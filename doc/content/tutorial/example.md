---
title: "サンプルの実行"
date: 2022-09-23T08:05:57Z
weight: 1
---

配布ファイルに同封しているサンプルプログラム *logger* を実行するチュートリアルです．

### 1. SysDCをダウンロードしたパスに移動する

```sh
$ cd /path/to/sysdc
```

移動後，以下のコマンドを入力して SysDC が実行できることを確認してください．  
正しく実行できている場合はヘルプが表示されます．

```text
$ ./sysdc
subcommand 0.1.0
SysDC: System Definition Language and Tools

USAGE:
    sysdc <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    exec     Execute tool
    parse    Parse *.def files
```

### 2. プログラムを .sysdc ファイルに変換する

SysDCを使用するためには以下の 2 手順が必要です．

1. プログラムを **.sysdcファイル** に変換する
2. **.sysdcファイル** を使用してツールを実行する

以下のコマンドで変換を行うことが出来ます．

```sh
$ ./sysdc parse example/logger/**/*.def
Loading: logger.def
Loading: std.def
Loading: io.def
Loading: time.def
4 units loaded!
```

コマンド実行後，**out.sysdc** ファイルが生成されたことを確認してください．

{{% notice info %}}
エラーが発生する場合は，正しくファイルパスを入力できているかを確認してください．
{{% /notice %}}

### 3. プログラムを .sysdc ファイルに変換する

変換結果を用いてツール **view** を実行します．  

```sh
$ ./sysdc exec view
```

{{% notice info %}}
エラーが発生する場合は，*WebView2* のインストールを行ってください．
{{% /notice %}}

{{% notice tip %}}
使用できるツールの一覧は [tool コマンド]({{%relref "tutorial/tool.md"%}}) を用いて確認することが出来ます．
{{% /notice %}}
