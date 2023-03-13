use std::str::FromStr;

use clap::Parser;
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
use sf_df::json_to_langs::{json_to_go, json_to_rust_};

fn main() {
    let args = CommandArgs::parse();
    let dist = match &args.dist {
        Some(s) => s,
        None => "./",
    };
    match &args.lang {
        Some(Lang::Go) => {
            let builder = GoTypeDescriptionGeneratorBuilder::new();
            let generator = args.build_generator(builder);
            json_to_go(args.source, dist, generator);
        }
        Some(Lang::Rust) => {
            let builder = RustTypeDescriptionGeneratorBuilder::new();
            let generator = args.build_generator(builder);
            json_to_rust_(args.source, dist, generator);
        }
        _ => {
            let builder = GoTypeDescriptionGeneratorBuilder::new();
            let generator = args.build_generator(builder);
            json_to_go(args.source, dist, generator);
        }
    };
}
#[derive(Parser)]
struct CommandArgs {
    #[clap(short, long)]
    whitelist: Option<Vec<String>>,
    #[clap(short, long)]
    blacklist: Option<Vec<String>>,
    #[clap(short, long)]
    pub_all: bool,
    #[clap(short, long)]
    source: String,
    #[clap(short, long)]
    dist: Option<String>,
    #[clap(short, long)]
    lang: Option<Lang>,
    #[clap(short, long)]
    optional_all: bool,
}

trait TypeDescriptionGeneratorBuilder<D, P, M>
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
impl CommandArgs {
    fn build_generator<D, P, M>(
        &self,
        mut builder: impl TypeDescriptionGeneratorBuilder<D, P, M>,
    ) -> TypeDescriptionGenerator<D, P, M>
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        if self.pub_all {
            builder = builder.pub_all();
        }
        if self.optional_all {
            builder = builder.all_optional();
        }
        if let Some(whitelist) = self.whitelist.as_ref() {
            builder = builder.whitelist(whitelist.clone());
        };
        if let Some(blacklist) = self.blacklist.as_ref() {
            builder = builder.blacklist(blacklist.clone());
        };
        builder.build()
    }
}
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
