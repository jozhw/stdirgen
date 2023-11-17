use assert_cmd::Command;
use std::path::PathBuf;
use tempfile::TempDir;

// without --path
#[test]
fn test_dir_command_without_subfile() {
    // create temporary dir to house all testing files
    let temp_dir: TempDir = tempfile::tempdir().expect("Failed to create temporary directory.");

    let assert = Command::cargo_bin("stdirgen")
        .expect("Failed to find binary")
        .current_dir(temp_dir.path())
        .args(&[
            "directory",
            "--name",
            "test_dir",
            "--iter",
            "2",
            "--start",
            "2",
        ])
        .assert();

    assert.success();

    // check to see if generated dirs were successful
    let name: &str = "test_dir";
    let iter: i32 = 2;
    let start: i32 = 2;

    // check for trailing hence the + 1
    for i in start..start + iter + 1 {
        let dir_name: String = format!("{}{}", &name, i);
        let gen_dir_path: PathBuf = temp_dir.path().join(dir_name);
        if i < start + iter {
            assert!(gen_dir_path.exists() && gen_dir_path.is_dir());
        } else {
            assert!(!gen_dir_path.exists());
        }
    }

    // delete temp dir
    temp_dir
        .close()
        .expect("Failed to delete temporary directory");
}

// with --path
#[test]
fn test_dir_command_with_subfile() {
    let temp_dir: TempDir = tempfile::tempdir().expect("Failed to create temporary directory.");

    let assert = Command::cargo_bin("stdirgen")
        .expect("Failed to find binary")
        .current_dir(temp_dir.path())
        .args(&[
            "directory",
            "--name",
            "test_dir",
            "--path",
            temp_dir.path().to_str().unwrap(),
            "--iter",
            "2",
            "--start",
            "2",
        ])
        .arg("--files")
        .arg("test1_file.txt")
        .arg("--files")
        .arg("test2_file.txt")
        .arg("--files-iter")
        .arg("2")
        .arg("--files-iter")
        .arg("3")
        .arg("--files-start")
        .arg("1")
        .arg("--files-start")
        .arg("2")
        .assert();

    assert.success();

    // check to see if generated dirs were successful
    let d_name: &str = "test_dir";
    let d_iter: i32 = 2;
    let d_start: i32 = 2;
    let f_names: Vec<&str> = vec!["test1_file", "test2_file"];
    let f_length = 2;
    let f_exts: Vec<&str> = vec![".txt", ".txt"];
    let f_starts: Vec<i32> = vec![1, 2];
    let f_iters: Vec<i32> = vec![2, 3];

    // no need to test for exta directory since the previous test does just that
    for i in d_start..d_start + d_iter {
        let dir_name: String = format!("{}{}", &d_name, i);
        let gen_dir_path: PathBuf = temp_dir.path().join(&dir_name);

        assert!(gen_dir_path.exists() && gen_dir_path.is_dir());

        // check if subfile exists
        for j in 0..f_length {
            let f_name = f_names[j];
            let f_iter = f_iters[j];
            let f_start = f_starts[j];
            let f_ext = f_exts[j];

            // test if extra file
            for k in f_start..f_start + f_iter + 1 {
                let f_enumer_name = format!("{}{}{}", &f_name, &k, &f_ext);
                let gen_subfile_path = gen_dir_path.clone().join(&f_enumer_name);
                if k < f_start + f_iter {
                    assert!(gen_subfile_path.exists() && gen_subfile_path.is_file());
                } else {
                    assert!(!gen_subfile_path.exists());
                }
            }
        }
    }

    // delete temp dir
    temp_dir
        .close()
        .expect("Failed to delete temporary directory");
}
