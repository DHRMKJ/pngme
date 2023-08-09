use std::path::PathBuf;
use clap::{Parser, Args, Subcommand};

#[derive(Parser,Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand,Debug)]
pub enum Commands {
    /// encode the secret in the png file
    Encode(EncodeArgs),
    Decode(DecodeArgs)
}

#[derive(Args,Debug)]
pub struct EncodeArgs {
    /// path to image file
    #[arg(long, short)]
    pub path: PathBuf,

    /// chunk_type 
    #[arg(long, short, default_value_t=String::from("ruSt"))]
    pub chunk_type: String,

    #[arg(long, short)]
    pub message: String
}


#[derive(Args,Debug)]
pub struct DecodeArgs {
/// path to image file
    #[arg(long, short)]
    pub path: PathBuf,

    /// chunk_type 
    #[arg(long, short, default_value_t=String::from("ruSt"))]
    pub chunk_type: String,
}
