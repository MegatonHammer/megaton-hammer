#![no_std]
#![feature(repr_transparent, const_fn, allocator_api, ptr_internals, alloc, align_offset)]

extern crate alloc;
extern crate megaton_hammer;
extern crate bit_field;
extern crate spin;

use alloc::allocator::{Alloc, Layout, AllocErr};
use megaton_hammer::loader::{self, HeapStrategy};
use bit_field::BitField;
use spin::Once;
use core::fmt::{Debug, Formatter, Error};
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
struct BlockHdr(u64);

impl BlockHdr {
    pub fn get_size(&self) -> u64 {
        self.0.get_bits(0..40)
    }
    pub fn set_size(&mut self, size: u64) {
        // TODO: Verify size is 8-byte aligned.
        self.0.set_bits(0..40, size);
    }

    pub fn is_end(&self) -> bool {
        self.0.get_bit(62)
    }
    pub fn set_end(&mut self, end: bool) {
        self.0.set_bit(62, end);
    }

    pub fn is_free(&self) -> bool {
        self.0.get_bit(63)
    }
    pub fn set_free(&mut self, free: bool) {
        self.0.set_bit(63, free);
    }
}

impl Debug for BlockHdr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "BlockHdr {{ size: {}, is_end: {}, is_free: {} }}", self.get_size(), self.is_end(), self.is_free())
    }
}

struct Block(*mut BlockHdr, *mut BlockHdr);

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Block({:x} {:?}, {:x} {:?})", self.0 as *const BlockHdr as usize, self.0, self.1 as *const BlockHdr as usize, self.1)
    }
}

impl Block {
    pub unsafe fn from_start(start: *mut BlockHdr) -> Block {
        Block(start, (start as usize + 8 + (*start).get_size() as usize) as *mut BlockHdr)
    }
    pub unsafe fn from_end(end: *mut BlockHdr) -> Block {
        Block((end as usize - (*end).get_size() as usize - 8) as *mut BlockHdr, end)
    }
    pub fn get_size(&self) -> u64 {
        unsafe { (*self.0).get_size() }
    }
    pub fn set_size(&mut self, size: u64) {
        unsafe {
            (*self.0).set_size(size);
            (*self.1).set_size(size);
        }
    }

    pub fn is_end(&self) -> bool {
        unsafe {
            (*self.0).is_end()
        }
    }
    pub fn set_end(&mut self, end: bool) {
        unsafe {
            (*self.0).set_end(end);
            (*self.1).set_end(end);
        }
    }

    pub fn is_free(&self) -> bool {
        unsafe {
            (*self.0).is_free()
        }
    }
    pub fn set_free(&mut self, free: bool) {
        unsafe {
            (*self.0).set_free(free);
            (*self.1).set_free(free);
        }
    }

    pub fn split(self, size: usize) -> (Self, Self) {
        //use core::fmt::Write;
        //writeln!(&mut ::loader::Logger, "Splitting block {:?} to size {}", self, size);

        if !self.is_free() || self.get_size() < size as u64 + 16 {
            panic!("WTF");
        }
        let cursize = self.get_size();
        unsafe {
            let newend = (self.0 as *mut u8).offset(8 + size as isize) as *mut BlockHdr;
            let newstart = newend.offset(1);
            *newend = *self.0;
            *newstart = *self.1;

            let mut block_start = Block(self.0, &mut *newend);
            block_start.set_size(size as u64);
            block_start.set_end(false);

            let mut block_end = Block(&mut *newstart, self.1);
            block_end.set_size(cursize - (size as u64) - 16);
            //writeln!(&mut ::loader::Logger, "New blocks are {:?} and {:?}", block_start, block_end);
            (block_start, block_end)
        }
    }

    pub fn get_content_ptr(&self) -> *mut u8 {
        unsafe { self.0.offset(1) as *mut u8 }
    }
}

struct BlockIter(*mut BlockHdr);

impl BlockIter {
    pub fn new(ptr: *mut BlockHdr) -> BlockIter {
        BlockIter(ptr)
    }
}

