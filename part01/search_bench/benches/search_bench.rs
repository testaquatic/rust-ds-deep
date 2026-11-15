use std::hint;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn linear_search<T>(data: &[T], target: T) -> bool
where
    T: PartialEq,
{
    data.iter().any(|x| x == &target)
}

fn binary_search<T>(data: &[T], target: T) -> bool
where
    T: Ord,
{
    data.binary_search(&target).is_ok()
}

fn bench_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("search");

    for &size in &[100, 1000, 10000, 100000] {
        // 정렬된 상태
        let data = (0..size).collect::<Vec<u64>>();
        // 최악의 경우
        let target = size as u64 - 1;

        group.bench_with_input(
            BenchmarkId::new("linear", size),
            &(&data, target),
            |b, (data, target)| {
                b.iter(|| linear_search(hint::black_box(data.as_ref()), hint::black_box(*target)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("binary", size),
            &(&data, target),
            |b, (data, target)| {
                b.iter(|| binary_search(hint::black_box(data.as_ref()), hint::black_box(*target)));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_search);
criterion_main!(benches);
