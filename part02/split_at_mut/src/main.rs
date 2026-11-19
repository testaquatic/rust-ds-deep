fn split_at_mut_safe<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    assert!(mid <= len, "{}는 {}보다 작저가 같아야 함", mid, len);

    let ptr = slice.as_mut_ptr();

    // 안전한 이유
    // mid <= len 이므로 두 슬라이스를 겹치지 않음
    // ptr..ptr+mid와 ptr+mid..ptrZ
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let (left, right) = split_at_mut_safe(&mut v, 3);
    left[0] = 10;
    right[0] = 40;
    // [10, 2, 3, 40, 5]
    println!("{:?}", v);
    assert_eq!(v, vec![10, 2, 3, 40, 5]);
}
