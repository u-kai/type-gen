// json の置き場に困ったが，json ではなく，parser のような subdomain を作り，その中の一つすればいいのでは？
// 実際は parse のほとんどを serde_json をやっているが，user 目線では json の文字列が type_structure に変わるのでいいのでは？？

// そう考えると，やっぱり，type_strucutre の配下におくべきでは？

type-gen.parser.json
type-gen.parser.grapql
type-gen.parser.langs.shared
type-gen.parser.langs.rust
type-gen.parser.langs.typescript

type-gen.type_structure.from.json
type-gen.type_structure.from.graphql
type-gen.type_structure.from.langs.shared
type-gen.type_structure.from.langs.rust
type-gen.type_structure.from.langs.typescript

// ただし上だと，type_description の rust と type_structure の rust が分離してしまう
// rust を domain に持っていくと以下のようになる
type-gen.langs.rust/description_generator.type_mapper
type-gen.langs.rust/description_generator.builder.orchestrator
type-gen.langs.rust/description_generator.builder.type_part
type-gen.langs.rust/description_generator.builder.property_part
type-gen.langs.rust/into_type_structure
type-gen.langs/shared.parser
