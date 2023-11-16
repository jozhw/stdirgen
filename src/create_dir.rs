use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::errors::create_error::CreateError;

// need name, path, iter, start
pub fn create_dir(
    name: &str,
    path: &PathBuf,
    iter: &i32,
    start: &i32,
) -> Result<(), Box<dyn Error>> {
    let end = iter + start;

    for i in *start..end {
        let owned_name: String = name.to_string();
        let numerate: String = i.to_string();
        let dir_name: String = owned_name + &numerate;

        let mut complete_path = PathBuf::new();
        // push the original path not including dir_name
        complete_path.push(path);
        // push the dir_name
        complete_path.push(&dir_name);

        // Use if let to handle the result of fs::create_dir and return an error if it fails
        if let Err(err) = fs::create_dir(&complete_path) {
            return Err(Box::new(CreateError(format!(
                "Error creating directory '{}': {}",
                dir_name, err
            ))));
        }
    }
    // all iterations are successful
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_dir() {
        let name = "test_dir";
        let path = PathBuf::from("./target");
        let iter = 2;
        let start = 1;

        let result = create_dir(name, &path, &iter, &start);

        assert!(result.is_ok(), "Failed to create directories {:?}", result);

        // confirm directories were created
        for i in start..start + iter {
            let dir_name = format!("{}{}", name, i);
            let mut complete_path = path.clone();
            complete_path.push(dir_name);

            assert!(
                complete_path.exists() && complete_path.is_dir(),
                "Directory not found {:?}",
                complete_path
            )
        }

        // remove test directories
        for i in start..start + iter {
            let dir_name = format!("{}{}", name, i);
            let mut complete_path = path.clone();
            complete_path.push(dir_name);

            let _ = std::fs::remove_dir_all(&complete_path);
        }
    }
}
