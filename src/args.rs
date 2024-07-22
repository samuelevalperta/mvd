use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
/// Move the most recently added files from a folder to the current directory
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(default_value_t = 1)]
    /// Number of files to move
    n: u32,

    /// A list of new names for the moved files
    names: Vec<Option<String>>,

    /// Copy files instead of move
    #[arg(short = 'c', long = "copy")]
    copy: bool,

    /// Include files whose names begin with a dot (‘.’).
    #[arg(short = 'a')]
    all_files: bool,

    /// Specify the source folder
    #[arg(short, long, env = "DOWNLOAD_DIR", default_value = "$HOME/Downloads")]
    source: std::path::PathBuf,
}