impl Iterator for BlockIter {
    type Item = Block;
    fn next(&mut self) -> Option<Block> {
        if self.0.is_null() {
            return None;
        }

        let block_start = self.0;
        let block = unsafe {
            let block = Block::from_start(self.0);
            if *block.0 != *block.1 {
                use core::fmt::Write;
                writeln!(&mut ::loader::Logger, "WHAT THE FUCK: {:?}", block);
                ::loader::exit(0);
            }
            block
        };
        if block.is_end() {
            self.0 = core::ptr::null_mut();
        } else {
            self.0 = unsafe { block.1.offset(1) };
        }
        Some(block)
    }
}

/// A very simple allocator. It's not very smart or efficient, but it tries its
/// best.
pub struct Allocator { base: Once<usize>, size: AtomicUsize, strategy: Once<HeapStrategy> }

impl Allocator {
    fn get_base(&self) -> usize {
        let strategy = self.strategy.call_once(|| loader::acquire_heap_strategy().unwrap());

        *self.base.call_once(|| {
            let ptr = match strategy {
                &HeapStrategy::OverrideHeap(mut ptr) => {
                    unsafe { self.size.store(ptr.as_ref().len(), Ordering::SeqCst); }
                    ptr.as_ptr() as *mut u8 as usize
                },
                &HeapStrategy::SetHeapSize => {
                    // Allocate the first block.
                    //TODO: Locking
                    let (ret, ptr) = unsafe { megaton_hammer::kernel::svc::set_heap_size(0x200_000) };
                    if ret != 0 {
                        panic!("Failed to allocate 2MB: {}", ret);
                    }
                    self.size.store(0x200_000, Ordering::SeqCst);
                    ptr as usize
                }
            };

            // Initialize first block.
            //writeln!(&mut ::loader::Logger, "Initializing");
            let mut initial_hdr = BlockHdr(0);
            initial_hdr.set_end(true);
            initial_hdr.set_free(true);
            initial_hdr.set_size(self.size.load(Ordering::SeqCst) as u64 - 16);
            unsafe {
                *(ptr as *mut u64) = initial_hdr.0;
                *((ptr + self.size.load(Ordering::SeqCst) - 8) as *mut u64) = initial_hdr.0;
            }
            ptr
        })
    }
}

