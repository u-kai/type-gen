use clap::Parser;
use tg::command::Cli;

#[tokio::main]
async fn main() {
    Cli::parse().exec().await;
}
