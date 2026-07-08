use std::collections::BinaryHeap;

use max_heap::{MaxHeap, heap_sort};

fn main() {
    let mut heap = MaxHeap::new();
    [3, 1, 7, 5, 2, 8, 4].into_iter().for_each(|v| heap.push(v));

    println!("pop 순서: ");

    while let Some(v) = heap.pop() {
        print!("{} ", v);
    }
    println!();

    // 힙 정렬 테스트
    let mut data = vec![5, 3, 8, 1, 9, 2, 7, 4, 6];
    heap_sort(&mut data);
    println!("힙 정렬: {:?}", data);

    #[derive(Eq, PartialEq)]
    struct Task {
        priority: u32,
        name: String,
    }

    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.priority.cmp(&other.priority)
        }
    }

    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut scheduler = BinaryHeap::new();

    scheduler.push(Task {
        priority: 3,
        name: "저장 백업".into(),
    });
    scheduler.push(Task {
        priority: 10,
        name: "긴급 패치".into(),
    });
    scheduler.push(Task {
        priority: 7,
        name: "로그 분석".into(),
    });

    while let Some(task) = scheduler.pop() {
        println!("처리: [{}] {}", task.priority, task.name);
    }
}
