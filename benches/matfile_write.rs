use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use std::hint::black_box;

use matrw::*;

#[path = "../tests/shared_functions.rs"]
mod shared_functions;

use crate::shared_functions::TestFile;

/// Create some data and write it to MAT-file
fn write_big_matrix(n: usize) {
    let file = TestFile { path: "tests.mat" };

    let mut rng = Pcg64Mcg::seed_from_u64(1);

    // Preallocate vector and fill in-place
    let mut data = vec![0.0f64; n * n];
    for x in &mut data {
        *x = rng.random();
    }

    let var = MatVariable::NumericArray(NumericArray::new(vec![n, n], MatlabType::from(data), None).unwrap());

    let mat = matfile!(var: var);

    let _ = save_matfile_v7(file.path, mat, false);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    group.sample_size(10);
    group.bench_function("write MAT-file matrix", |b| {
        b.iter(|| write_big_matrix(black_box(15000)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
