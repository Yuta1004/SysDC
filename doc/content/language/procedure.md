---
title: "プロシージャ(Procedure)"
date: 2022-09-23T09:38:44Z
weight: 6
---

プロシージャ (Procedure) はある値を受け取り，何かしらの処理を実行するような一連の処理を表現するために使用します．  

### 構文

```text
proc <NAME>() {
    <ANNOTATION>
    ...
}

proc <NAME>(<NAME>: <TYPE>) {
    <ANNOTATION>
    ...
}

proc <NAME>(<NAME>: <TYPE>, <NAME>: <TYPE>, ...) {
    <Annotattion>
    ...
}
```

プロシージャ (Procedure) は必ず [モジュール(Module)]({{%relref "language/module.md"%}}) 内に定義される必要があります．

#### NAME

NAME は **\.** を含まない文字列です．  
ただし，既に同じ NAME をもつ [データ(Data)]({{%relref "language/data.md"%}}) や [モジュール(Module)]({{%relref "language/module.md"%}}) などが定義されている場合，エラーになります．

#### TYPE

TYPE は **\.** を含まない文字列です．  
ただし，プリミティブ型として指定できる文字列や定義済みデータの名前を指定しない場合，エラーになります．

#### ANNOTATION

ANNOTATION はプロシージャ (Procedure) の処理を表現するために使用します．  
使用できる [アノテーション(Annotation)]({{%relref "language/annotation.md"%}}) は以下の通りです．

- `spawn`
- `modify`
- `affect`

詳細は [アノテーション(Annotation)]({{%relref "language/annotation.md"%}}) を参照してください．

### サンプル

```text
unit test;

module ModuleA {
    proc test(a: i32) {
        @affect test2(b)

        @spawn b: i32 {
            use a;
        }
    }

    proc test2(a: i32) {}
}
```
