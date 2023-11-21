use std::fs;
use std::path::PathBuf;
use anyhow::Context;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use log::{debug, error, info};

#[derive(Parser)]
#[command(author, version)]
struct Args {
    file_path: PathBuf,
    #[command(flatten)]
    verbose: Verbosity,
}

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    debug!("start reading file");
    let file_content = fs::read_to_string(&args.file_path);
    if file_content.is_err() { error!("failed to read file content")  }
    let file_content = file_content.with_context(|| format!("could not read file '{}'", args.file_path.display()))?;
    info!("file content was read");

    // TODO: Use Shellborn
    Ok(())
}
