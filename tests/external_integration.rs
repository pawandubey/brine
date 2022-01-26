use std::{process::Command, path::PathBuf, fs};

use test_case::test_case;

#[test_case("chap04_scanning")]
fn acceptance_test(chapter: &str) {
    let binary_path_var = std::env::var("BINARY_PATH").expect("BINARY_PATH env var required");
    let dart_dir_var = std::env::var("DART_DIR").expect("DART_DIR env var required");

    let binary_path = fs::canonicalize(binary_path_var).unwrap();
    let dart_dir = fs::canonicalize(dart_dir_var).unwrap();

    assert!(binary_path.exists());
    assert!(dart_dir.exists());

    let original_dir = std::env::current_dir().expect("Failed to get pwd");

    std::env::set_current_dir(PathBuf::from(dart_dir)).expect("Failed to change directory");

    let mut cmd = Command::new("dart");

    cmd.arg("tool/bin/test.dart")
        .arg(chapter)
        .arg("--interpreter")
        .arg(binary_path);

    println!("Command: {:?}", cmd);

    let output = cmd.output()
        .expect("Failed to execute command");

    std::env::set_current_dir(original_dir).expect("Failed to change directory");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    assert!(output.status.success())
}
