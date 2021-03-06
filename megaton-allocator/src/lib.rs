#![no_std]
#![feature(repr_transparent, const_fn, allocator_api, ptr_internals, alloc, align_offset)]

extern crate alloc;
extern crate megaton_hammer;
extern crate spin;
extern crate linked_list_allocator;

use core::alloc::{GlobalAlloc, Layout, AllocErr};
use megaton_hammer::loader::{self, HeapStrategy};
use megaton_hammer::kernel::sync::{InternalMutex as Mutex, InternalMutexGuard as MutexGuard};
use spin::Once;
use core::ops::Deref;
use core::ptr::NonNull;
use linked_list_allocator::{Heap, align_up};
use megaton_hammer::error::*;

pub struct Allocator(Mutex<Heap>, Once<HeapStrategy>);

impl Allocator {
    /// safely expands the heap if possible.
    fn expand(&self, by: usize, heap: &mut MutexGuard<Heap>) -> Result<()> {
        // TODO: does we really need to only acquire this once?
        // TODO: do we want to do something even if acquire_heap_strategy fails, such as default to SetHeapSize?
        let strategy = self.1.call_once(|| loader::acquire_heap_strategy().unwrap());
        match strategy {
            HeapStrategy::OverrideHeap(ptr) => if heap.bottom() == 0 {
                unsafe { heap.init(ptr.as_ptr() as *mut u8 as usize, ptr.as_ref().len()) };
            } else {
                // TODO: Should this panic instead of do nothing?
            },
            HeapStrategy::SetHeapSize => {
                let total = heap.size() + align_up(by, 0x200_000); // set_heap_size requires this allignment.

                let ptr = unsafe { megaton_hammer::kernel::svc::set_heap_size(total as u32)? };

                if heap.bottom() == 0 {
                    unsafe { **heap = Heap::new(ptr as *mut u8 as usize, total) };
                } else {
                    unsafe { heap.extend(by) };
                }
            }
        }
        Ok(())
    }

    /// Creates a new heap based off of loader settings.
    pub const fn new() -> Allocator {
        Allocator(Mutex::new(Heap::empty()), Once::new())
    }
}

impl Deref for Allocator {
    type Target = Mutex<Heap>;

    fn deref(&self) -> &Mutex<Heap> {
        &self.0
    }
}

unsafe impl<'a> GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut heap = self.0.lock();
        let allocation = heap.allocate_first_fit(layout);
        // If the heap is exhausted, then extend and attempt the allocation another time.
        match allocation {
            Err(AllocErr) => {
                if let Ok(_) = self.expand(layout.size(), &mut heap) {
                    let alloc = heap.allocate_first_fit(layout);
                    alloc
                } else {
                    // Return the original failed allocation of we can't expand.
                    allocation
                }
            }
            _ => allocation
        }.ok().map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().deallocate(NonNull::new(ptr).unwrap(), layout)
    }
}
