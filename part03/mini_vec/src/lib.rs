use std::{
    alloc::{self, Layout},
    ptr::NonNull,
};

/// 미니 Vec 구현
/// 교육 목적으로만 사용
/// 프로덕션에서는 [Vec]을 사용할 것
pub struct MiniVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> MiniVec<T> {
    pub fn new() -> Self {
        Self {
            // 비어 있을 때는 실제 메모리를 할당하지 않는다.
            // NonNull::dangling()은 null이 아니지만 역참초하면 UB인 포인터
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// 용량을 grow한다.
    /// [Self::cap]이 0이면 4로 시작, 이후 두 배씩
    fn grow(&mut self) {
        let new_cap = if self.is_empty() { 4 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).expect("레이아웃 계산 실패");
        let new_ptr = if self.is_empty() {
            // 처음 할당
            unsafe { alloc::alloc(new_layout) }
        } else {
            // 재할당
            let old_layout = Layout::array::<T>(self.cap).expect("레이아웃 계산 실패");
            unsafe { alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) }
        };

        // 할당 실패 시 패닉
        self.ptr = NonNull::new(new_ptr as *mut T)
            .unwrap_or_else(|| alloc::handle_alloc_error(new_layout));
        self.cap = new_cap;
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            // ptr.add(len) 위치에 value를 쓴다.
            std::ptr::write(self.ptr.as_ptr().add(self.len), value);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            // len 위치의 값을 읽고 소유권을 이전한다.
            unsafe { Some(std::ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len {
            None
        } else {
            unsafe { Some(self.ptr.as_ptr().add(idx).as_ref_unchecked()) }
        }
    }
}

impl<T> Drop for MiniVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // 남아 있는 원소를 drop
            while self.pop().is_some() {}

            // 메모리 해제
            let layout = Layout::array::<T>(self.cap).expect("레이아웃 계산 실패");
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> Default for MiniVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
