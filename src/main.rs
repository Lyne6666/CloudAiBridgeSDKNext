// src/main.rs
/*
 * Main executable for CloudAiBridgeSDKNext
 */

use clap::Parser;
use cloudaibridgesdknext::{Result, run};

#[derive(Parser)]
#[command(version, about = "CloudAiBridgeSDKNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
