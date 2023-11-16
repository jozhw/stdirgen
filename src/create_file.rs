use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::errors::create_error::CreateError;
use crate::utils::split_string::{split_string, SplitResult};

// need name, path, iter, start
pub fn create_file(
    name: &str,
    path: &PathBuf,
    iter: &i32,
    start: &i32,
) -> Result<(), Box<dyn Error>> {
    let end = iter + start;
    let delimiter: char = '.';

    // split the file extension from name and concat after numeration occurs
    let result = split_string(name, delimiter);

    match result {
        SplitResult::Parts(f_name, ext_name) => {
            // convert delimter to string so that it can be used to concat
            let owned_delimiter: String = delimiter.to_string();

            for i in *start..end {
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

    // all iterations are successful
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() {
        let name = "test_file.txt";
        let path = PathBuf::from("./target");
        let iter = 2;
        let start = 1;
        // test case
        let test_name = "test_file";
        let test_ext = "txt";
        let delimiter = ".";
        // must fail path

        let result = create_file(name, &path, &iter, &start);

        assert!(result.is_ok(), "Failed to create files {:?}", result);

        // confirm files were created
        for i in start..start + iter {
            let file_name = format!("{}{}{}{}", test_name, i, delimiter, test_ext);
            let mut complete_path = path.clone();
            complete_path.push(file_name);

            let mut fail_complete_path = path.clone();
            let test_file_name = format!("{}{}{}", test_name, i, test_ext);

            // must fail because file name is missing delimiter
            fail_complete_path.push(test_file_name);
            assert_ne!(
                complete_path, fail_complete_path,
                "The values are equal, but are not supposed to be."
            );

            // must pass because matches desired file name
            assert!(
                complete_path.exists() && complete_path.is_file(),
                "File not found {:?}",
                complete_path
            )
        }

        // remove test files
        for i in start..start + iter {
            let file_name = format!("{}{}{}{}", test_name, i, delimiter, test_ext);
            let mut complete_path = path.clone();
            complete_path.push(file_name);

            let _ = std::fs::remove_file(&complete_path);
        }
    }
}


