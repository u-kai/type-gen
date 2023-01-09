// lib として新しい言語の生成器を作りたい時は呼び出される

//1
type-gen.type_description.type_mapper
type-gen.type_description.generator.bases.orchestrator
type-gen.type_description.generator.bases.type_part
type-gen.type_description.generator.bases.property_part
type-gen.type_description.generator.customizable.type_part
type-gen.type_description.generator.customizable.property_part

//2
type-gen.type_description.generator.orchestrator
type-gen.type_description.generator.type_part.base
type-gen.type_description.generator.type_part.customizable.alias_type
type-gen.type_description.generator.type_part.customizable.composite_type
type-gen.type_description.generator.property_part.base
type-gen.type_description.generator.property_part.customizable

//3
// orchestrtor が type_part と property_part の trait を実装してそれらに依存させるべしっていう考えでいく
// これだと customizable が orchestrator 内の type_part,property_part trait に依存するのがよくわかる

type-gen.type_description.generator.orchestrator
type-gen.type_description.generator.customizable.type_part
type-gen.type_description.generator.customizable.property_part
