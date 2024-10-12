use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

/// Options for searching files and directories
#[derive(Clone)]
pub struct SearchOptions<'a> {
    pub name_pattern: Option<&'a str>,
    pub file_type: Option<&'a str>,
}

/// Performs a breadth-first search to find files and dirs

pub fn bfs_search(root: &Path, options: SearchOptions) -> Vec<PathBuf> {
    let mut results = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(PathBuf::from(root));

    while let Some(current_path) = queue.pop_front() {
        if let Ok(entries) = fs::read_dir(&current_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let file_name = entry.file_name();
                    let file_name = file_name.to_string_lossy();

                    // Filter with given pattern
                    if let Some(pattern) = options.name_pattern {
                        if !file_name.contains(pattern) {
                            continue;
                        }
                    }

                    // Filter by file type
                    let is_file = path.is_file();
                    let is_dir = path.is_dir();

                    if let Some(file_type) = options.file_type {
                        if file_type == "f" && !is_file {
                            continue;
                        } else if file_type == "d" && !is_dir {
                            continue;
                        }
                    }

                    results.push(path.clone());

                    if is_dir {
                        queue.push_back(path);
                    }
                }
            }
        }
    }

    results
}


/// basic test to see if it works
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_bfs_search() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create some test files and directories
        fs::create_dir(dir_path.join("subdir")).unwrap();
        let mut file1 = File::create(dir_path.join("file1.txt")).unwrap();
        writeln!(file1, "Hello, world!").unwrap();
        let mut file2 = File::create(dir_path.join("subdir/file2.rs")).unwrap();
        writeln!(file2, "fn main() {{}}").unwrap();

        let options = SearchOptions {
            name_pattern: Some("file"),
            file_type: Some("f"),
        };

        let results = bfs_search(dir_path, options);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&dir_path.join("file1.txt")));
        assert!(results.contains(&dir_path.join("subdir/file2.rs")));
    }
}