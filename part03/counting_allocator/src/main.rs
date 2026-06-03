use std::{
    alloc::{GlobalAlloc, System},
    sync::atomic::{AtomicUsize, Ordering},
};

struct CountingAllocator;

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
static DEALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        DEALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static A: CountingAllocator = CountingAllocator;

fn main() {
    let before_alloc = ALLOC_COUNT.load(Ordering::Relaxed);

    let mut v1 = Vec::new();
    for i in 0..100 {
        v1.push(i);
    }
    let after_v1 = ALLOC_COUNT.load(Ordering::Relaxed);

    let before_v2 = ALLOC_COUNT.load(Ordering::Relaxed);

    let mut v2 = Vec::with_capacity(100);
    for i in 0..100 {
        v2.push(i);
    }
    let after_v2 = ALLOC_COUNT.load(Ordering::Relaxed);

    println!("before alloc  : {}", before_alloc);
    println!("after v1      : {}", after_v1);
    println!("before v2     : {}", before_v2);
    println!("after v2      : {}", after_v2);
}
