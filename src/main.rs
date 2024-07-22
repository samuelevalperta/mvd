use clap::Parser;
use std::process;

use mvd::{args, run};

fn main() {
    let args = args::Args::parse();
    if let Err(e) = run(args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
