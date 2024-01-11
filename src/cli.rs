/// To Do:: Add Error handeling for methods
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
    /// The function gets the name input for either the Directory of File commands.
    ///
    /// # Returns
    ///
    /// A string reference (&str) to the argument name input.
    pub fn get_name(&self) -> &str {
        match self {
            Self::Directory(args) => &args.name,
            Self::File(args) => &args.name,
        }
    }

    // could not get it to return a reference to PathBuf, had to create a new instance of because
    // of the temporary error
    //
    /// The function gets the path that the creation of name will be in.
    ///
    /// # Notes
    ///
    /// If a directory path is not provided, it will use the current directory.
    ///
    /// # Returns
    ///
    /// A PathBuf type of the path of the directory that the creation will occur in.
    pub fn get_path(&self) -> PathBuf {
        let current_dir: PathBuf = env::current_dir().expect("Failed to find current directory");
        // control for a None object; if none then will use current dir
        match self {
            Self::Directory(args) => args.path.clone().unwrap_or(current_dir),

            Self::File(args) => args.path.clone().unwrap_or(current_dir),
        }
    }
    /// Gets the amount of times the file or directory will be enumerated
    ///
    /// # Returns
    ///
    /// A i32 reference.
    pub fn get_iter(&self) -> &i32 {
        match self {
            Self::Directory(args) => &args.iter,
            Self::File(args) => &args.iter,
        }
    }

    /// Gets the start enumeration of the file or directory will be enumerated
    ///
    /// # Returns
    ///
    /// A i32 reference.
    pub fn get_start(&self) -> &i32 {
        match self {
            Self::Directory(args) => &args.start,
            Self::File(args) => &args.start,
        }
    }
    // get the subfile commands for the Directory struct, but for File struct the object does not
    // exist
    //
    /// Get all of the subfile commands that is only valid for the DirectoryCommands.
    ///
    /// # Note
    /// The program will panic if the vector length for file name, iter, and start are not the
    /// same.
    ///
    /// # Return
    ///
    /// Option with SubfileCommands containing vector references.
    pub fn get_subfile_commands(&self) -> Option<SubfileCommands> {
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

    /// Optional: Creates files within each directory created. Note: Requires the delimiter "." and currently only allows for one
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

    /// Gets the subfile commands
    ///
    /// # Returns
    ///
    /// Option<SubfileCommands>, because subfile commands are optional
    fn get_subfile_commands(&self) -> Option<SubfileCommands> {
        // check to see if subfile commands exist with files since files_iter and files_start depend on files

        if let Some(subfile_commands) = &self.files {
            // all of the values have to have the same length in order to match the commands
            if subfile_commands.len() != self.files_iter.len()
                || subfile_commands.len() != self.files_start.len()
                || self.files_iter.len() != self.files_start.len()
            {
                panic!("The length of files, files_iter, files_start are not equal!")
            }

            Some(SubfileCommands {
                files: subfile_commands,
                files_iter: &self.files_iter,
                files_start: &self.files_start,
            })
        } else {
            None
        }
    }
}

/* Used in the DirectoryCommand implementation get_subfile_commands
 * to return an object containing all of the subfile commands
 * the lifetime of the object points to the lifetime of the original object from inital parsing
 * Option exists for files because files_iter and files_start depend on files and thus checking
 * for existence of subcommands is contigent upon the presence of files */

#[derive(Debug)]
pub struct SubfileCommands<'a> {
    pub files: &'a Vec<String>,
    pub files_iter: &'a Vec<i32>,
    pub files_start: &'a Vec<i32>,
}

#[derive(Debug, Args, Clone)]
pub struct FileCommand {
    /// Name of file, enumeration will be appended at the end of the name. Note: Must contain the
    /// delimiter "." currently only allows for one delimiter "."
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
    use super::*;
    use crate::Cli;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
    #[test]
    fn test_get_name_directory() {
        let args: DirectoryCommand = DirectoryCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
            files: Some(vec!["file1".to_string(), "file2".to_string()]),
            files_iter: vec![1, 2, 3],
            files_start: vec![10, 20, 30],
        };

