use std::{collections::HashMap, hint::black_box};

use criterion::{BenchmarkId, Criterion};

fn vec_contains<'a>(data: &'a [(u32, &str)], key: u32) -> Option<&'a &'a str> {
    data.iter()
        .find_map(|(k, v)| if *k == key { Some(v) } else { None })
}

fn hashmap_get<'a>(map: &'a HashMap<u32, &str>, key: u32) -> Option<&'a &'a str> {
    map.get(&key)
} // (HashMap<u32, &str>)

fn bench_small_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_lookup");

    for size in [8, 16, 32, 64, 128, 256] {
        let vec_data = (0..size).map(|i| (i, "value")).collect::<Vec<_>>();
        let map = vec_data.iter().copied().collect::<HashMap<_, _>>();
        let target = size as u32 - 1;

        group.bench_with_input(
            BenchmarkId::new("vec_contains", size),
            &target,
            |b, &target| b.iter(|| vec_contains(black_box(&vec_data), black_box(target))),
        );

        group.bench_with_input(
            BenchmarkId::new("hashmap_get", size),
            &target,
            |b, &target| b.iter(|| hashmap_get(black_box(&map), black_box(target))),
        );
    }
}

criterion::criterion_group!(benches, bench_small_lookup);
criterion::criterion_main!(benches);
