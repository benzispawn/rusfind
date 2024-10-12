use rusfind::search::bfs::{bfs_search, SearchOptions};
use std::path::Path;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_bfs_search(c: &mut Criterion) {
    let root = Path::new(".");
    let options = SearchOptions {
        name_pattern: Some("src"),
        file_type: Some("d"),
    };

    c.bench_function("bfs_search", |b| {
        b.iter(|| {
            bfs_search(black_box(root), black_box(options.clone()))
        })
    });
}

criterion_group!(benches, benchmark_bfs_search);
criterion_main!(benches);
