use std::time::Instant;

/// O(1)
fn o1_access(data: &[u64]) -> bool {
    data.first().is_some()
}

/// O(log n)
fn o_log_n_search(data: &[u64], target: u64) -> bool {
    data.binary_search(&target).is_ok()
}

/// O(n)
fn o_n_search(data: &[u64], target: u64) -> bool {
    data.iter().any(|x| *x == target)
}

/// O(n^2)
fn o_n2_squared_sort(data: &mut [u64]) {
    let n = data.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
            }
        }
    }
}

fn measure<R>(label: &str, n: usize, mut f: impl FnMut() -> R) {
    let start = Instant::now();
    let _result = f();
    let elapsed = start.elapsed();
    println!(
        "{} (n = {n}): {:.3} us",
        label,
        elapsed.as_secs_f64() * 1_000_000.0
    );
}

fn main() {
    let sizes = [100, 1000, 10000, 100000];

    for n in sizes {
        let data = (0..n as u64).collect::<Vec<_>>();
        let target = (n - 1) as u64;

        println!("--- n = {} ---", n);

        measure("O(1) 배열 접근", n, || o1_access(&data));
        measure("O(log n) 이진 탐색", n, || {
            o_log_n_search(&data, target)
        });
        measure("O(n) 선형 탐색", n, || o_n_search(&data, target));

        if n <= 10000 {
            let mut unsorted = (0..n as u64).rev().collect::<Vec<_>>();
            measure("O(n^2) 버블 정렬", n, || {
                o_n2_squared_sort(&mut unsorted)
            });
        }
    }
}
