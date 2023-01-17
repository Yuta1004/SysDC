---
title: "名前"
date: 2022-09-23T09:09:02Z
weight: 8
---

SysDC で定義されたすべての要素は名前を持ちます．

- [ユニット(Unit)]({{%relref "language/unit.md"%}}) 
- [モジュール(Module)]({{%relref "language/module.md"%}}) 
- [データ(Data)]({{%relref "language/data.md"%}}) 
- (など…)

名前は以下に示すような **\.** で区切られた文字列です．  
すべての名前は **.0** を先頭に持ちます．

- .0
- .0.test.Test
- .0.test.TestModule
- .0.test.TestModule.func_a

### サンプル

以下のサンプルプログラム内において，各要素は次のような名前を持ちます．  

```text
unit test;  → .0.test

data DataA { → .0.test.DataA
    x: i32,  → .0.test.DataA.x
    y: i32   → .0.test.DataA.y
}

module ModuleA {  → .0.test.ModuleA
    proc test() {  → .0.test.ModuleA.test
        @spawn a: i32 → .0.test.ModuleA.test.a
    }
}

module ModuleB { }  → .0.test.ModuleB

```
