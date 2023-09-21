use args::{Args, Commands};
use clap::Parser;
use commands::{decode, encode, print, remove};
use pngsecret::result::Result;

mod args;
mod commands;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => encode(file_path, chunk_type, message, output_file)?,

        Commands::Decode {
            file_path,
            chunk_type,
        } => decode(file_path, chunk_type)?,

        Commands::Remove {
            file_path,
            chunk_type,
            output_file,
        } => remove(file_path, chunk_type, output_file)?,

        Commands::Print { file_path } => print(file_path)?,
    }
    Ok(())
}
