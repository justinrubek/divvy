use crate::error::Result;
use clap::Parser;
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};
use tracing::debug;

mod commands;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    debug!(?args);

    let file = File::open(&args.file).await?;
    let file_size = file.metadata().await?.len() as usize;
    debug!(?file_size);
    let mut reader = BufReader::new(file);

    let mut bytes_read = 0;
    let mut files_created = 0;
    while bytes_read < file_size {
        let bytes_to_read = match file_size - bytes_read {
            n if n < args.byte_count => n,
            _ => args.byte_count,
        };
        debug!("reading {bytes_to_read} bytes");
        let mut buffer = vec![0; bytes_to_read];
        reader.read_exact(&mut buffer).await?;
        bytes_read += bytes_to_read;
        files_created += 1;
        tokio::fs::write(format!("x{}", files_created), buffer).await?;
    }

    Ok(())
}
