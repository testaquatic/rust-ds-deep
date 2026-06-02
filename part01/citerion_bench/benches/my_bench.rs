use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

fn process_slice(data: &[u64]) -> u64 {
    data.iter().sum()
}

fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("slice_mut");

    for size in [1000, 10000, 100000] {
        let data = (0..size).collect::<Vec<u64>>();

        group.throughput(Throughput::Elements(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| process_slice(black_box(data)))
        });
    }

    group.finish();
}
criterion_group!(benches, bench_throughput);
criterion_main!(benches);
