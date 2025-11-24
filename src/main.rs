mod commands;

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
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
        Commands::Count { file } => {
            let file_open = File::open(&file).expect("Unable to open file");
            let mut content = String::new();
            let mut buffer_reader = BufReader::new(file_open);

            println!(
                "{:?} {}",
                buffer_reader
                    .read_to_string(&mut content)
                    .expect("Failed to get file bytes"),
                file
            )
        }
        Commands::Words { file } => {
            let mut words_count: u8 = 0;
            let file_buffer = BufReader::new(File::open(&file).expect("Unable to open file"));

            for line in file_buffer.lines() {
                for _ in line.unwrap().split_whitespace() {
                    words_count += 1;
                }
            }

            println!("{} {}", words_count, file);
        }
    }
}
