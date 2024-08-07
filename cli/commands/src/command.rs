use std::path::Path;

use clap::{Parser, Subcommand};
use go::generator_builder::GoTypeDescriptionGeneratorBuilder;
use rust::{
    description_generator::RustTypeDescriptionGenerator,
    generator_builder::RustTypeDescriptionGeneratorBuilder,
};
use sf_df::{
    extension::Extension,
    fileconvertor::{FileStructure, PathStructure},
    fileoperator::file_structures_to_files,
};

use crate::config::{InlineSource, SourceConvertor, TypeGenSource};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    sub: Sub,
}
impl Cli {
    pub async fn exec(self) {
        match self.sub {
            Sub::Go {
                dist,
                source,
                remote_config_file,
                pub_all,
                all_pointer,
                comment,
                extension,
                optional_all,
                name,
                json_tag,
                row,
                console,
            } => {
                Sub::exec_go(
                    dist,
                    extension,
                    source,
                    remote_config_file,
                    pub_all,
                    all_pointer,
                    comment,
                    optional_all,
                    name,
                    json_tag,
                    row,
                    console,
                )
                .await;
            }
            Sub::Rust {
                dist,
                source,
                extension,
                remote_config_file,
                pub_all,
                derives,
                comment,
                optional_all,
                name,
                row,
                console,
            } => {
                Sub::exec_rust(
                    dist,
                    extension,
                    source,
                    remote_config_file,
                    pub_all,
                    derives,
                    comment,
                    optional_all,
                    name,
                    row,
                    console,
                )
                .await;
            }
        }
    }
}

#[derive(Subcommand)]
enum Sub {
    Go {
        #[clap(short, long)]
        extension: Option<String>,
        #[clap(short, long)]
        dist: Option<String>,
        #[clap(short, long)]
        source: Option<String>,
        #[clap(short, long)]
        remote_config_file: Option<String>,
        #[clap(short, long)]
        pub_all: bool,
        #[clap(short, long)]
        all_pointer: bool,
        #[clap(short, long)]
        comment: Option<String>,
        #[clap(short, long)]
        optional_all: bool,
        #[clap(short, long)]
        name: Option<String>,
        #[clap(short, long)]
        json_tag: bool,
        #[clap(long)]
        row: Option<String>,
        #[clap(long)]
        console: bool,
    },
    Rust {
        #[clap(short, long)]
        extension: Option<String>,
        #[clap(short, long)]
        dist: Option<String>,
        #[clap(short, long)]
        source: Option<String>,
        #[clap(short, long)]
        remote_config_file: Option<String>,
        #[clap(short, long)]
        pub_all: bool,
        #[clap(long)]
        derives: Option<Vec<String>>,
        #[clap(short, long)]
        comment: Option<String>,
        #[clap(short, long)]
        optional_all: bool,
        #[clap(short, long)]
        name: Option<String>,
        #[clap(long)]
        row: Option<String>,
        #[clap(long)]
        console: bool,
    },
}
impl Sub {
    async fn exec_go(
        dist: Option<String>,
        extension: Option<String>,
        source: Option<String>,
        remote_config_file: Option<String>,
        pub_all: bool,
        pointer_all: bool,
        _comment: Option<String>,
        optional_all: bool,
        name: Option<String>,
        json_tag: bool,
        row: Option<String>,
        console: bool,
    ) {
        let dist = if let Some(dist) = dist {
            dist
        } else {
            "./dist".to_string()
        };
        let extension: Extension = if let Some(extension) = extension.as_ref() {
            extension.as_str().into()
        } else {
            "json".into()
        };

        let source = Self::make_source(name, source, remote_config_file, extension, row);

        let mut builder = GoTypeDescriptionGeneratorBuilder::new();
        if pub_all {
            builder = builder.declare_part_pub_all();
            builder = builder.property_part_pub_all();
        }
        if pointer_all {
            builder = builder.property_part_all_pointer();
        }
        if json_tag {
            builder = builder.property_part_json_marshal();
        }
        if optional_all {
            builder = builder.property_part_all_optional();
        }
        let generator = builder.build();
        if console {
            SourceConvertor::new(source).console(&generator);
            return;
        }
        file_structures_to_files(
            SourceConvertor::new(source)
                .convert(&dist, &generator, "go")
                .await,
            sf_df::fileoperator::NamingPrincipal::Snake,
        );
    }
    async fn exec_rust(
        dist: Option<String>,
        extension: Option<String>,
        source: Option<String>,
        remote_config_file: Option<String>,
        pub_all: bool,
        derives: Option<Vec<String>>,
        comment: Option<String>,
        optional_all: bool,
        name: Option<String>,
        row: Option<String>,
        console: bool,
    ) {
        let dist = if let Some(dist) = dist {
            dist
        } else {
            "./dist".to_string()
        };
        let extension: Extension = if let Some(extension) = extension.as_ref() {
            extension.as_str().into()
        } else {
            "json".into()
        };
        let source = Self::make_source(name, source, remote_config_file, extension, row);
        let mut builder = RustTypeDescriptionGeneratorBuilder::new();
        if pub_all {
            builder = builder.declare_part_pub_all();
            builder = builder.property_part_pub_all();
        }
        if derives.is_some() {
            builder = builder.declare_part_set_all_derive_with_serde(derives.unwrap());
            //declare_part_all_attrs_with_serde(derives.unwrap());
        }
        if comment.is_some() {
            let comment = comment.unwrap();
            builder = builder.declare_part_all_comment(&comment.as_str());
            builder = builder.property_part_all_comment(&comment.as_str());
        }
        if optional_all {
            builder = builder.property_part_all_optional();
        }
        let generator = builder.build();
        if console {
            SourceConvertor::new(source).console(&generator);
            return;
        }
        file_structures_to_files(
            SourceConvertor::new(source)
                .convert(&dist, &generator, "rs")
                .await,
            sf_df::fileoperator::NamingPrincipal::Snake,
        );
        if dist.len() > "../".len() {
            create_rust_mod_files(&dist);
        }
    }
    fn make_source(
        name: Option<String>,
        source: Option<String>,
        remote_config_file: Option<String>,
        extension: Extension,
        row: Option<String>,
    ) -> TypeGenSource {
        match (name, source, remote_config_file, row) {
            (Some(name), _, _, Some(row)) => TypeGenSource::Inline(InlineSource::new(row, name)),
            (None, Some(source), _, _) => TypeGenSource::new(&source, extension),
            (None, None, Some(config), _) => TypeGenSource::from_config_file(&config).unwrap(),
            _ => TypeGenSource::new("./", extension),
        }
    }
}

