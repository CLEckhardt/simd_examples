
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_examples::sum_up;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sum up", |b| b.iter(|| sum_up(black_box(1000000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

