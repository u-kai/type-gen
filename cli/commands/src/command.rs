use std::path::Path;

use clap::{Parser, Subcommand};
use go::generator_builder::GoTypeDescriptionGeneratorBuilder;
use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
use sf_df::{
    extension::Extension,
    fileconvertor::{FileStructer, PathStructure},
    fileoperator::file_structures_to_files,
};

use crate::config::{SourceConvertor, TypeGenSource};

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
        let source = match (source, remote_config_file) {
            (Some(source), _) => TypeGenSource::new(&source, extension),
            (None, Some(config)) => TypeGenSource::from_config_file(&config).unwrap(),
            _ => TypeGenSource::new("./", extension),
        };
        let mut builder = GoTypeDescriptionGeneratorBuilder::new();
        if pub_all {
            builder = builder.declare_part_pub_all();
            builder = builder.property_part_pub_all();
        }
        if pointer_all {
            builder = builder.property_part_all_pointer();
        }
        //if comment.is_some() {
        //let comment = comment.unwrap();
        //builder = builder.declare_part_all_comment(&comment.as_str());
        //builder = builder.property_part_all_comment(&comment.as_str());
        //}
        if optional_all {
            builder = builder.property_part_all_optional();
        }
        let generator = builder.build();
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
        let source = match (source, remote_config_file) {
            (Some(source), _) => TypeGenSource::new(&source, extension),
            (None, Some(config)) => TypeGenSource::from_config_file(&config).unwrap(),
            _ => TypeGenSource::new("./", extension),
        };
        let mut builder = RustTypeDescriptionGeneratorBuilder::new();
        if pub_all {
            builder = builder.declare_part_pub_all();
            builder = builder.property_part_pub_all();
        }
        if derives.is_some() {
            builder = builder.declare_part_all_attrs_with_serde(derives.unwrap());
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

                FileStructer::new(
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

            FileStructer::new(
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
