use std::hint;

use criterion::BenchmarkId;

fn fill_without_capacity(n: usize) -> Vec<u64> {
    // 초기 용량 0
    let mut v = Vec::new();
    for i in 0..n as u64 {
        // n에 따라서 재할당 발생
        v.push(i);
    }

    v
}

fn fill_with_capacity(n: usize) -> Vec<u64> {
    // 처음부터 n개 예약
    let mut v = Vec::with_capacity(n);
    for i in 0..n as u64 {
        // 재할당 없음
        v.push(i);
    }

    v
}

fn bench_capacity(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("vec_capacity");

    for size in [1_000, 10_000, 100_000, 1_000_000] {
        group.bench_with_input(
            BenchmarkId::new("without_capacity", size),
            &size,
            |b, &size| b.iter(|| fill_without_capacity(hint::black_box(size))),
        );

        group.bench_with_input(
            BenchmarkId::new("with_capacity", size),
            &size,
            |b, &size| b.iter(|| fill_with_capacity(hint::black_box(size))),
        );
    }
}

criterion::criterion_group!(benches, bench_capacity);
criterion::criterion_main!(benches);
