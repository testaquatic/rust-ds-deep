use std::{collections::HashSet, hint::black_box};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_contains(c: &mut Criterion) {
    let size = [100, 1000, 10000, 100000];

    let mut group = c.benchmark_group("contains");

    for &n in size.iter() {
        let data = (0..n).collect::<Vec<u32>>();
        let set = HashSet::<u32>::from_iter(data.iter().copied());
        let target = n as u32 - 1;

        group.bench_with_input(BenchmarkId::new("Vec", n), &n, |b, _| {
            b.iter(|| black_box(data.contains(&target)));
        });

        group.bench_with_input(BenchmarkId::new("HashSet", n), &n, |b, _| {
            b.iter(|| black_box(set.contains(&target)));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_contains);

criterion_main!(benches);
