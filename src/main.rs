mod commands;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

use crate::commands::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { file } => {
            let mut lines_count: u8 = 0;
            let file_buffer = BufReader::new(File::open(&file).expect("Unable to open file"));

            for _ in file_buffer.lines() {
                lines_count += 1;
            }

            println!("{} {}", lines_count, file);
        }
    }
}
