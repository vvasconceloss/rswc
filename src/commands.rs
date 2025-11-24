use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    List { file: String },
    Count { file: String },
    Words { file: String },
}
