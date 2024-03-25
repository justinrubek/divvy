use crate::error::Result;
use clap::Parser;
use tracing::debug;

mod commands;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    debug!("{:?}", args);

    Ok(())
}
