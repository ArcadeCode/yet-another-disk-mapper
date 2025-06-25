mod cli;
mod yadm;

use std::path::Path;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Version => {
            println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        }

        Commands::Serialize(args) => {
            let path = &args.path;
            let use_zstd = args.use_zstd;
            let verbose = cli.verbose;

            yadm::serialize(Path::new(path), use_zstd, verbose);
        }

        Commands::Parse(args) => {
            let path = &args.path;
            let verbose = cli.verbose;

            yadm::parse(Path::new(path), verbose);
        }
    }
}
