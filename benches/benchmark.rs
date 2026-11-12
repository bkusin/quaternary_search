use bvec::quaternary_search;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const SLICE_LEN: i32 = 100;
const TARGET: i32 = 73;

fn bench_quaternary_search(c: &mut Criterion) {
    let data: Vec<i32> = (0..SLICE_LEN).collect();

    c.bench_function("quaternary_search/len_100", |b| {
        b.iter(|| black_box(quaternary_search(black_box(&data), black_box(TARGET))))
    });
}

fn bench_binary_search(c: &mut Criterion) {
    let data: Vec<i32> = (0..SLICE_LEN).collect();

    c.bench_function("binary_search/len_100", |b| {
        b.iter(|| black_box(data.binary_search(black_box(&TARGET))))
    });
}

criterion_group!(benches, bench_quaternary_search, bench_binary_search);
criterion_main!(benches);