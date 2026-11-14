use std::{hint, iter::Sum};

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

fn process_slice<'a, T>(data: &'a [T]) -> T
where
    T: Sum<&'a T>,
{
    data.iter().sum()
}

fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("slice_sum");

    for size in [1_000, 10_000, 100_000] {
        let data = (0..size).collect::<Vec<u64>>();

        // throughput을 설정하면 "elements/seconds" 단위로 결과가 나옴
        group.throughput(Throughput::Elements(size));

        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| process_slice(hint::black_box(data)));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);
