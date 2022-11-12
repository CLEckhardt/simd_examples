#![allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_examples::{sum_up, simd_sum_up};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sum up", |b| {
        b.iter(|| sum_up(black_box((0..=1000000).collect::<Vec<i64>>())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
