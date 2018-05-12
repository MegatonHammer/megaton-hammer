#![no_std]
#![feature(repr_transparent, const_fn, allocator_api, ptr_internals, alloc, align_offset)]

extern crate alloc;
extern crate megaton_hammer;
extern crate bit_field;
extern crate spin;
extern crate linked_list_allocator;

use alloc::allocator::{Alloc, Layout, AllocErr};
use megaton_hammer::loader::{self, HeapStrategy};
use bit_field::BitField;
use spin::{Once, Mutex};
use core::fmt::{Debug, Formatter, Error};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::ops::Deref;
use linked_list_allocator::{Heap, align_up};

pub struct LockedMegatonHeap(Mutex<Heap>, HeapStrategy);

impl LockedMegatonHeap {
    /// Creates an empty heap. All allocate calls will return `None`.
    // pub const fn empty() -> LockedMegatonHeap {
    //     LockedMegatonHeap(Mutex::new(Heap::empty()))
    // }

    fn expand(&self, by: usize) {
        match self.1 {
            HeapStrategy::OverrideHeap(_) => return, // TODO: what should this do, other than nothing? panic? throw oom?
            HeapStrategy::SetHeapSize => {
                let by = align_up(by, 0x200_000); // set_heap_size requires this allignment.
                let mut heap = self.0.lock();

                let (ret, ptr) = unsafe { megaton_hammer::kernel::svc::set_heap_size((heap.size() + by) as u32) };
                if ret != 0 {
                    panic!("Failed to allocate 2MB: {}", ret);
                }

                unsafe { heap.extend(by) };
            }
        }
    }

    /// Creates a new heap based off of loader settings.
    pub fn new() -> LockedMegatonHeap {
        let strategy = loader::acquire_heap_strategy().unwrap();
        let (heap_bottom, heap_size) = match strategy {
            HeapStrategy::OverrideHeap(mut ptr) => {
                (ptr.as_ptr() as *mut u8 as usize, unsafe { ptr.as_ref() }.len())
            },
            HeapStrategy::SetHeapSize => {
                // Allocate the first block.
                let (ret, ptr) = unsafe { megaton_hammer::kernel::svc::set_heap_size(0x200_000) };
                if ret != 0 {
                    panic!("Failed to allocate 2MB: {}", ret);
                }
                (ptr as usize, 0x200_000)
            }
        };
        LockedMegatonHeap(Mutex::new(unsafe { Heap::new(heap_bottom, heap_size) }), strategy)
    }
}

impl Deref for LockedMegatonHeap {
    type Target = Mutex<Heap>;

    fn deref(&self) -> &Mutex<Heap> {
        &self.0
    }
}

// TODO: This'll need updating for GlobalAlloc
unsafe impl<'a> Alloc for &'a LockedMegatonHeap {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let addr = self.0.lock().allocate_first_fit(layout);
        if let Err(AllocErr::Exhausted {request: layout}) = addr {
            self.expand(layout.size()); // TODO: how much should I extend by?
            self.0.lock().allocate_first_fit(layout)
        } else {
            addr
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.0.lock().deallocate(ptr, layout)
    }
}
