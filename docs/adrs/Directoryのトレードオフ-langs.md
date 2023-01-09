// lib として新しい言語の生成器を作りたい時は呼び出される

// langs 配下に rust や Typescript などを入れる
// まとまりがあるようにも見えるが，まとめることが本当にいいのか？
type-gen.langs.rust.type_description.type_mapper
type-gen.langs.rust.type_description.generator.type_part
type-gen.langs.rust.type_description.generator.property_part
type-gen.langs.rust.type_description.generator.builder
type-gen.langs.typescript.type_description.type_mapper
type-gen.langs.typescript.type_description.generator.type_part
type-gen.langs.typescript.type_description.generator.property_part
type-gen.langs.typescript.type_description.generator.builder

// langs でまとめるよりも言語ごとにまとめた方がインストール量を調整できるかも
// また typescript が未完成でも rust だけデプロイ可能となるのはこっちなのでこっちの方が良さそう
type-gen.rust.type_description.type_mapper
type-gen.rust.type_description.generator.type_part.alias_type_convertors
type-gen.rust.type_description.generator.type_part.composite_type_convertors
type-gen.rust.type_description.generator.builder
type-gen.typescript.type_description.type_mapper
type-gen.typescript.type_description.generator.type_part.alias_type_convertors
type-gen.typescript.type_description.generator.type_part.composite_type_convertors
type-gen.typescript.type_description.generator.builder

// 上のやり方だと，parse する方も rust のドメイン内に入りそうだが，parse する方ができていなくても rust の型定義する方は生きていて欲しいので分けるべしな気もする
ドメインは一緒でも component が違えば独立してデプロイ可能なので，別にドメインは一緒でもいいのか？
そうなると一旦 langs でまとめても良さそうな気もするがどうやろな
おそらく rust だと workspace で記述した通りに workspace がきられるので，その切り方で独立してデプロイ可能かどうかが決まるはず
だからどこで workspace を切るかも独立してデプロイ可能な 1 要素として重要なのでは？
でも独立してデプロイ可能とか言いながら，bin がないものって lib だからそもそもデプロイとかいう概念ってあるんかいな？
