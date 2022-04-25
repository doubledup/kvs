use std::process::exit;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, author, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Set a key to a value
    Set { key: String, value: String },

    /// Get a value for a key
    Get { key: String },

    /// Remove a value for a key
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();

    // if cli.version {
    //     println!("0.1.1");
    // }

    match &cli.command {
        Commands::Set {
            key: _key,
            value: _value,
        } => {
            // println!("Setting key `{}` to `{}`", key, value);
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Get { key: _key } => {
            // println!("Getting value for key `{}`", key);
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { key: _key } => {
            // println!("Removing value at key `{}`", key);
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
