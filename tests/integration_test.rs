use std::process::Command;
use tempfile::tempdir;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_cli_find_files() {
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    // Create some test files and directories
    fs::create_dir(dir_path.join("subdir")).unwrap();
    let mut file1 = File::create(dir_path.join("file1.txt")).unwrap();
    writeln!(file1, "Hello, world!").unwrap();
    let mut file2 = File::create(dir_path.join("subdir/file2.rs")).unwrap();
    writeln!(file2, "fn main() {{}}").unwrap();

    // Run the CLI command
    let output = Command::new("cargo")
        .args(&["run", "--", dir_path.to_str().unwrap(), "-n", "file", "-t", "f"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("file1.txt"));
    assert!(stdout.contains("subdir/file2.rs"));
}
