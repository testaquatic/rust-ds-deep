use std::{collections::HashMap, hint};

use criterion::{BenchmarkId, Criterion};
use rustc_hash::FxHashMap;

fn bench_hashmap(n: usize) -> u64 {
    let mut map = HashMap::with_capacity(n);
    for i in 0..n as u64 {
        map.insert(i, i);
    }
    let mut sum = 0u64;
    for i in 0..n as u64 {
        sum = sum.wrapping_add(*map.get(&i).unwrap());
    }

    sum
}

fn bench_fxhashmap(n: usize) -> u64 {
    let mut map = FxHashMap::with_capacity_and_hasher(n, Default::default());
    for i in 0..n as u64 {
        map.insert(i, i);
    }
    let mut sum = 0u64;
    for i in 0..n as u64 {
        sum = sum.wrapping_add(*map.get(&i).unwrap());
    }

    sum
}

fn bench_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_benchmark");
    for size in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::new("HashMap", &size), &size, |b, &size| {
            b.iter(|| {
                let result = bench_hashmap(hint::black_box(size));
                hint::black_box(result)
            });
        });

        group.bench_with_input(BenchmarkId::new("FxHashMap", &size), &size, |b, &size| {
            b.iter(|| {
                let result = bench_fxhashmap(hint::black_box(size));
                hint::black_box(result)
            });
        });
    }

    group.finish();
}

criterion::criterion_group!(benches, bench_map);
criterion::criterion_main!(benches);
