use bvec::quaternary_search;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use random::*;
use std::hint::black_box;

const SLICE_LEN: i32 = 100_000;
const TARGET: i32 = 73;

fn bench_quaternary_search(c: &mut Criterion) {
    let data: Vec<i32> = (0..SLICE_LEN).collect();

    c.bench_function("quaternary_search/len_100_000", |b| {
        b.iter(|| black_box(quaternary_search(black_box(&data), black_box(TARGET))))
    });
}

fn bench_binary_search(c: &mut Criterion) {
    let data: Vec<i32> = (0..SLICE_LEN).collect();

    c.bench_function("binary_search/len_100_000", |b| {
        b.iter(|| black_box(data.binary_search(black_box(&TARGET))))
    });
}

// ====================

fn bench_quaternary_search_random(c: &mut Criterion) {
    let data: Vec<i32> = (0..SLICE_LEN).collect();
    
    let targets: Vec<i32> =  {
        let rand_length: usize = 4096;
        let mut rng = random::default(42);
        rng.iter().take(rand_length).collect()
    };

    let mut next_target = 0;

    c.bench_function("quaternary_search_random/len_100_000", |b| {
        b.iter_batched(
            || {
                let target = targets[next_target];
                next_target = (next_target + 1) % targets.len();
                target
            },
            |target| black_box(quaternary_search(black_box(&data), black_box(target))),
            BatchSize::SmallInput,
        )
    });
}

fn bench_binary_search_random(c: &mut Criterion) {
    let data: Vec<i32> = (0..SLICE_LEN).collect();
    let targets: Vec<i32> =  {
        let rand_length: usize = 4096;
        let mut rng = random::default(42);
        rng.iter().take(rand_length).collect()
    };
    
    let mut next_target = 0;

    c.bench_function("binary_search_random/len_100_000", |b| {
        b.iter_batched(
            || {
                let target = targets[next_target];
                next_target = (next_target + 1) % targets.len();
                target
            },
            |target| black_box(data.binary_search(black_box(&target))),
            BatchSize::SmallInput,
        )
    });
}


criterion_group!(
    benches,
    bench_quaternary_search,
    bench_binary_search,
    bench_quaternary_search_random,
    bench_binary_search_random
);
criterion_main!(benches);