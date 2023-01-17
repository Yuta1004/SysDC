---
title: "モジュール(Module)"
date: 2022-09-23T08:44:31Z
weight: 4
---

Module は [関数(Function)]({{%relref "language/function.md"%}}) または [プロシージャ(Procedure)]({{%relref "language/procedure.md"%}}) をある一つの単位にまとめるために使用します．

### 構文

```text
module <NAME> {}

module <NAME> {
    <FUNCTION>
}

module <NAME> {
    <PROCEDURE>
}

module <NAME> {
    <FUNCTION>
    <PROCEDURE>
    ...
}
```

#### NAME

NAME は **\.** を含まない文字列です．  
ただし，既に同じ NAME をもつデータやモジュールが定義済みである場合，エラーになります．

#### FUNCTION

[関数(Function)]({{%relref "language/function.md"%}}) を参照してください．

#### PROCEDURE

[プロシージャ(Procedure)]({{%relref "language/procedure.md"%}}) を参照してください．

### サンプル

```text
unit test;

module ModuleA {
    proc test() {

    }

    func test2(a: i32) -> i32 {
        @return a
    }
}
```
