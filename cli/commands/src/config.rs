use std::str::FromStr;

use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use go::{
    description_generator::{
        declare_part_generator::GoDeclarePartGenerator, mapper::GoMapper,
        property_part_generator::GoPropertyPartGenerator, GoTypeDescriptionGenerator,
    },
    generator_builder::GoTypeDescriptionGeneratorBuilder,
};
use rust::{
    description_generator::{
        declare_part_generator::RustDeclarePartGenerator, mapper::RustMapper,
        property_part_generator::RustPropertyPartGenerator, RustTypeDescriptionGenerator,
    },
    generator_builder::RustTypeDescriptionGeneratorBuilder,
};

// データの取り方に責務を持たせる
// 色々なデータの取得方法を知っている
// Configファイルにも責務を持つ？
// into_type_structuresを返すのかstringを返すのかは難しいところ
// into_type_structuresを返すのであれば，その生成方法やその他の情報(元の言語は何かや，その言語固有の情報)を知らないとダメ
// そもそもディレクトリにあるデータを全て返すだけのように甘くはないからこれうまくいかん気がする
pub enum TypeGenSource {
    Dir(String),
    File(String),
    Direct(String),
    Remote(String),
}

trait Source {
    fn gen(&self) -> String;
}
#[cfg(test)]
mod tests {
    #[test]
    fn 様々な情報源がある() {
        // enumでやるのかtraitでやるのかが問題としてありそう
        // enumの場合は結合度が強くなるけど，肩が静的に決定する
        // traitの場合は結合度が緩くなるけど，肩を静的に決定するのがむずかいしいのではないか？
        // entry pointが一つでそこからsourceを返す感じだと，boxにするか元々肩を知っている必要があるから，結局enumでいい気がするな
        //let s = DirSource::new("tests");
        //let f = FileSource::new("test.json");
        //let d = DirectSource::new(r#"{"kai":"u"}"#);
    }
}

//pub trait TypeGenSource {

//}

// そもそも違う言語の設定を統一しようとするのが厳しいのでは？
// 言語ごとに設定は違うので，言語ごとに

pub struct TypeGenConfig {}

pub trait TypeDescriptionGeneratorBuilder<D, P, M>
where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    fn build(self) -> TypeDescriptionGenerator<D, P, M>;
    fn pub_all(self) -> Self;
    fn all_optional(self) -> Self;
    fn whitelist<S: Into<String>>(self, s: Vec<S>) -> Self;
    fn blacklist<S: Into<String>>(self, s: Vec<S>) -> Self;
}
//impl TypeGenConfig {
//pub fn build_generator<D, P, M>(
//&self,
//mut builder: impl TypeDescriptionGeneratorBuilder<D, P, M>,
//) -> TypeDescriptionGenerator<D, P, M>
//where
//D: DeclarePartGenerator<Mapper = M>,
//P: PropertyPartGenerator<M>,
//M: TypeMapper,
//{
//if self.pub_all {
//builder = builder.pub_all();
//}
//if self.optional_all {
//builder = builder.all_optional();
//}
//if let Some(whitelist) = self.whitelist.as_ref() {
//builder = builder.whitelist(whitelist.clone());
//};
//if let Some(blacklist) = self.blacklist.as_ref() {
//builder = builder.blacklist(blacklist.clone());
//};
//builder.build()
//}
//}
macro_rules! create_builder {
($($lang:ident),*) => {
        $(
            paste::item!{
                impl TypeDescriptionGeneratorBuilder<[<$lang DeclarePartGenerator>], [<$lang PropertyPartGenerator>], [<$lang Mapper>]>
                    for [<$lang TypeDescriptionGeneratorBuilder>]
                {
                    fn pub_all( self)->Self {
                        self.declare_part_pub_all().property_part_pub_all()
                    }
                    fn all_optional(self) ->Self{
                        self.property_part_all_optional()
                    }
                    fn blacklist<S: Into<String>>( self, s: Vec<S>) ->Self{
                        self.property_part_set_blacklist_with_keys(s)
                    }
                    fn whitelist<S: Into<String>>( self, s: Vec<S>)->Self {
                        self.property_part_set_whitelist_with_keys(s)
                    }
                    fn build(self) -> [<$lang TypeDescriptionGenerator>] {
                        self.build()
                    }
                }
            }
        )*
    }
}
create_builder!(Go, Rust);

enum Lang {
    Go,
    Rust,
}
impl Default for Lang {
    fn default() -> Self {
        Self::Rust
    }
}
impl FromStr for Lang {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "go" | "Go" | "GO" => Ok(Lang::Go),
            "rust" | "rs" | "Rust" => Ok(Lang::Rust),
            _ => Err(format!("not support lang {}", s)),
        }
    }
}
