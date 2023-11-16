use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    Directory(DirectoryCommand),
    File(FileCommand),
}

#[derive(Debug, Args)]
pub struct DirectoryCommand {
    /// Name of directory, enumeration will be appended at the end fo the name
    #[arg(short, long)]
    pub name: String,

    /// Optional: Path to directory of interest to start the creation if null will use current
    /// directory
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Sets the number of times iteration occurs, if null value is set at 1
    #[arg(short, long, default_value_t = 1)]
    pub iter: i32,

    /// Optional: Sets the initial numbering, if null begins at 1
    #[arg(short, long, default_value_t = 1)]
    pub start: i32,

    /// Optional: Creates files within each directory created
    #[arg(short, long, requires = "files_iter")]
    pub files: Option<Vec<String>>,

    /// Sets the number of times iteration occurs
    #[arg(long, requires = "files_start")]
    pub files_iter: Vec<i32>,

    /// Sets the start
    #[arg(long, requires = "files")]
    pub files_start: Vec<i32>,
}

#[derive(Debug, Args)]
pub struct FileCommand {
    /// Name of file, enumeration will be appended at the end of the name
    #[arg(short, long)]
    pub name: String,

    /// Optional: Path to directory of interest to start the creation if null will use current
    /// directory
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
