use std::{collections::LinkedList, hint::black_box};

use criterion::{BenchmarkId, Criterion};

fn vec_insert_front(n: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n as u64 {
        // 앞에 삽입
        v.insert(0, i);
    }
    v
}

fn linked_list_push_front(n: usize) -> LinkedList<u64> {
    let mut list = LinkedList::new();
    for i in 0..n as u64 {
        // 앞에 삽입
        list.push_front(i);
    }
    list
}

fn bench_insert_front(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_front");

    for size in [100, 500, 1_000, 5_000] {
        group.bench_with_input(
            BenchmarkId::new("Vec_insert_front", size),
            &size,
            |b, &size| b.iter(|| vec_insert_front(black_box(size))),
        );

        group.bench_with_input(
            BenchmarkId::new("LinkedList_push_front", size),
            &size,
            |b, &size| b.iter(|| linked_list_push_front(black_box(size))),
        );
    }
}

criterion::criterion_group!(benches, bench_insert_front);
criterion::criterion_main!(benches);
