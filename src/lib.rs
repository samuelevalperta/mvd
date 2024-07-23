pub mod args;

use std::env;
use std::fs::{self, rename, DirEntry};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use args::Args;

pub fn run(mut args: Args) -> Result<()> {
    let mut entries = get_file_entries(args.source)?;

    // Remove all the folders from the entries if the relative flag a is not set
    if !args.include_folders {
        entries.retain(|entry| entry.metadata().unwrap().is_file())
    };

    // Remove all the hidden files from the entries if the relative flag a is not set
    if !args.all_files {
        entries.retain(|entry| !entry.file_name().to_str().unwrap().starts_with('.'))
    };

    for i in 0..args.n {
        if let false = args.copy {
            let entry = match entries.pop() {
                Some(entry) => entry,
                None => std::process::exit(0),
            }

            let old_path = entry.path();
            let original_name = entry.file_name()
                .to_str()
                .with_context(|| format!("Failed to read file name: {:?}", old_path))?;

            let mut new_path = env::var("PWD").unwrap();

            if let Some(new_name) = args.names.pop() {
                new_path.push_str(&new_name.unwrap())
            } else {
                new_path.push_str(&original_name)
            }
            
            rename(old_path, new_path);
        }
        
    }

    sort_entries_by_creation_time(&mut entries);

    for e in entries {
        println!("Path: {:?}", e.path())
    }

    Ok(())
}

fn get_file_entries<P: AsRef<Path>>(path: P) -> Result<Vec<DirEntry>> {
    let entries: Vec<DirEntry> = fs::read_dir(&path)
        .with_context(|| format!("Failed to read directory: {:?}", path.as_ref()))?
        .filter_map(Result::ok)
        .collect();

    Ok(entries)
}

fn sort_entries_by_creation_time(entries: &mut [DirEntry]) {
    entries.sort_by_key(|entry| get_creation_time(entry).unwrap_or(UNIX_EPOCH));
    entries.reverse();
}

fn get_creation_time(entry: &DirEntry) -> Result<SystemTime> {
    entry
        .metadata()
        .with_context(|| format!("Failed to read metadata for: {:?}", entry.path()))?
        .created()
        .with_context(|| format!("Failed to get creation time for: {:?}", entry.path()))
}
