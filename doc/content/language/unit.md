---
title: "ユニット(Unit)"
date: 2022-09-23T08:44:25Z
weight: 2
---

設計は Unit 単位で扱われます．  
SysDCでは 1 つのプログラムファイルが 1 つの Unit 定義に対応します．  
したがって， 新しくプログラムファイルを作成することは新しく Unit を定義することに対応します．  

Unit は [データ(Data)]({{%relref "language/data.md"%}}) または [モジュール(Module)]({{%relref "language/module.md"%}}) の集合により構成されます．  


### 構文

```text
unit <UNITNAME>;
```

**unit** は必ずプログラムファイルの先頭に記述される必要があります．

#### UNITNAME

UNITNAME は以下に示すような **\.** で区切られた文字列です．

- dev
- dev.system.A
- dev.system.B.test

次のような UNITNAME は指定できません．

- .
- .dev
- ...dev
- .dev..a

{{% notice info %}}
[parse]({{%relref "language/data.md"%}}) コマンドに一度に渡すプログラムを一つの単位としたとき，この中で UNITNAME は一意に特定できるものでなければなりません．  
もし衝突した場合はエラーになります．
{{% /notice %}}

### サンプル

次のプログラムファイルは Unit **test** として扱われます．  
Unit test には
[Data]({{%relref "language/data.md"%}}) DataA ,
[Module]({{%relref "language/module.md"%}}) ModuleA ,
[Module]({{%relref "language/module.md"%}}) ModuleB
が含まれます．

```text
unit test;

data DataA {
    x: i32,
    y: i32
}

module ModuleA {}

module ModuleB {}
```
