pub mod args;

use std::fs::{self, DirEntry};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use args::Args;

pub fn run(_args: Args) -> Result<()> {
    let mut entries = get_entries(".")?;
    sort_entries_by_creation_time(&mut entries)?;

    // for e in entries {
    //     println!("Path: {:?}", e.path())
    // }

    Ok(())
}

fn get_entries<P: AsRef<Path>>(path: P) -> Result<Vec<DirEntry>> {
    let entries: Vec<DirEntry> = fs::read_dir(&path)
        .with_context(|| format!("Failed to read directory: {:?}", path.as_ref()))?
        .filter_map(Result::ok)
        .collect();

    Ok(entries)
}

fn sort_entries_by_creation_time(entries: &mut [DirEntry]) -> Result<()> {
    entries.sort_by_key(|entry| get_creation_time(entry).unwrap_or(UNIX_EPOCH));
    entries.reverse();
    Ok(())
}

fn get_creation_time(entry: &DirEntry) -> Result<SystemTime> {
    entry
        .metadata()
        .with_context(|| format!("Failed to read metadata for: {:?}", entry.path()))?
        .created()
        .with_context(|| format!("Failed to get creation time for: {:?}", entry.path()))
}
