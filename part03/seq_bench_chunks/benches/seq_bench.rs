use criterion::{BenchmarkId, Criterion};

fn bench_chunks(c: &mut Criterion) {
    let data = (1..=100_000).collect::<Vec<_>>();

    let mut group = c.benchmark_group("chunks");
    for size in [100, 1000, 10000] {
        group.bench_with_input(BenchmarkId::new("chunks", size), &size, |b, &s| {
            b.iter(|| data.chunks(s).map(|chunk| chunk.iter().sum::<i32>()))
        });
    }

    group.finish();
}

criterion::criterion_group!(benches, bench_chunks);
criterion::criterion_main!(benches);
