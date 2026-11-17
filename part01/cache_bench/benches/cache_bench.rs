use std::{collections::LinkedList, hint};

use criterion::{BenchmarkId, Criterion};

fn sum_iter<'a>(data: impl Iterator<Item = &'a u64>) -> u64 {
    data.sum()
}

fn bench_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_friendliness");

    for size in [1000, 10000, 100000] {
        // Vec: 연속 메모리
        let vec_data = (0..size).collect::<Vec<u64>>();
        // LinkedList: 흩어진 메모리
        let list_data = (0..size).collect::<LinkedList<_>>();

        group.bench_with_input(BenchmarkId::new("Vec", size), &vec_data, |b, data| {
            b.iter(|| sum_iter(hint::black_box(data.iter())));
        });

        group.bench_with_input(
            BenchmarkId::new("LinkedList", size),
            &list_data,
            |b, data| {
                b.iter(|| sum_iter(hint::black_box(data.iter())));
            },
        );
    }
}

criterion::criterion_group!(benches, bench_cache);
criterion::criterion_main!(benches);
