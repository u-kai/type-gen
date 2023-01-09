# Adr2 Directory 構成の見直し

## ステータス

- 承認済み

## コンテキスト

- 現在の workspace は以下の構造をしている

```
type-gen/lang-common.type_defines
type-gen/lang-common.type_defines.additional_defines
type-gen/lang-common.type_defines.generators
type-gen/lang-common.type_defines.generators.mapper
type-gen/lang-common.type_defines.generators.property_statement_generator
type-gen/lang-common.type_defines.generators.type_define_generator
type-gen/lang-common.type_defines.builder
type-gen/lang-common.type_defines.type_define
type-gen/lang-common.type_defines.types.primitive_type
type-gen/lang-common.type_defines.types.property_key
type-gen/lang-common.type_defines.types.property_type
type-gen/lang-common.type_defines.types.structures
type-gen/lang-common.type_defines.types.type_name

type-gen/json.into_type_structure
type-gen/json.json

type-gen/langs.rust.additional_statement
type-gen/langs.rust.attribute
type-gen/langs.rust.builder
type-gen/langs.rust.mapper
type-gen/langs.rust.property_generator
type-gen/langs.rust.reserved_words
type-gen/langs.rust.type_statement_generator

type-gen/cli.from_src_files
type-gen/cli.type_configuration

type-gen/utils.store_fn
```

- 依存関係はうまくやっているつもりだが，ディレクトリの構造がわかりにくい
- また，ライブラリとして利用するときやツールとして利用したい場合でも，細かく json ディレクトリなどがインストール可能(外部から見える状態)になっている

## 決定

- application.domain.subdomain.component.source-code といったように構造を分かりやすい形に修正する
- また，これにより，本当に外部に公開したいものだけを pub して公開する
- それ以外は pub(crate)などで依存性をコントロールする
- 各ドメインについては別資料
- 利用者にはどのように使って欲しいのか？

  - tool として
    - cli
    - api
  - lib として
    - 新しい Lang や DSL の型定義
      - その場合は mapper と customize_generator の convetor struct を定義することで色々カスタマイズできる

- 変更後の構成及び，その構成にした理由を以下に示す
- workspace は/で区切ってみる

```
application.domain.subdomain.component.source-code

type-gen.type_structure/domain.type_structure
type-gen.type_structure/domain.alias_type
type-gen.type_structure/domain.composite_type
type-gen.type_structure/domain.parts.type_name
type-gen.type_structure/domain.parts.property_key
type-gen.type_structure/domain.parts.property_type(primitive_type はこの中でもいいかもしれない)
type-gen.type_structure/domain.parts.primitive_type
type-gen.type_structure/from.json
type-gen.type_structure/from.langs.shared
type-gen.type_structure/from.langs.rust
type-gen.type_structure/from.langs.typescript

type-gen.type_description.domain.generator/orchestrator??
type-gen.type_description.domain.generator/customizable.type_part
type-gen.type_description.domain.generator/customizable.property_part
type-gen.type_description.domain.generator/type_mapper

type-gen.type_description.rust.generator/type_mapper
type-gen.type_description.rust.generator/type_part
type-gen.type_description.rust.generator/property_part
type-gen.type_description.rust.generator/orchestrator_builder


type-gen.output.file.src_to_dist

type-gen.cli.src_to_dist.command
type-gen.cli.src_to_dist.config
```

// rust とか言語の詳細を rust 配下に収めることができる

```
type-gen.type_define/structure.type_structure
type-gen.type_define/structure.alias_type
type-gen.type_define/structure.composite_type
type-gen.type_define/structure.parts.type_name
type-gen.type_define/structure.parts.property_key
type-gen.type_define/structure.parts.property_type(primitive_type はこの中でもいいかもしれない)
type-gen.type_define/structure.parts.primitive_type

type-gen.type_define/description_generator.orchestrator
type-gen.type_define/description_generator.customizable.type_part
type-gen.type_define/description_generator.customizable.property_part
type-gen.type_define/description_generator.type_mapper

type-gen.langs.rust/description_generator.type_mapper
type-gen.langs.rust/description_generator.builder.orchestrator
type-gen.langs.rust/description_generator.builder.type_part
type-gen.langs.rust/description_generator.builder.property_part
type-gen.langs.rust/into_type_structure
type-gen.langs/shared.parser

type-gen/json.json_structure
type-gen/json.into_type_structure

type-gen.output/file.src_to_dist

type-gen/cli.src_to_dist.command
type-gen/cli.src_to_dist.config
```

## 影響

- 現在の構成よりも分かりやすくなる
- 必要最低限のもののみを公開できる

## 備考

- 考案日:1/9

## 学び所感

- workspace 機能を覚えて嬉しくなって機能分解しまくったが，もう少しドメインや公開するべきもので構成を考えれば良かったのかもしれない
