use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::mem;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

struct FreeListNode {
    size: usize,
    next: *mut FreeListNode,
}

pub struct FreeListAllocator {
    head: UnsafeCell<*mut FreeListNode>, 
    heap_start: AtomicUsize,
    heap_end: AtomicUsize,
}

unsafe impl Send for FreeListAllocator {}
unsafe impl Sync for FreeListAllocator {}

impl FreeListAllocator {
    pub const fn new() -> Self {
        FreeListAllocator {
            head: UnsafeCell::new(null_mut()),
            heap_start: AtomicUsize::new(0),
            heap_end: AtomicUsize::new(0),
        }
    }

    pub unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        self.heap_start.store(heap_start, Ordering::Relaxed) ;
        self.heap_end.store(heap_start + heap_size, Ordering::Relaxed) ;

        let first_node = heap_start as *mut FreeListNode;
        first_node.write(FreeListNode { size: heap_size, next: null_mut() });

        *self.head.get() = first_node;
    }

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut prev: *mut FreeListNode = null_mut();
        let mut current = *self.head.get();

        while !current.is_null() {
            let node = &mut *current;

            let alloc_start = (current as usize + mem::size_of::<FreeListNode>() + layout.align() - 1)
                & !(layout.align() - 1);
            let alloc_end = alloc_start + layout.size();

            if alloc_end > self.heap_end.load(Ordering::Relaxed) || alloc_end > (current as usize + node.size) {
                prev = current;
                current = node.next;
                continue;
            }

            let remaining_size = (current as usize + node.size) - alloc_end;
            if remaining_size > mem::size_of::<FreeListNode>() {
                let new_node = alloc_end as *mut FreeListNode;
                new_node.write(FreeListNode {
                    size: remaining_size,
                    next: node.next,
                });

                if prev.is_null() {
                    *self.head.get() = new_node;
                } else {
                    (*prev).next = new_node;
                }
            } else {
                if prev.is_null() {
                    *self.head.get() = node.next;
                } else {
                    (*prev).next = node.next;
                }
            }

            return alloc_start as *mut u8;
        }

        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let free_node = ptr as *mut FreeListNode;
        free_node.write(FreeListNode {
            size: layout.size(),
            next: *self.head.get(),
        });

        *self.head.get() = free_node;
    }
}

#[global_allocator]
static ALLOCATOR: FreeListAllocator = FreeListAllocator::new();

unsafe impl GlobalAlloc for FreeListAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.dealloc(ptr, layout)
    }
}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

#[no_mangle]
pub extern "C" fn memset(dest: *mut u8, value: i32, n: usize) -> *mut u8 {
    let byte = value as u8;
    for i in 0..n {
        unsafe {
            *dest.add(i) = byte;
        }
    }
    dest
}

pub fn init_heap(heap_start: usize, heap_size: usize) {
    unsafe { 
        ALLOCATOR.init(heap_start, heap_size); 
    }
}
