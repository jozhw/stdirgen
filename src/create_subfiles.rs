use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::errors::create_error::CreateError;
use crate::utils::split_string::{split_string, SplitResult};

/// create_subfiles creates subfiles from a list of file names that are enumerated within a particular directory
///
/// # Arguments
///
/// * 'files' - A vector of file names to create that must contain the delimiter ".".
/// * 'files_iter' - A vector of iterations for the respective file.
/// * 'files_start' - A vector of when the enumeration should start.
/// * 'path' - The path to the directory the file will reside in. Note: the directory must be
/// included.
///
/// # Returns
///
/// Result type.
pub fn create_subfiles(
    files: &Vec<String>,
    files_iter: &Vec<i32>,
    files_start: &Vec<i32>,
    // path must contain the dir the file will reside in
    path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    // will come back to optimize, but for the current build
    // will have O(n^2) time complexity
    let number_iter_files: usize = files.len();
    let delimiter: char = '.';

    for i in 0..number_iter_files {
        let name: &String = &files[i];
        let iter: i32 = files_iter[i];
        let start: i32 = files_start[i];
        let end: i32 = &iter + &start;
        let result: SplitResult<'_> = split_string(name, delimiter);

        match result {
            SplitResult::Parts(f_name, ext_name) => {
                // convert delimter to string so that it can be used to concat
                let owned_delimiter: String = delimiter.to_string();

                for j in start..end {
                    let owned_name: String = f_name.to_string();
                    let numerate: String = j.to_string();
                    let file_name: String = owned_name + &numerate + &owned_delimiter + ext_name;

                    let mut complete_path: PathBuf = PathBuf::new();
                    // push the original path not including file_name
                    complete_path.push(path);
                    // push the file_name
                    complete_path.push(&file_name);

                    // Use if let to handle the result of fs::create_dir and return an error if it fails
                    if let Err(err) = fs::File::create(&complete_path) {
                        return Err(Box::new(CreateError(format!(
                            "Error creating file '{}': {}",
                            file_name, err
                        ))));
                    }
                }
            }

            SplitResult::Error(err) => {
                println!("Error: {}", err);
            }
        }
    }

    // all iterations are successful
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() {
        let files: Vec<String> = vec!["test1_file.txt".to_string(), "test2_file.txt".to_string()];
        let files_iter: Vec<i32> = vec![2, 3];
        let files_start: Vec<i32> = vec![1, 2];
        let path: PathBuf = PathBuf::from("./target");

        // test pass cases
        let files_name: Vec<String> = vec!["test1_file".to_string(), "test2_file".to_string()];
        let extensions_name: Vec<String> = vec!["txt".to_string(), "txt".to_string()];

        let result: Result<(), Box<dyn Error>> =
            create_subfiles(&files, &files_iter, &files_start, &path);

        assert!(result.is_ok(), "Failed to create files {:?}", result);

        // confirm files were created
        let number_iter_files: usize = files.len();

        for i in 0..number_iter_files {
            let iter: i32 = files_iter[i];
            let start: i32 = files_start[i];
            let end: i32 = &iter + &start;
            let file_name: &String = &files_name[i];
            let file_extension: &String = &extensions_name[i];
            let delimiter: char = '.';

            for j in start..end {
                let file_name: String =
                    format!("{}{}{}{}", file_name, j, delimiter, file_extension);
                let mut complete_path: PathBuf = path.clone();
                complete_path.push(file_name);

                // must pass because matches desired file name
                assert!(
                    complete_path.exists() && complete_path.is_file(),
                    "File not found {:?}",
                    complete_path
                )
            }

            // remove test files
            for j in start..end {
                let file_name: String =
                    format!("{}{}{}{}", file_name, j, delimiter, file_extension);
                let mut complete_path: PathBuf = path.clone();
                complete_path.push(file_name);

                let _ = std::fs::remove_file(&complete_path);
            }
        }
    }
}
