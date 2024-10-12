use rusfind::search::bfs::{bfs_search, SearchOptions};
use std::path::Path;

fn main() {
    let root = Path::new(".");
    let options = SearchOptions {
        name_pattern: Some("main"),
        file_type: Some("f"),
    };

    let results = bfs_search(root, options);

    for path in results {
        println!("{}", path.display());
    }
}
