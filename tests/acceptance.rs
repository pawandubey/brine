use std::{env, fs, process::Command};

use test_case::test_case;

#[test_case("chap04_scanning")]
fn acceptance_test_for(chapter: &str) {
    let binary_path_var = std::env::var("BINARY_PATH").expect("BINARY_PATH env var required");
    let dart_dir_var = std::env::var("DART_DIR").expect("DART_DIR env var required");

    let binary_path = fs::canonicalize(binary_path_var).unwrap();
    let dart_dir = fs::canonicalize(dart_dir_var).unwrap();

    assert!(binary_path.exists());
    assert!(dart_dir.exists());

    let original_dir = env::current_dir().expect("Failed to get pwd");

    env::set_current_dir(dart_dir).expect("Failed to change directory");

    let output = Command::new("dart")
        .arg("tool/bin/test.dart")
        .arg(chapter)
        .arg("--interpreter")
        .arg(binary_path)
        .output()
        .expect("Failed to execute command");

    env::set_current_dir(original_dir).expect("Failed to change directory");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    assert!(output.status.success())
}
