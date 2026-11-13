use std::{
    collections::HashSet,
    hint::{self},
};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_contains(c: &mut Criterion) {
    let size = [100, 1_000, 10_000, 100_000];
    let mut group = c.benchmark_group("contains");

    for &n in size.iter() {
        let data = (0..n as u32).collect::<Vec<_>>();
        let set = (0..n as u32).collect::<HashSet<_>>();
        let target = n as u32 - 1;

        group.bench_with_input(BenchmarkId::new("Vec", n), &n, |b, _| {
            b.iter(|| hint::black_box(data.contains(hint::black_box(&target))));
        });

        group.bench_with_input(BenchmarkId::new("HashSet", n), &n, |b, _| {
            b.iter(|| hint::black_box(set.contains(hint::black_box(&target))));
        });
    }
}

criterion_group!(benches, bench_contains);
criterion_main!(benches);
