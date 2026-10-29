mod cli;
mod commands;
mod error;

extern crate quick_xml;
extern crate serde;

use crate::cli::{Cli, Commands};
use crate::commands::convert_to_graphics;
use anyhow::Result;
use clap::Parser;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::ConvertToGraphics(args) => {
            info!("Transform to a graphics format.");
            convert_to_graphics::run(args)?;
        }
    }

    Ok(())
}
