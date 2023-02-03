# sf-df

1. 対象ディレクトリの対象拡張子のファイルを全て読み込む

- filepath,content の構造体を配列で作成する

- 最終的には dist の filepath,type_defines やその他出力したいファイルがあれば良い

- 受け取った file_structure を異なる file_structure に変換するだけ

1. filepath の dist 版の配列を作成する

   - convert_files

1. convert_files と
1. 上の配列を読み込んで，型定義，

## 設計/機能

- 対象のディレクトリにある対象の拡張子を読み込んでいく
- 読み込んだ内容と出力するディレクトリの内容を同じにする
  - ディレクトリを作成する機能が抽出できる
- 対象のディレクトリにある対象の拡張子の中身を読み取って TypeStructre にしていく
  - ファイルの中身を読み取る作業が入る
- 型定義に直して出力先に出力する
  - ファイル名を決定するロジックが必要
- rust などでは mod を pub するようなファイルも作成する
- 上記をコントローラとドメインロジックに分解してみる

---

- コントローラー

  - 指定されたディレクトリの中を全て読み込む

- ドメインロジック
  - 渡されたディレクトリの中身と，拡張子から，別の拡張子にして全く同じものを返す
  - /src/test.json -> /dist/test.rs

---

- コントローラー

  - 指定されたディレクトリ，およびファイルを作成する
  - ファイルであればファイルの中身も必要

- ドメインロジック
  - 渡されたディレクトリの中身と，拡張子から，中身を TypeStructure に変換して，その後 TypeDefine に変換してそれをパス付きの構造体にして返す
  - /src/test.json -> /dist/test.rs

## What is

- This crate do source lang(or json) file to dist lang(or json) file

## How to use

- You write sf-df.json like below

```json
{
  "src": {
    "root": "./jsons",
    "extension": "json"
  },
  "dist": {
    "root": "./src/structs",
    "extension": "rs"
  }
}
```

- And run this command

```shell
./sf-df
```

- Create to dist files

- If you customize source and dist setting, add sf-df.json like bleow

```json
{
  "dist": {
    "root": "./src/structs",
    "extension": "rs"
  },
  "rust": {
    "all_optional": true,
    "all_visibility": "pub",
    "add_derives_with_serde": ["Clone", "Debug"]
  }
}
```
