use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::cmd;

#[derive(Debug, Parser)]
#[command(version, author)]
pub struct SkCLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Initialize a new database")]
    Init {
        #[arg(default_value = "./")]
        path: PathBuf,
    },
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = SkCLI::parse();

    match cli.command {
        Commands::Init { path } => {
           todo!()
        }
    }
}
