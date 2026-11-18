use std::{collections::HashMap, hint};

use criterion::{BenchmarkId, Criterion};

fn vec_contains<'a>(data: &'a [(u32, &str)], key: u32) -> Option<&'a str> {
    data.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

fn hashmap_get<'a>(map: &'a HashMap<u32, &str>, key: u32) -> Option<&'a str> {
    map.get(&key).map(|v| *v)
}

fn bench_small_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("small lookup");

    for size in [8, 16, 32, 64, 128, 256] {
        let vec_data = (0..size as u32).map(|i| (i, "value")).collect::<Vec<_>>();
        let hashmap_data = vec_data.iter().copied().collect::<HashMap<_, _>>();

        // 최악의 경우 탐색
        let target = size as u32 - 1;

        group.bench_with_input(
            BenchmarkId::new("Vec_linear", size),
            &(&vec_data, target),
            |b, &(data, target)| {
                b.iter(|| vec_contains(hint::black_box(data), hint::black_box(target)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("HashMap_get", size),
            &(&hashmap_data, target),
            |b, &(map, target)| {
                b.iter(|| hashmap_get(hint::black_box(map), hint::black_box(target)));
            },
        );
    }
}

criterion::criterion_group!(benches, bench_small_lookup);
criterion::criterion_main!(benches);
