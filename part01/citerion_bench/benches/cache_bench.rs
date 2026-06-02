use std::{collections::LinkedList, hint::black_box};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn sum_vec(data: &[u64]) -> u64 {
    data.iter().sum()
}

fn sum_linked_list(data: &LinkedList<u64>) -> u64 {
    data.iter().sum()
}

fn bench_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_friendliness");

    for size in [1_000, 10_000, 100_000] {
        let vec_data = (0..size).collect::<Vec<u64>>();
        let linked_list_data = (0..size).collect::<LinkedList<u64>>();

        group.bench_with_input(BenchmarkId::new("Vec", size), &vec_data, |b, data| {
            b.iter(|| sum_vec(black_box(data)))
        });

        group.bench_with_input(
            BenchmarkId::new("LinkedList", size),
            &linked_list_data,
            |b, data| b.iter(|| sum_linked_list(black_box(data))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_cache);
criterion_main!(benches);
