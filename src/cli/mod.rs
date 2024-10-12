use clap::Parser;
use std::path::Path;
use rayon::ThreadPoolBuilder;
use crate::search::bfs::{bfs_search, SearchOptions};

#[derive(Parser, Default, Debug)]
#[command(name = "rusfind")]
#[command(author = "Raphael Benzi raphael_benzi@hotmail.com")]
#[command(version = "0.1.0")]
#[command(about = "A simple finder like linux find", long_about = None)]
pub struct Cli {
    #[arg(short='p', long="path", default_value = ".", help="The directory to start the search")]
    path: String,
    #[arg(short='n', long="name", help = "The name or pattern to search for")]
    name: String,
    #[arg(short='t', long="f_type", help="Specify 'f' for files or 'd' for directories")]
    f_type: String,
    #[arg(short='r', long="threads", help="Number of threads to use for parallelism")]
    threads: usize,
}
/// Runs the CLI application.
pub fn run() {
    let cli = Cli::parse();

    let path = cli.path;
    let name_pattern = Some(cli.name.as_str());
    let file_type = Some(cli.f_type.as_str());
    let num_threads: usize = cli.threads;

    // Set the number of threads for Rayon
    ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();

    let options = SearchOptions {
        name_pattern,
        file_type,
    };

    let root = Path::new(&path);
    let results = bfs_search(root, options);

    for path in results {
        println!("{}", path.display());
    }
}
