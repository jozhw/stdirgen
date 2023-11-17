use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::errors::create_error::CreateError;

/// Create the enumerated generation of directories of interest.
///
/// # Arguments
///
/// * 'name' - Reference str: The name of the directory.
/// * 'path' - Reference PathBuf: The path of where the directory will be.
/// * 'iter' - Reference i32: How many times the enumeration will occur.
/// * 'start' - Reference i32: Which value does the enumeration begin.
///
/// # Returns
///
/// If no errors, the function will return a vector containing the names of the enumerated
/// directories that were generated.
pub fn create_dir(
    name: &str,
    path: &PathBuf,
    iter: &i32,
    start: &i32,
) -> Result<Vec<String>, Box<dyn Error>> {
    let end: i32 = iter + start;
    let mut enumerated_directories: Vec<String> = Vec::new();
    for i in *start..end {
        let owned_name: String = name.to_string();
        let numerate: String = i.to_string();
        let dir_name: String = owned_name + &numerate;

        let mut complete_path: PathBuf = PathBuf::new();
        // push the original path not including dir_name
        complete_path.push(path);
        // push the dir_name
        complete_path.push(&dir_name);
        // push the dir_name into enumerated_directories
        enumerated_directories.push(dir_name.clone());

        // Use if let to handle the result of fs::create_dir and return an error if it fails
        if let Err(err) = fs::create_dir(&complete_path) {
            return Err(Box::new(CreateError(format!(
                "Error creating directory '{}': {}",
                dir_name, err
            ))));
        }
    }
    // all iterations are successful
    Ok(enumerated_directories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_dir() {
        let name: &str = "test_dir";
        let path: PathBuf = PathBuf::from("./target");
        let iter: i32 = 2;
        let start: i32 = 1;

        let result: Result<Vec<String>, Box<dyn Error>> = create_dir(name, &path, &iter, &start);

        assert!(result.is_ok(), "Failed to create directories {:?}", result);

        // confirm directories were created
        for i in start..start + iter {
            let dir_name: String = format!("{}{}", name, i);
            let mut complete_path: PathBuf = path.clone();
            complete_path.push(dir_name);

            assert!(
                complete_path.exists() && complete_path.is_dir(),
                "Directory not found {:?}",
                complete_path
            )
        }

        // remove test directories
        for i in start..start + iter {
            let dir_name: String = format!("{}{}", name, i);
            let mut complete_path: PathBuf = path.clone();
            complete_path.push(dir_name);

            let _ = std::fs::remove_dir_all(&complete_path);
        }
    }
}
