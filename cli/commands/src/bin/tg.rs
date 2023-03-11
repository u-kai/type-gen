use std::str::FromStr;

use clap::Parser;
use go::{
    description_generator::{
        declare_part_generator::GoDeclarePartGenerator, mapper::GoMapper,
        property_part_generator::GoPropertyPartGeneratorBuilder, GoTypeDescriptionGenerator,
    },
    generator_builder::GoTypeDescriptionGeneratorBuilder,
};
use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
use sf_df::json_to_langs::{json_to_go, json_to_rust_};

fn main() {
    let args = CommandArgs::parse();
    let dist = match &args.dist {
        Some(s) => s,
        None => "./",
    };
    match &args.lang {
        Some(Lang::Go) => {
            let builder = GoTypeDescriptionGeneratorBuilder::new().property_part_json_marshal();
            let builder = if args.pub_all {
                builder.property_part_pub_all().declare_part_pub_all()
            } else {
                builder
            };
            let builder = if args.pub_all {
                builder.property_part_all_optional()
            } else {
                builder
            };
            let generator = args.build_generator(builder);
            json_to_go(args.source, dist, generator);
        }
        Some(Lang::Rust) => {
            let builder = RustTypeDescriptionGeneratorBuilder::new()
                .declare_part_set_all_derive_with_serde(vec!["Debug", "Clone"]);
            let builder = if args.pub_all {
                builder.property_part_all_optional()
            } else {
                builder
            };
            let generator = if args.pub_all {
                builder
                    .declare_part_pub_all()
                    .property_part_pub_all()
                    .build()
            } else {
                builder.build()
            };
            json_to_rust_(args.source, dist, generator);
        }
        _ => {
            let builder = GoTypeDescriptionGeneratorBuilder::new().property_part_json_marshal();

            let builder = if args.pub_all {
                builder.property_part_pub_all().declare_part_pub_all()
            } else {
                builder
            };
            let builder = if args.pub_all {
                builder.property_part_all_optional()
            } else {
                builder
            };
            let generator = args.build_generator(builder);
            json_to_go(args.source, dist, generator);
        }
    };
}
#[derive(Parser)]
struct CommandArgs {
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

trait CommandSupportOption<T>
where
    T: Sized,
{
    fn pub_all(self) -> Self;
}
impl CommandArgs {
    fn build_generator(
        &self,
        builder: GoTypeDescriptionGeneratorBuilder,
    ) -> GoTypeDescriptionGenerator {
        builder.build()
    }
}

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
