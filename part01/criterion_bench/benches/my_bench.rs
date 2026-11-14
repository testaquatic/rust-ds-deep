use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn sum_to_n(n: u64) -> u64 {
    (0..n).sum()
}

fn bench_sum(c: &mut Criterion) {
    c.bench_function("sum_to_1000", |b| {
        b.iter(|| sum_to_n(black_box(1000)));
    });
}

criterion_group!(benches, bench_sum);
criterion_main!(benches);
