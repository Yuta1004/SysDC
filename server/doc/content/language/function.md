---
title: "関数(Function)"
date: 2022-09-23T09:38:41Z
weight: 5
---

関数 (Function) はある値を受け取り，何かしらの処理を実行した結果としてある値を返すような一連の処理を表現するために使用します．  

### 構文

```text
func <NAME>() -> <TYPE> {
    <ANNOTATION>
    ...
}

func <NAME>(<NAME>: <TYPE>) -> <TYPE> {
    <ANNOTATION>
    ...
}

func <NAME>(<NAME>: <TYPE>, <NAME>: <TYPE>, ...) -> <TYPE> {
    <Annotattion>
    ...
}
```

関数 (Function) は必ず [モジュール(Module)]({{%relref "language/module.md"%}}) 内に定義される必要があります．

#### NAME

NAME は **\.** を含まない文字列です．  
ただし，既に同じ NAME をもつ [データ(Data)]({{%relref "language/data.md"%}}) や [モジュール(Module)]({{%relref "language/module.md"%}}) などが定義されている場合，エラーになります．

#### TYPE

TYPE は **\.** を含まない文字列です．  
ただし，プリミティブ型として指定できる文字列や定義済みデータの名前を指定しない場合，エラーになります．

#### ANNOTATION

ANNOTATION は関数 (Function) の処理を表現するために使用します．  
使用できる [アノテーション(Annotation)]({{%relref "language/annotation.md"%}}) は以下の通りです．

- `return`
- `spawn`
- `modify`
- `affect`

詳細は [アノテーション(Annotation)]({{%relref "language/annotation.md"%}}) を参照してください．

### サンプル

```text
unit test;

module ModuleA {
    func test(a: i32) -> i32 {
        @return b

        @spawn b: i32 {
            use a;
        }
    }
}
```
