use args::{Args, Commands};
use clap::Parser;
use pngsecret::result::Result;

mod args;

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => {
            let default = "default".to_string();
            let output_file = output_file.as_ref().unwrap_or(&default);
            let s = format!(
                "cmd: {}, path: {}, type: {}, msg: {}, output: {}",
                "encode", file_path, chunk_type, message, output_file
            );
            println!("{}", s);
        }
        Commands::Decode {
            file_path,
            chunk_type,
        } => println!(
            "cmd: {}, path: {}, type: {}",
            "decode", file_path, chunk_type
        ),
        Commands::Remove {
            file_path,
            chunk_type,
        } => println!(
            "cmd: {}, path: {}, type: {}",
            "remove", file_path, chunk_type
        ),
        Commands::Print { file_path } => println!("cmd: {}, path: {}", "print", file_path),
    }
    Ok(())
}
