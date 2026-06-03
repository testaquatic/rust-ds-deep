use std::hint::black_box;

use criterion::{BenchmarkId, Criterion};

fn bench_chunks(c: &mut Criterion) {
    let mut group = c.benchmark_group("chunks");

    let data = (1..=100_000).collect::<Vec<_>>();

    for size in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::new("chunks", size), &size, |b, &size| {
            b.iter(|| {
                let _sum = black_box(&data)
                    .chunks(size)
                    .map(|chunk| chunk.iter().sum::<i32>())
                    .sum::<i32>();
            });
        });
    }

    group.finish();
}

criterion::criterion_group!(benches, bench_chunks);
criterion::criterion_main!(benches);
