use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::errors::create_error::CreateError;
use crate::utils::split_string::{split_string, SplitResult};

// need name, path, iter, start
pub fn create_subfiles(
    files: &Vec<String>,
    files_iter: &Vec<i32>,
    files_start: &Vec<i32>,
    path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    // will come back to optimize, but for the current build
    // will have O(n^2) time complexity
    let number_iter_files = files.len();
    let delimiter: char = '.';

    for i in 0..number_iter_files {
        let name = &files[i];
        let iter = files_iter[i];
        let start = files_start[i];
        let end = &iter + &start;
        let result = split_string(name, delimiter);

        match result {
            SplitResult::Parts(f_name, ext_name) => {
                // convert delimter to string so that it can be used to concat
                let owned_delimiter: String = delimiter.to_string();

                for i in start..end {
                    let owned_name: String = f_name.to_string();
                    let numerate: String = i.to_string();
                    let file_name: String = owned_name + &numerate + &owned_delimiter + ext_name;

                    let mut complete_path = PathBuf::new();
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
