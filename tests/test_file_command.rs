use assert_cmd::Command;
use std::path::PathBuf;
use tempfile::TempDir;

// without --path
#[test]
fn test_file_command_without_path() {
    // create temporary dir to house all testing files
    let temp_dir: TempDir = tempfile::tempdir().expect("Failed to create temporary directory.");

    let assert = Command::cargo_bin("stdirgen")
        .expect("Failed to find binary")
        .current_dir(temp_dir.path())
        .args(&[
            "file",
            "--name",
            "test_file.txt",
            "--iter",
            "2",
            "--start",
            "2",
        ])
        .assert();

    assert.success();

    // check to see if generated files were successful
    let name: &str = "test_file";
    let ext: &str = ".txt";
    let iter: i32 = 2;
    let start: i32 = 2;

    // check for trailing hence the + 1
    for i in start..start + iter + 1 {
        let file_name: String = format!("{}{}{}", &name, &i, &ext);
        let gen_file_path: PathBuf = temp_dir.path().join(file_name);
        if i < start + iter {
            assert!(gen_file_path.exists() && gen_file_path.is_file());
        } else {
            assert!(!gen_file_path.exists());
        }
    }

    // delete temp dir
    temp_dir
        .close()
        .expect("Failed to delete temporary directory");
}

// with --path
#[test]
fn test_file_command_with_path() {
    let temp_dir: TempDir = tempfile::tempdir().expect("Failed to create temporary directory.");

    let assert = Command::cargo_bin("stdirgen")
        .expect("Failed to find binary")
        .args(&[
            "file",
            "--name",
            "test_file.txt",
            "--path",
            temp_dir.path().to_str().unwrap(),
            "--iter",
            "2",
            "--start",
            "2",
        ])
        .assert();

    assert.success();

    // check to see if generated files were successful
    let name: &str = "test_file";
    let ext: &str = ".txt";
    let iter: i32 = 2;
    let start: i32 = 2;

    // check for trailing hence the + 1
    for i in start..start + iter + 1 {
        let file_name: String = format!("{}{}{}", &name, &i, &ext);
        let gen_file_path: PathBuf = temp_dir.path().join(file_name);
        if i < start + iter {
            assert!(gen_file_path.exists() && gen_file_path.is_file());
        } else {
            assert!(!gen_file_path.exists());
        }
    }

    // delete temp dir
    temp_dir
        .close()
        .expect("Failed to delete temporary directory");
}
