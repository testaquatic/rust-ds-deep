use std::{alloc::Layout, ptr};

pub struct SimpleVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> SimpleVec<T> {
    pub fn new() -> Self {
        Self {
            // null 대신 dangling 포인터
            ptr: ptr::dangling_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }

        // 안전한 이유
        // Self::grow()후 self.ptr은 self.capacity 크기의 유효한 힙 메모리를 가르킴
        // self.len < self.capacity이므로 ptr.add(self.len)은 범위 안
        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index > self.len {
            return None;
        }

        // 안전한 이유: index < self.len이므로 유효한 범위 안
        unsafe { self.ptr.add(index).as_ref() }
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let layout = Layout::array::<T>(new_capacity).unwrap();

        // 안전한 이유
        // layout은 유효하고, alloc이 실패하면 handle_alloc_error가 처리
        let new_ptr = unsafe { std::alloc::alloc(layout) as *mut T };
        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(layout)
        }

        if self.capacity > 0 {
            // 안전한 이유
            // 기존 ptr은 self.capcity 크기의 유효한 메모리를 가리킴
            unsafe {
                std::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.ptr as *mut u8, old_layout);
            }
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

impl<T> Drop for SimpleVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            // 저장된 요소를 drop
            unsafe {
                for i in 0..self.len {
                    ptr::drop_in_place(self.ptr.add(i));
                }
                let layout = Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

impl<T> Default for SimpleVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
