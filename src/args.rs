use std::path::PathBuf;
use clap::{Parser, Args, Subcommand};

#[derive(Parser,Debug)]
#[command(name = "Png Encoder")]
#[command(author = "Dharmik")]
#[command(version = "1.0")]
#[command(about, long_about = "a command line app to encode secret messages in a png, implmentation by https://github.com/Dh-rm-k")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand,Debug)]
pub enum Commands {
    /// encode the secret in the png file
    Encode(EncodeArgs),
    /// decode the secret message from the given png file
    Decode(DecodeArgs),
    /// remove the encoded message from the png file
    Remove(RemoveArgs),
    /// print all the chunks in a png file
    Print(PrintArgs)

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


#[derive(Args,Debug)]
pub struct RemoveArgs {
/// delete the encoded message from the png file
    #[arg(long, short)]
    pub path: PathBuf,

    /// chunk_type 
    #[arg(long, short, default_value_t=String::from("ruSt"))]
    pub chunk_type: String,
}


#[derive(Args,Debug)]
pub struct PrintArgs {
/// path to image file
    #[arg(long, short)]
    pub path: PathBuf,

    /// chunk_type 
    #[arg(long, short, default_value_t=String::from("ruSt"))]
    pub chunk_type: String,
}





