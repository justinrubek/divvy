use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    // TODO: support 'k', 'm', 'g', 't' suffixes
    /// Create files *byte_count* bytes in length.
    #[arg(short, long)]
    pub(crate) byte_count: usize,
    /// The file to split.
    pub(crate) file: PathBuf,
}
