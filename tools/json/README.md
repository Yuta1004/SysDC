# SysDC-Tool-JSON

## 概要

内部表現をJSON形式で標準出力する

## 引数

指定した場合，引数をファイル名としてファイル出力  
指定しない場合，標準出力へ出力

## 使用方法

```
let system: SysDCSystem = ~~;
let args: Vec<String> = ~~:
JSONTool::exec(&system, &args);
```
