use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

fn linear_search(data: &[u64], target: u64) -> bool {
    data.iter().any(|x| *x == target)
}

fn binary_search(data: &[u64], target: u64) -> bool {
    data.binary_search(&target).is_ok()
}

fn bench_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("search");

    for size in [1000, 10000, 100000] {
        let data = (0..size).collect::<Vec<u64>>();
        let target = size as u64 - 1;

        group.throughput(Throughput::Elements(size));
        group.bench_with_input(
            BenchmarkId::new("linear", size),
            &(&data, target),
            |b, &(data, target)| {
                b.iter(|| linear_search(black_box(data), black_box(target)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("binary", size),
            &(&data, target),
            |b, &(data, target)| b.iter(|| binary_search(black_box(&data), black_box(target))),
        );
    }
}

criterion_group!(benches, bench_search);
criterion_main!(benches);
