use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};  // For thread-safe access
use rayon::prelude::*;
use crate::search::metadata_cache::MetadataCache;
use std::fs::{self, ReadDir};

/// Options for searching files and directories
#[derive(Clone)]
pub struct SearchOptions<'a> {
    pub name_pattern: Option<&'a str>,
    pub file_type: Option<&'a str>,
}

/// Performs a breadth-first search to find files and dirs

pub fn bfs_search<'a>(root: &'a Path, options: SearchOptions<'a>) -> Box<dyn Iterator<Item = PathBuf> + 'a + Send>/*Vec<PathBuf>*/ {
    // let mut results = Vec::new();
    // let results = Arc::new(Mutex::new(Vec::new()));
    let queue = Arc::new(Mutex::new(VecDeque::new()));// for directory
    let metadata_cache = Arc::new(Mutex::new(MetadataCache::new()));
    let files_queue = Arc::new(Mutex::new(VecDeque::new()));

    queue.lock().unwrap().push_back(PathBuf::from(root));
    // Create iterator that lazily processes the queue
    let iter = std::iter::from_fn(move || {
        loop {
            if let Some(matching_file) = files_queue.lock().unwrap().pop_front() {
                // println!("Yielding matching file: {:?}", matching_file);  // Debugging output
                return Some(matching_file);
            }

            let current_path = queue.lock().unwrap().pop_front();

            match current_path {
                Some(path) => {
                    let read_dir_result = fs::read_dir(&path);

                    if let Ok(read_dir) = read_dir_result {
                        process_directory(read_dir, &options, &queue, &metadata_cache, &files_queue);
                    }

                    if let Some(matching_file) = files_queue.lock().unwrap().pop_front() {
                        // println!("Yielding file after processing directory: {:?}", matching_file);
                        return Some(matching_file);
                    }
                    continue
                    // files_queue.lock().unwrap().pop_front()
                }, None => {
                    return None;
                },
            }
        }

    });

    Box::new(iter)

    // while let Some(current_path) = queue.lock().unwrap().pop_front() {
    //     if let Ok(entries) = fs::read_dir(&current_path) {
    //         // Convert entries into a parallel iterator using rayon
    //         let entries: Vec<_> = entries.filter_map(Result::ok).collect();
    //         entries.par_iter().for_each(|entry| {
    //             let path = entry.path();
    //             let file_name = entry.file_name();
    //             let file_name = file_name.to_string_lossy();
    //
    //             // Filter with given pattern
    //             if let Some(pattern) = options.name_pattern {
    //                 if !file_name.contains(pattern) {
    //                     return;
    //                 }
    //             }
    //
    //             // Get metadata from cache or file system
    //             let metadata = {
    //                 let cache = metadata_cache.lock().unwrap();
    //                 cache.get_metadata(&path)
    //             };
    //
    //             if let Some(metadata) = metadata {
    //                 // Filter by file type
    //                 let is_file = metadata.is_file();
    //                 let is_dir = metadata.is_dir();
    //
    //                 if let Some(file_type) = options.file_type {
    //                     if file_type == "f" && !is_file {
    //                         return;
    //                     } else if file_type == "d" && !is_dir {
    //                         return;
    //                     }
    //                 }
    //
    //                 let mut results_lock = results.lock().unwrap();
    //                 results_lock.push(path.clone());
    //
    //                 if is_dir {
    //                     let mut queue_lock = queue.lock().unwrap();
    //                     queue_lock.push_back(path);
    //                 }
    //             }
    //
    //         });
    //     }
    // }
    //
    // let final_results = Arc::try_unwrap(results)
    //     .unwrap()
    //     .into_inner()
    //     .unwrap();
    // final_results
}

pub fn process_directory(
    read_dir: ReadDir,
    options: &SearchOptions,
    queue: &Arc<Mutex<VecDeque<PathBuf>>>,
    metadata_cache: &Arc<Mutex<MetadataCache>>,
    files_queue: &Arc<Mutex<VecDeque<PathBuf>>>,
) {
    for entry_result in read_dir {
        if let Ok(entry) = entry_result {
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();

            // println!("Found file or directory: {:?}", path);

            // Get metadata from cache or file system
            let metadata = {
                let cache = metadata_cache.lock().unwrap();
                cache.get_metadata(&path)
            };

            if let Some(metadata) = metadata {
                let is_file = metadata.is_file();
                let is_dir = metadata.is_dir();

                // If the entry is a directory, push it to the queue for further processing
                if is_dir {
                    let mut queue_lock = queue.lock().unwrap();
                    queue_lock.push_back(path.clone());
                    // println!("Enqueued directory(for traversal): {:?}", path);
                }

                // Apply filters before adding to `files_queue`
                if !apply_filters(&options, &file_name, is_file, is_dir) {
                    // println!("Skipped: {:?}", path);  // Log skipped file/directory due to filtering
                    continue;  // Skip if the filters don’t match
                }

                // Add the matching file or directory to the files_queue
                let mut files_lock = files_queue.lock().unwrap();
                files_lock.push_back(path.clone());
                // println!("Enqueued matching file: {:?}", path);

            }
        }
    }
}

fn apply_filters(options: &SearchOptions, file_name: &str, is_file: bool, is_dir: bool) -> bool {
    // Convert file name to lowercase for case-insensitive matching
    let file_name_lower = file_name.to_lowercase();

    // Filter by name pattern if provided
    if let Some(pattern) = options.name_pattern {
        let pattern_lower = pattern.to_lowercase();
        if !file_name_lower.contains(&pattern_lower.as_str()) {
            return false;  // Skip if the file name doesn't match the pattern
        }
    }

    // Filter by file type: 'f' for files, 'd' for directories
    if let Some(file_type) = options.file_type {
        if file_type == "f" && !is_file {
            return false;  // Skip if we’re looking for files and this is not a file
        } else if file_type == "d" && !is_dir {
            return false;  // Skip if we’re looking for directories and this is not a directory
        }
    }

    // If the filters match, return true
    true
}

// basic test to see if it works
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs::{self, File};
//     use std::io::Write;
//     use tempfile::tempdir;
//
//     #[test]
//     fn test_bfs_search() {
//         let dir = tempdir().unwrap();
//         let dir_path = dir.path();
//
//         // Create some test files and directories
//         fs::create_dir(dir_path.join("subdir")).unwrap();
//         let mut file1 = File::create(dir_path.join("file1.txt")).unwrap();
//         writeln!(file1, "Hello, world!").unwrap();
//         let mut file2 = File::create(dir_path.join("subdir/file2.rs")).unwrap();
//         writeln!(file2, "fn main() {{}}").unwrap();
//
//         let options = SearchOptions {
//             name_pattern: Some("file"),
//             file_type: Some("f"),
//         };
//
//         let results = bfs_search(dir_path, options);
//         assert_eq!(results.len(), 2);
//         assert!(results.contains(&dir_path.join("file1.txt")));
//         assert!(results.contains(&dir_path.join("subdir/file2.rs")));
//     }
// }