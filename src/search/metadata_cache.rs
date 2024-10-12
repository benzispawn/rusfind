use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf};
use std::sync::{Arc, Mutex};  // To share cache across threads

/// Structure to cache metadata for files and directories.
#[derive(Default)]
pub struct MetadataCache {
    cache: Mutex<HashMap<PathBuf, fs::Metadata>>,  // Thread-safe HashMap for caching
}

impl MetadataCache {
    /// Creates a new MetadataCache.
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Get metadata from cache or retrieve it from the file system if not cached.
    pub fn get_metadata(&self, path: &PathBuf) -> Option<fs::Metadata> {
        let mut cache_lock = self.cache.lock().unwrap();

        // If metadata is already in the cache, return it.
        if let Some(metadata) = cache_lock.get(path) {
            return Some(metadata.clone());
        }

        // If not in cache, get metadata from the file system.
        if let Ok(metadata) = fs::metadata(path) {
            cache_lock.insert(path.clone(), metadata.clone());
            Some(metadata)
        } else {
            None
        }
    }
}