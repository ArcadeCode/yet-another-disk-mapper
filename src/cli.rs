use clap::{Parser, Subcommand, Args, ArgAction};

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "YADM is a lightweight mapping tool designed for small-scale projects. It allows you to serialize the structure of a file system directory into compact and optimized formats for fast storage and access."
)]
pub struct Cli {
    /// Active le mode verbeux
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show the current version of YADM
    Version,

    /// Serialize a directory or just one file
    Serialize(SerializeArgs),

    /// Parse a serialize file
    Parse(ParseArgs),
}

#[derive(Args)]
pub struct SerializeArgs {
    /// Path to the element to serialize
    #[arg(value_name = "PATH")]
    pub path: String,

    /// Enable or disable compression using zstd, (set to true by default)
    #[arg(long, action = ArgAction::Set, default_value_t = true)]
    pub use_zstd: bool,
}

#[derive(Args)]
pub struct ParseArgs {
    /// Path to the file to parse.
    #[arg(value_name = "PATH")]
    pub path: String,
}
