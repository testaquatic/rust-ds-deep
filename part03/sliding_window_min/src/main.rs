use std::collections::VecDeque;

fn sliding_window_min(arr: &[i32], k: usize) -> Vec<i32> {
    let n = arr.len();
    let mut result = Vec::with_capacity(n - k + 1);
    let mut deque = VecDeque::new();

    for i in 0..n {
        while let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        }

        while let Some(&back) = deque.back() {
            if arr[back] >= arr[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        if i + 1 >= k {
            result.push(arr[*deque.front().unwrap()]);
        }
    }

    result
}

fn main() {
    let arr = vec![3, 1, 2, 4, 0, 5, 2, 1];
    let k = 3;
    let result = sliding_window_min(&arr, k);
    println!("{:?}", result);
}
