---
title: "データ(Data)"
date: 2022-09-23T08:44:28Z
weight: 3
---

データ (Data) はプリミティブ型では表現できない複雑な型を表現するために使用します．  
データはC言語の構造体に似ています．

### 構文

```text
data <NAME> {
    <NAME>: <TYPE>
}

data <NAME> {
    <NAME>: <TYPE>,
    <NAME>: <TYPE>,
    <NAME>: <TYPE>
}
```

#### NAME

NAME は **\.** を含まない文字列です．  
ただし，同じデータに重複する NAME が含まれる場合，エラーになります．

#### TYPE

TYPE は **\.** を含まない文字列です．  
ただし，プリミティブ型として指定できる文字列や定義済みデータの名前を指定しない場合，エラーになります．

### サンプル

```text
unit test;

data DataA {
    x: i32,
    y: u32
}

data DataB {
    id: i32,
    a: DataA
}

data DataC {
    b: DataB
}
```
