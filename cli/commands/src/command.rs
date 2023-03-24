use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    sub: Sub,
}

#[derive(Subcommand)]
enum Sub {
    Go,
    Rust,
}
