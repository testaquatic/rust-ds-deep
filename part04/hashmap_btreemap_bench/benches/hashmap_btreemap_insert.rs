use std::{
    collections::{BTreeMap, HashMap},
    hint,
};

use criterion::{BenchmarkId, Criterion};

fn hashmap_insert(n: u64) {
    let mut hmap = HashMap::with_capacity(n as usize);
    for i in 0..n {
        hmap.insert(i, i * 2);
    }
}

fn btreemap_insert(n: u64) {
    let mut btree = BTreeMap::new();
    for i in 0..n {
        btree.insert(i, i * 2);
    }
}

fn bench_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_benchmark");
    for size in [100, 1_000, 10_000, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::new("hashmap", size), &size, |b, &n| {
            b.iter(|| hashmap_insert(hint::black_box(n)))
        });
        group.bench_with_input(BenchmarkId::new("btreemap", size), &size, |b, &n| {
            b.iter(|| btreemap_insert(hint::black_box(n)))
        });
    }

    group.finish();
}

criterion::criterion_group!(benches, bench_map);
criterion::criterion_main!(benches);
