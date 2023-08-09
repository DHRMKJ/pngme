mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use clap::Parser;

fn main() -> Result<()> {
    // todo!()
    let cli = args::Cli::parse();

    match cli.command {
        args::Commands::Encode(args) => {return commands::encode(args)},
        args::Commands::Decode(args) => {return commands::decode(args)}
    };
}
