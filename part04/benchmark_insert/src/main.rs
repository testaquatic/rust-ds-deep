use std::collections::{BTreeMap, HashMap};

fn benchmark_insert(n: usize) {
    let start = std::time::Instant::now();

    let mut hmap = HashMap::with_capacity(n);
    (0..n).for_each(|i| {
        hmap.insert(i, i * 2);
    });
    let hash_time = start.elapsed();

    let start = std::time::Instant::now();
    let mut bmap = BTreeMap::new();
    (0..n).for_each(|i| {
        bmap.insert(i, i * 2);
    });
    let btree_time = start.elapsed();

    println!("n = {}, hash: {:?}, btree: {:?}", n, hash_time, btree_time);
}

fn main() {
    benchmark_insert(100_000);
    benchmark_insert(1_000_000);
}
