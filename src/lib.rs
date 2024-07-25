pub mod args;

use std::env;
use std::fs::{self, copy, rename, DirEntry};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};
use args::Args;

pub fn run(mut args: Args) -> Result<()> {
    let mut entries = get_file_entries(args.source)?;
    entries.apply_filter(args.all_files);
    entries.sort_entries_by_creation_time();

    // for every file user want to move
    for _ in 0..args.n {
        // get the last file
        let entry = match entries.pop() {
            Some(entry) => entry,
            None => {
                return Err(anyhow!("Not enough files to move"));
            }
        };

        // get info about last file
        let old_path: String = entry.path().to_string_lossy().into_owned();
        let original_name: String = entry.file_name().to_str().unwrap().to_string();

        // handle new path and name info
        let mut new_path: String = env::var("PWD").unwrap();
        new_path.push('/');
        if let Some(new_name) = args.names.pop() {
            new_path.push_str(&new_name.unwrap())
        } else {
            new_path.push_str(&original_name)
        }

        // do the operation
        if args.copy {
            copy(&old_path, &new_path)
                .with_context(|| format!("Unable to copy {} to {}", old_path, new_path))?;
        } else {
            rename(&old_path, &new_path)
                .with_context(|| format!("Unable to move {} to {}", old_path, new_path))?;
        }
    }

    // give a feedback
    let operation = match args.copy {
        true => String::from("copied"),
        false => String::from("moved"),
    };

    if args.n > 1 {
        println!("Succesfully {} {} files", operation, args.n)
    } else if args.n > 0 {
        println!("Succesfully {} 1 file", operation)
    } else {
        println!("Nothing to do!")
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
    fn apply_filter(&mut self, all_files: bool) -> &mut Self;
    fn sort_entries_by_creation_time(&mut self) -> &mut Self;
}

impl EntryFilter for Vec<DirEntry> {
    fn apply_filter(&mut self, all_files: bool) -> &mut Self {
        // Remove folders from entries
        self.retain(|entry| entry.metadata().unwrap().is_file());

        // Remove hidden files from entries if the relative flag is not set
        if !all_files {
            self.retain(|entry| !entry.file_name().to_str().unwrap().starts_with('.'));
        };

        self
    }

    fn sort_entries_by_creation_time(&mut self) -> &mut Self {
        // sort so that pop() will return the last created
        self.sort_by_key(|entry| get_creation_time(entry).unwrap_or(UNIX_EPOCH));

        self
    }
}
