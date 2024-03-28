use clap::Parser;

mod cli;
mod random;

use cli::*;
use random::*;

// cargo run -- --help

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Float {}) => {
            println!("{}", float());
        }
        Some(Commands::Integer { sign }) => {
            if *sign {
                println!("{}", signed());
            } else {
                println!("{}", unsigned());
            }
        }
        Some(Commands::String { length }) => {
            if let Some(length) = length {
                println!("{}", string(*length));
            } else {
                println!("{}", string(LENGTH));
            }
        }
        Some(Commands::FloatRange { start, end }) => {
            println!("{}", range(*start, *end));
        }
        Some(Commands::IntegerRange { start, end }) => {
            println!("{}", range(*start, *end));
        }
        Some(Commands::Custom { length, chars }) => {
            println!("{}", custom(*length, chars.as_bytes()));
        }
        None => {
            println!("{}", custom(LENGTH, CHARACTERS));
        }
    }
}
