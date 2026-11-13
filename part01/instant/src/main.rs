use std::time::Instant;

/// O(1) - 배열의 첫 번째 원소 반환
fn o1_access(data: &[u64]) -> u64 {
    data[0]
}

/// O(log n) - 이진 탐색
fn o_log_n_search(data: &[u64], target: u64) -> bool {
    data.binary_search(&target).is_ok()
}

/// O(n) - 선형탐색
fn o_n_search(data: &[u64], target: u64) -> bool {
    data.iter().any(|x| *x == target)
}

/// O(n^2) - 버블 정렬 (측정 목적으로 단순 구현)
fn o_n_square_sort(data: &mut [u64]) {
    let n = data.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
            }
        }
    }
}

fn measure<F, R>(label: &str, n: usize, mut f: F)
where
    F: FnMut() -> R,
{
    let start = Instant::now();
    let _result = f();
    let elapsed = start.elapsed();
    println!(
        "{} (n={}): {:.3} us",
        label,
        n,
        elapsed.as_secs_f64() * 1_000_000.0
    );
}

fn main() {
    // 정렬된 데이터 준비 (이진 탐색을 위해 정렬 필요)
    let sizes = [100, 1_000, 10_000, 100_000];

    for n in sizes {
        let data = (0..n).collect::<Vec<u64>>();
        // 최악의 경우 탐색
        let target = n;

        println!("--- n = {} ---", n);

        // O(1): 크기가 바뀌어도 시간이 거의 바뀌지 않아야 함
        measure("O(1) 배열 접근", n as usize, || o1_access(&data));

        // O(log n): n이 10배 커지면 약 3.3배 느려져야 함
        measure("O(log n) 이진 탐색", n as usize, || {
            o_log_n_search(&data, target)
        });

        // O(n): n이 10배 커지면 약 10배 느려져야 함
        measure("O(n) 선형 탐색", n as usize, || {
            o_n_search(&data, target)
        });

        // O(n^2): n이 10배 커지면 약 100배 느려져야 함
        let mut unsorted = (0..n).rev().collect::<Vec<u64>>();
        measure("O(n^2) 버블 정렬", n as usize, || {
            o_n_square_sort(&mut unsorted)
        });

        println!();
    }
}
