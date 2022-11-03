---
title: "exec コマンド"
date: 2022-09-23T08:27:10Z
weight: 3
---

**.sysdcファイル** を使用してツールを実行するコマンドです．

```sh
./sysdc exec <TOOL> -i <INPUT>
```

### 引数

#### TOOL

実行するツール名を指定します．

{{% notice tip %}}
使用できるツールの一覧は [tool コマンド]({{%relref "tutorial/tool.md"%}}) を用いて確認することが出来ます．
{{% /notice %}}

#### INPUT

`-i` または `--input` を用いて出力ファイル名を指定することができます．  
デフォルトは **out.sysdc** です．
