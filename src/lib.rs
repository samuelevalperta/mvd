pub mod args;

use std::env;
use std::fs::{self, copy, rename, DirEntry};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use args::Args;

pub fn run(mut args: Args) -> Result<()> {
    let mut entries = get_file_entries(args.source)?;

    entries.apply_entry_filter(args.include_folders, args.all_files);

    entries.sort_entries_by_creation_time();

    println!("{:?}", entries);

    for _ in 0..args.n {
        let entry = match entries.pop() {
            Some(entry) => entry,
            None => std::process::exit(0),
        };
        let old_path: String = entry.path().to_string_lossy().into_owned();
        let original_name: String = entry.file_name().to_str().unwrap().to_string();

        let mut new_path: String = env::var("PWD").unwrap();
        new_path.push('/');

        if let Some(new_name) = args.names.pop() {
            new_path.push_str(&new_name.unwrap())
        } else {
            new_path.push_str(&original_name)
        }

        if args.copy {
            println!("Copying{:?} to {:?}", old_path, new_path);
            copy(&old_path, &new_path)
                .with_context(|| format!("Unable to copy {} to {}", old_path, new_path))?;
        } else {
            println!("Moving {:?} to {:?}", old_path, new_path);
            rename(&old_path, &new_path)
                .with_context(|| format!("Unable to move {} to {}", old_path, new_path))?;
        }
    }

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

fn get_creation_time(entry: &DirEntry) -> Result<SystemTime> {
    entry
        .metadata()
        .with_context(|| format!("Failed to read metadata for: {:?}", entry.path()))?
        .created()
        .with_context(|| format!("Failed to get creation time for: {:?}", entry.path()))
}

trait EntryFilter {
    fn apply_entry_filter(&mut self, include_folders: bool, all_files: bool) -> &mut Self;
    fn sort_entries_by_creation_time(&mut self) -> &mut Self;
}

impl EntryFilter for Vec<DirEntry> {
    fn apply_entry_filter(&mut self, include_folders: bool, all_files: bool) -> &mut Self {
        // Remove all the folders from the entries if the relative flag a is not set
        if !include_folders {
            self.retain(|entry| entry.metadata().unwrap().is_file());
            println!("Removed folders");
        } else {
            todo!();
        };

        // Remove all the hidden files from the entries if the relative flag a is not set
        if !all_files {
            self.retain(|entry| !entry.file_name().to_str().unwrap().starts_with('.'));
            println!("Remove hidden files");
        };

        self
    }

    fn sort_entries_by_creation_time(&mut self) -> &mut Self {
        self.sort_by_key(|entry| get_creation_time(entry).unwrap_or(UNIX_EPOCH));
        self.reverse();

        self
    }
}
