use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        file_path: String,
        chunk_type: String,
        message: String,
        output_file: String,
    },
    Decode {
        file_path: String,
        chunk_type: String,
    },
    Remove {
        file_path: String,
        chunk_type: String,
        output_file: String,
    },
    Print {
        file_path: String,
    },
}
