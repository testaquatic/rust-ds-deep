use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn fill_without_capacity(n: usize) -> Vec<u64> {
    let mut v = Vec::new();
    for i in 0..n as u64 {
        v.push(i);
    }
    v
}

fn fill_with_capacity(n: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n as u64 {
        v.push(i);
    }
    v
}

fn bench_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_capacity");

    for size in [1_000, 10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::new("with_capacity", size), &size, |b, size| {
            let size = *size;
            b.iter(|| fill_with_capacity(black_box(size)))
        });

        group.bench_with_input(
            BenchmarkId::new("without_capacity", size),
            &size,
            |b, size| {
                let size = *size;
                b.iter(|| fill_without_capacity(black_box(size)))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_capacity);
criterion_main!(benches);