        let entity_type: EntityType = EntityType::Directory(args.clone());

        assert_eq!(entity_type.get_name(), &args.name);
    }

    #[test]
    fn test_get_path_directory() {
        let args: DirectoryCommand = DirectoryCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
            files: Some(vec!["file1".to_string(), "file2".to_string()]),
            files_iter: vec![1, 2, 3],
            files_start: vec![10, 20, 30],
        };

        let entity_type: EntityType = EntityType::Directory(args.clone());

        assert_eq!(entity_type.get_path(), args.path.unwrap());
    }

    #[test]
    fn test_get_iter_directory() {
        let args: DirectoryCommand = DirectoryCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
            files: Some(vec!["file1".to_string(), "file2".to_string()]),
            files_iter: vec![1, 2, 3],
            files_start: vec![10, 20, 30],
        };

        let entity_type: EntityType = EntityType::Directory(args.clone());

        assert_eq!(entity_type.get_iter(), &args.iter);
    }

    #[test]
    fn test_get_start_directory() {
        let args: DirectoryCommand = DirectoryCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
            files: Some(vec!["file1".to_string(), "file2".to_string()]),
            files_iter: vec![1, 2, 3],
            files_start: vec![10, 20, 30],
        };

        let entity_type: EntityType = EntityType::Directory(args.clone());

        assert_eq!(entity_type.get_start(), &args.start);
    }

    #[test]
    fn test_get_subfile_commands_with_files() {
        // make sure that the FileCommand fails since this method is only for the DirectoryCommand

        let directory_args: DirectoryCommand = DirectoryCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
            files: Some(vec![
                "file1".to_string(),
                "file2".to_string(),
                "file3".to_string(),
            ]),
            files_iter: vec![1, 2, 3],
            files_start: vec![10, 20, 30],
        };

        let entity_type: EntityType = EntityType::Directory(directory_args.clone());

        if let Some(subfile_commands) = entity_type.get_subfile_commands() {
            assert_eq!(subfile_commands.files, &directory_args.files.unwrap());
            assert_eq!(subfile_commands.files_iter, &directory_args.files_iter);
            assert_eq!(subfile_commands.files_start, &directory_args.files_start)
        } else {
            panic!("Expected Some(subfile_commands), but got None")
        }
    }

    #[test]
    fn test_get_subfile_commands_without_files() {
        let directory_args: DirectoryCommand = DirectoryCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
            files: None,
            files_iter: vec![1, 2, 3],
            files_start: vec![10, 20, 30],
        };

        let entity_type: EntityType = EntityType::Directory(directory_args.clone());

        if entity_type.get_subfile_commands().is_none() {
            assert!(true)
        } else {
            panic!("Expected None value, but got a Some(SubfileCommands) value")
        }
    }

    #[test]
    fn test_get_subfile_commands_files_command() {
        let file_args: FileCommand = FileCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
        };

        let entity_type: EntityType = EntityType::File(file_args.clone());

        if entity_type.get_subfile_commands().is_none() {
            assert!(true)
        } else {
            panic!("Expected None value, but got a Some(SubfileCommands) value")
        }
    }

    #[test]
    fn test_get_name_file() {
        let args: FileCommand = FileCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
        };

        let entity_type: EntityType = EntityType::File(args.clone());
        assert_eq!(entity_type.get_name(), &args.name)
    }

    #[test]
    fn test_get_path_file() {
        let args: FileCommand = FileCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
        };

        let entity_type: EntityType = EntityType::File(args.clone());
        assert_eq!(entity_type.get_path(), args.path.unwrap())
    }

    #[test]
    fn test_get_iter_file() {
        let args: FileCommand = FileCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
        };

        let entity_type: EntityType = EntityType::File(args.clone());
        assert_eq!(entity_type.get_iter(), &args.iter)
    }

    #[test]
    fn test_get_start_file() {
        let args: FileCommand = FileCommand {
            name: "test_directory".to_string(),
            path: Some(PathBuf::from("/test/path")),
            iter: 10,
            start: 5,
        };

        let entity_type: EntityType = EntityType::File(args.clone());
        assert_eq!(entity_type.get_start(), &args.start)
    }
}
