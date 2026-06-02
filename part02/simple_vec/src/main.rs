use std::{
    alloc::{self, Layout},
    ptr,
};

struct SimpleVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> SimpleVec<T> {
    fn new() -> Self {
        SimpleVec {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(&*self.ptr.add(index)) }
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let layout = Layout::array::<T>(new_capacity).unwrap();

        let new_ptr = unsafe { alloc::alloc(layout) as *mut T };
        if new_ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        if self.capacity > 0 {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                alloc::dealloc(self.ptr as *mut u8, old_layout);
            }
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

impl<T> Drop for SimpleVec<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.add(i));
            }
            let layout = Layout::array::<T>(self.capacity).unwrap();
            alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

fn main() {
    let mut v = SimpleVec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    for i in 0..3 {
        if let Some(val) = v.get(i) {
            println!("[{}] = {}", i, val);
        }
    }
}
