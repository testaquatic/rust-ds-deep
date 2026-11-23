use std::collections::VecDeque;

/// 크기 `k`의 슬라이딩 윈도우에서 각 위치의 최솟값을 구한다.
fn sliding_window_min(arr: &[i32], k: usize) -> Vec<i32> {
    let n = arr.len();
    let mut result = Vec::with_capacity(n - k + 1);
    // deque에는 인덱스를 저장, arr[deque.front()]가 현재 윈도의 최솟값
    let mut deque = VecDeque::new();

    for i in 0..n {
        // 윈도우를 벗어난 인덱스를 앞에서 제거
        while let Some(front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        }

        // 현재 원소보다 크거나 같은 원소는 뒤에서 제거
        while let Some(&back) = deque.back() {
            if arr[back] >= arr[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        // 윈도우가 k크기가 된 시점부터 결과에 추가
        if i + 1 >= k {
            result.push(arr[*deque.front().unwrap()]);
        }
    }

    result
}

fn main() {
    let arr = vec![3, 1, 2, 4, 0, 5, 2, 1];
    let k = 3;
    let mins = sliding_window_min(&arr, k);
    println!("슬라이딩 윈도우(k={}) 최솟값: {:?}", k, mins);
    // [1, 1, 0, 0, 0, 1]
}
