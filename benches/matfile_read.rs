use criterion::{Criterion, criterion_group, criterion_main};

use matrw::*;

#[path = "../tests/shared_functions.rs"]
mod shared_functions;

use crate::shared_functions::{MatlabCommand, TestFile};

/// Read data from MAT-file
fn read_data(filepath: &str) {
    let _ = load_matfile(filepath).expect("Failure");
}

fn criterion_benchmark(c: &mut Criterion) {
    let filepath = TestFile {
        path: "benches/large.mat",
    };

    let out = MatlabCommand::run("benches/large_file.m");
    println!("error output: {}", out);

    let mut group = c.benchmark_group("read large MAT-files");
    group.sample_size(10);
    group.bench_function("read large MAT-file", |b| b.iter(|| read_data(filepath.path)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