unsafe impl<'a> Alloc for &'a Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        use core::fmt::Write;
        writeln!(&mut ::loader::Logger, "Allocating {:?}", layout);
        let strategy = self.strategy.call_once(|| loader::acquire_heap_strategy().unwrap());

        let base = self.get_base();

        for block in BlockIter::new(base as *mut BlockHdr) {
            writeln!(&mut ::loader::Logger, "Got block {:?}", block);
            if block.is_free() {
                let mut block = block;

                if block.get_content_ptr().align_offset(layout.align()) != 0 {
                    // Align it first.
                    let offset = block.get_content_ptr().align_offset(layout.align());
                    if offset + 16 + layout.size() < block.get_size() as usize {
                        let (block_left, block_right) = block.split(offset - 16);
                        block = block_right;
                    } else {
                        continue;
                    }
                }
                if layout.size() + layout.padding_needed_for(8) + 16 < block.get_size() as usize {
                    // Minimum alignment is 8. Yes, really.
                    let (new_block, newblock) = block.split(layout.size() + layout.padding_needed_for(8));
                    block = new_block;
                }

                if layout.size() + layout.padding_needed_for(8) <= block.get_size() as usize {
                    block.set_free(false);
                    //writeln!(&mut ::loader::Logger, "Returning {:p}", block.get_content_ptr());
                    return Ok(block.get_content_ptr());
                }
            }
        }

        // No block found. Extend last block.
        match strategy {
            &HeapStrategy::OverrideHeap(_) => Err(AllocErr::Exhausted { request: layout }),
            &HeapStrategy::SetHeapSize => {
                writeln!(&mut ::loader::Logger, "Expanding heap");
                // Try to allocate more memory. First: figure out how much we need.
                let mut last_block = Block::from_end((base + self.size.load(Ordering::SeqCst) - 8) as *mut BlockHdr);
                let size = last_block.get_size();
                let additional_size = (layout.size() + layout.padding_needed_for(8) + if last_block.is_free() { 0 } else { 16 }) - last_block.get_size() as usize;
                // Align it to the next upper 2MB
                let additional_size = if additional_size & (0x200000 - 1) == 0 { additional_size } else { (additional_size + 0x200000) & !(0x200000 - 1) };
                let new_size = self.size.load(Ordering::SeqCst) + additional_size;


                // Allocate moar.
                let (res, new_addr) = megaton_hammer::kernel::svc::set_heap_size(new_size as u32);
                self.size.store(new_size, Ordering::SeqCst);

                if last_block.is_free() {
                    // Extend block.
                    let new_last_block_end = (base + new_size - 8) as *mut BlockHdr;
                    *new_last_block_end = *last_block.1;
                    last_block = Block(last_block.0, new_last_block_end.as_mut().unwrap());
                    last_block.set_size(additional_size as u64);
                } else {
                    // Create new block
                    last_block.set_end(false);
                    let new_block_start = last_block.1.offset(1);
                    let new_block_end = (base + new_size) as *mut BlockHdr;
                    let mut new_block = Block(new_block_start, new_block_end);
                    new_block.set_end(true);
                    new_block.set_free(true);
                    new_block.set_size(additional_size as u64 - 16);
                    last_block = new_block;
                }

                // Split if necessary.
                if layout.size() + layout.padding_needed_for(8) + 16 < last_block.get_size() as usize {
                    // Minimum alignment is 8. Yes, really.
                    let (new_last_block, newlast_block) = last_block.split(layout.size() + layout.padding_needed_for(8));
                    last_block = new_last_block;
                }

                last_block.set_free(false);
                //writeln!(&mut ::loader::Logger, "Returning {:p}", last_block.get_content_ptr());
                return Ok(last_block.get_content_ptr());
            }
        }
    }
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        //use core::fmt::Write;
        //writeln!(&mut ::loader::Logger, "Deallocating {:p} {:?}", ptr, layout);
        // TODO: Handle big alignments
        let strategy = self.strategy.call_once(|| loader::acquire_heap_strategy().unwrap());

        let base = self.get_base();

        let mut start : *mut BlockHdr = ptr.offset(-8) as *mut BlockHdr;
        let mut end : *mut BlockHdr = ptr.offset((*start).get_size() as isize) as *mut BlockHdr;

        if start as usize > base {
            let previous_end : *mut BlockHdr = start.offset(-1) as *mut BlockHdr;
            if (*previous_end).is_free() {
                let previous_start = (previous_end as *mut u8).offset(- (((*previous_end).get_size() + 8) as isize)) as *mut BlockHdr;
                (*previous_start).set_size((*previous_start).get_size() + 16 + (*start).get_size());
                (*previous_start).set_end((*start).is_end());
                start = previous_start;
                (*end).set_size((*start).get_size());
            }
        }

        if !(*start).is_end() {
            let next_start : *mut BlockHdr = end.offset(1) as *mut BlockHdr;
            if (*next_start).is_free() {
                let next_end = (next_start as *mut u8).offset(8 + (*next_start).get_size() as isize) as *mut BlockHdr;
                (*next_end).set_size((*start).get_size() + 16 + (*next_end).get_size());
                end = next_end;
                (*start).set_size((*end).get_size());
                (*start).set_end((*end).is_end());
            }
        }

        (*start).set_free(true);
        (*end).set_free(true);
    }
}

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator { base: Once::new(), size: AtomicUsize::new(0), strategy: Once::new() }
    }

    pub fn print_allocs(&self, f: &mut core::fmt::Write) {
        
    }
}

unsafe impl Alloc for Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        (&*self).alloc(layout)
    }
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        (&*self).dealloc(ptr, layout)
    }
}
