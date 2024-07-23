use clap::Parser;

#[derive(Parser)]
/// Move the most recently added files from a folder to the current directory
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(default_value_t = 1)]
    /// Number of files to move
    pub n: u16,

    /// A list of new names for the moved files
    pub names: Vec<Option<String>>,

    /// Copy files instead of move
    #[arg(short = 'c', long = "copy")]
    pub copy: bool,

    /// Include files whose names begin with a dot (‘.’).
    #[arg(short = 'a')]
    pub all_files: bool,

    /// Include folders
    #[arg(short = 'r')]
    pub include_folders: bool,

    /// Specify the source folder
    #[arg(short, long, env = "DOWNLOAD_DIR", default_value_t = default_download_dir())]
    pub source: String,
}

fn default_download_dir() -> String {
    let mut home_dir = std::env::var("HOME").unwrap();
    home_dir.push_str("/Downloads");
    home_dir
}
