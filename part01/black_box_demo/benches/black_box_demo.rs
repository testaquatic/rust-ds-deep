use std::hint::{self, black_box};

use criterion::Criterion;

fn sum_to_n(n: u64) -> u64 {
    (0..n).sum()
}

// 올바른 사용: 입력과 결과를 black_box로 감삼
fn bench_correct(c: &mut Criterion) {
    c.bench_function("sum_corrent", |b| {
        b.iter(|| {
            let result = sum_to_n(hint::black_box(1000));
            black_box(result);
        });
    });
}

// 잘못된 사용: black_box 없음 => 컴파일러가 최적화로 없앨 수 있음
fn bench_wrong(c: &mut Criterion) {
    c.bench_function("sum_wrong", |b| {
        b.iter(|| sum_to_n(1000));
    });
}

criterion::criterion_group!(benches, bench_correct, bench_wrong);
criterion::criterion_main!(benches);
