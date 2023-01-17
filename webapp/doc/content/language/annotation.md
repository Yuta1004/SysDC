---
title: "アノテーション(Annotation)"
date: 2022-09-23T09:56:04Z
weight: 7
---

アノテーション(Annotation) は [関数(Function)]({{%relref "language/function.md"%}}) または [プロシージャ(Procedure)]({{%relref "language/procedure.md"%}}) の処理を表現するために使用します．

## Return

[関数(Function)]({{%relref "language/function.md"%}}) が返す値を示すために使用します．

### 構文

```text
@return <VAR_NAME>
```

1つの [関数(Function)]({{%relref "language/function.md"%}}) 内に複数の Return アノテーションが存在する場合，エラーになります．

#### VAR_NAME

VAR_NAME は **\.** を含まない文字列です．  
ただし，同じ VAR_NAME を持つ変数が定義されていない場合，エラーになります．

## Affect

[関数(Function)]({{%relref "language/function.md"%}}) または [プロシージャ(Procedure)]({{%relref "language/procedure.md"%}}) が処理中に実行する
[プロシージャ(Procedure)]({{%relref "language/procedure.md"%}}) を示すために使用します．

### 構文

```text
@affect <PROCEDURE_NAME>(<VAR_NAME>)

@affect <PROCEDURE_NAME>(<VAR_NAME>, ...)
```

#### PROCEDURE_NAME

PROCEDURE_NAME は文字列です．  
ただし，同じ PROCEDURE_NAME を持つ [プロシージャ(Procedure)]({{%relref "language/procedure.md"%}}) が定義されていない場合，エラーになります．

#### VAR_NAME

VAR_NAME は **\.** を含まない文字列です．  
ただし，同じ VAR_NAME を持つ変数が定義されていない場合，エラーになります．  

## Spawn

[関数(Function)]({{%relref "language/function.md"%}}) または [プロシージャ(Procedure)]({{%relref "language/procedure.md"%}}) が処理中に新しく作成する変数を示すために使用します．

### 構文

```text
@spawn <RESULT_NAME>: <TYPE>

@spawn <RESULT_NAME>: <TYPE> {
    use <VAR_NAME>;
}

@spawn <RESULT_NAME>: <TYPE> {
    use <VAR_NAME>, ...;
}

@spawn <RESULT_NAME>: <TYPE> {
    use <VAR_NAME>;

    let <INTER_VAR_NAME> = <FUNCTION_NAME>(<VAR_NAME>);
    let <INTER_VAR_NAME> = <FUNCTION_NAME>(<VAR_NAME>, ...);

    return <VAR_NAME>;
}
```

Spawn アノテーションは 型 TYPE の変数 RESULT_NAME を作成することを表現します．  
また，Spawn アノテーションに加えて `use` / `let` / `return` を使用することで，さらに詳細な作成過程を表現することが出来ます．  
それぞれのキーワードは以下のような過程を表現します．

- `use`: 変数 VAR_NAME を使用する
- `let`: ある [関数(Function)]({{%relref "language/function.md"%}}) FUNCTION_NAME を実行することで一時的な変数 INTER_VAR_NAME を作成する
- `return`: 最終的な成果物として VAR_NAME を使用する

{{% notice tip %}}
作成過程を明示するかどうかは選択することが出来ます．  
また，過程を明示する場合でも `use` のみを使用することが可能です．  
もし `let` を使用する場合は必ず `return` を含めなければいけません．
{{% /notice %}}

#### FUNCTION_NAME

FUNCTION_NAME は文字列です．  
ただし，同じ FUNCTION_NAME を持つ [関数(Function)]({{%relref "language/procedure.md"%}}) が定義されていない場合，エラーになります．

#### VAR_NAME

VAR_NAME は **\.** を含まない文字列です．  
ただし，同じ VAR_NAME を持つ変数が定義されていない場合，エラーになります．  

#### INTER_VAR_NAME

VAR_NAME は **\.** を含まない文字列です．  
ただし，同じ VAR_NAME または INTER_VAR_NAME を持つ変数が定義されている場合，エラーになります．  

## Modify

[関数(Function)]({{%relref "language/function.md"%}}) または [プロシージャ(Procedure)]({{%relref "language/procedure.md"%}}) が処理中に変数の値を変更することを表現するために使用します．

### 構文

```text
@modify <TARGET_NAME>

@modify <TARGET_NAME> {
    use <VAR_NAME>;
}

@modify <TARGET_NAME> {
    use <VAR_NAME>, ...;
}
```

Modify アノテーションは 変数 TARGET_NAME の値を変更することを表現します．  
また，Spawn アノテーションに加えて `use` を使用することで，さらに詳細な過程を表現することが出来ます．  
それぞれのキーワードは以下のような過程を表現します．

- `use`: 変数 VAR_NAME を使用する

{{% notice tip %}}
過程を明示するかどうかは選択することが出来ます．  
{{% /notice %}}

#### TARGET_VAR_NAME / VAR_NAME

TARGET_VAR_NAME または VAR_NAME は **\.** を含まない文字列です．  
ただし，同じ VAR_NAME を持つ変数が定義されていない場合，エラーになります．  
