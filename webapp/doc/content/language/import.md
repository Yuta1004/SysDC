---
title: "ユニット間参照"
date: 2022-09-23T14:28:51Z
weight: 9
---

`from` および `import` を使用することで，他ユニットの定義を参照することが出来ます．

### 構文

```text
from <UNITNAME> import <NAME>;
from <UNITNAME> import <NAME>, ...;
```

#### UNITNAME

UNITNAME は **\.** で区切られた文字列です．  
ただし，同じ UNITNAME をもつ [ユニット(Unit)]({{%relref "language/unit.md"%}}) が定義されていない場合，エラーになります．

#### NAME

NAME は **\.** を含まない文字列です．  
ただし，`from` で指定した [ユニット(Unit)]({{%relref "language/unit.md"%}}) 内に NAME が定義されていない場合，エラーになります．

{{% notice tip %}}
ユニット間参照できるのは [データ(Data)]({{%relref "language/data.md"%}}) または [モジュール(Module)]({{%relref "language/module.md"%}}) です．
{{% /notice %}}

### サンプル

```text
unit test.A;

data DataA {
    x: i32,
    y: i32
}

module ModuleA {
    func new() -> DataA {
        @return a
        @spawn a: DataA
    }
}
```

```text
unit test.B;

from test.A import DataA, ModuleA;

module ModuleB {
    func new_data_a() -> DataA {
        @return data_a

        @spawn data_a: DataA {
            let a = ModuleA.new();
            return a;
        }
    }
}
```