pub async fn json_to_rust(
    src: TypeGenSource,
    dist: &str,
    generator: &RustTypeDescriptionGenerator,
) {
    file_structures_to_files(
        SourceConvertor::new(src)
            .convert(dist, generator, "rs")
            .await,
        sf_df::fileoperator::NamingPrincipal::Snake,
    );
    if dist.len() > "../".len() {
        create_rust_mod_files(&dist);
    }
}

pub fn create_rust_mod_files(root: &str) {
    fn prepare_parent_files(root: &str) {
        let root_path: &Path = root.as_ref();
        match std::fs::read_dir(root_path) {
            Ok(root_dir) => {
                root_dir
                    .filter_map(|entry| entry.ok())
                    .filter_map(|entry| match entry.file_type() {
                        Ok(file_type) => Some((file_type, entry.path())),
                        Err(_) => None,
                    })
                    .for_each(|(file_type, path)| {
                        if file_type.is_dir() {
                            prepare_parent_files(path.to_str().unwrap());
                        }
                    });
                FileStructure::new(
                    "",
                    PathStructure::new(Extension::to_filepath(root, "rs"), "rs"),
                )
                .new_file();
            }
            Err(e) => println!("not dirs {}, {:#?}", root, e),
        };
    }
    prepare_parent_files(root);
    let mut this_dirs_files = Vec::new();
    let root_path: &Path = root.as_ref();
    match std::fs::read_dir(root_path) {
        Ok(root_dir) => {
            root_dir
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| match entry.file_type() {
                    Ok(file_type) => Some((file_type, entry.path())),
                    Err(_) => None,
                })
                .for_each(|(file_type, path)| {
                    if file_type.is_dir() {
                        create_rust_mod_files(path.to_str().unwrap());
                    } else {
                        this_dirs_files.push(path);
                    }
                });

            FileStructure::new(
                this_dirs_files.into_iter().fold(String::new(), |acc, s| {
                    format!("{}pub mod {};\n", acc, Extension::remove_extension(s))
                }),
                PathStructure::new(Extension::to_filepath(root, "rs"), "rs"),
            )
            .new_file();
        }
        Err(e) => println!("not dirs {}, {:#?}", root, e),
    };
}
