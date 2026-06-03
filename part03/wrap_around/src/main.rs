use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::from([1, 2, 3, 4, 5]);

    deque.push_front(0);
    deque.push_front(-1);

    println!("{:?}", deque);

    println!("인덱스 2: {}", deque[2]);

    let (first, second) = deque.as_slices();
    println!("first: {:?}, second: {:?}", first, second);

    let contiguous = deque.make_contiguous();
    println!("contiguous: {:?}", contiguous);
}
