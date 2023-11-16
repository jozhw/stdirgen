use std::env;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

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

impl EntityType {
    fn get_name(&self) -> &str {
        match self {
            Self::Directory(args) => &args.name,
            Self::File(args) => &args.name,
        }
    }

    // could not get it to return a reference to PathBuf, had to create a new instance of because
    // of the temporary error
    fn get_path(&self) -> PathBuf {
        let current_dir = env::current_dir().expect("Failed to find current directory");
        // control for a None object; if none then will use current dir
        match self {
            Self::Directory(args) => args.path.clone().unwrap_or(current_dir),

            Self::File(args) => args.path.clone().unwrap_or(current_dir),
        }
    }
    fn get_iter(&self) -> &i32 {
        match self {
            Self::Directory(args) => &args.iter,
            Self::File(args) => &args.iter,
        }
    }

    fn get_start(&self) -> &i32 {
        match self {
            Self::Directory(args) => &args.start,
            Self::File(args) => &args.start,
        }
    }
    // get the subfile commands for the Directory struct, but for File struct the object does not
    // exist
    fn get_subfile_commands(&self) -> Option<SubfileCommands> {
        match self {
            /* Could this be a point of error? Since the returned object is not a reference?
             * However, within the object are objects that are references */
            Self::Directory(args) => args.get_subfile_commands(),
            Self::File(_args) => None,
        }
    }
}

#[derive(Debug, Args, Clone)]
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

    /// Optional: Creates files within each directory created. Note: Currently only allows for one
    /// delimiter "."
    #[arg(short, long, requires = "files_iter")]
    pub files: Option<Vec<String>>,

    /// Sets the number of times iteration occurs
    #[arg(long, requires = "files_start")]
    pub files_iter: Vec<i32>,

    /// Sets the start
    #[arg(long, requires = "files")]
    pub files_start: Vec<i32>,
}

impl DirectoryCommand {
    /* the return SubfileCommands type contains a reference the values from the inital parsing */
    fn get_subfile_commands(&self) -> Option<SubfileCommands> {
        // check to see if subfile commands exist with files since files_iter and files_start depend on files
        if let None = &self.files {
            return None;
        }
        Some(SubfileCommands {
            files: &self.files,
            files_iter: &self.files_iter,
            files_start: &self.files_start,
        })
    }
}

/* Used in the DirectoryCommand implementation get_subfile_commands
 * to return an object containing all of the subfile commands
 * the lifetime of the object points to the lifetime of the original object from inital parsing
 * Option exists for files because files_iter and files_start depend on files and thus checking
 * for existence of subcommands is contigent upon the presence of files */

#[derive(Debug)]
pub struct SubfileCommands<'a> {
    files: &'a Option<Vec<String>>,
    files_iter: &'a Vec<i32>,
    files_start: &'a Vec<i32>,
}

#[derive(Debug, Args)]
pub struct FileCommand {
    /// Name of file, enumeration will be appended at the end of the name. Note: Currently only
    /// allows for one delimiter "."
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
}

#[cfg(test)]
mod tests {
    use crate::Cli;
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
