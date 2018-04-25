use alloc::allocator::{Alloc, Layout, AllocErr};
use core;
use loader::{self, HeapStrategy};
use byteorder::{LE, ByteOrder};
use bit_field::BitField;
use spin::Once;
use core::fmt::{Debug, Formatter, Error};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
struct BlockHdr(u64);

impl BlockHdr {
    pub fn get_size(&self) -> u64 {
        self.0.get_bits(0..40)
    }
    pub fn set_size(&mut self, size: u64) {
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

struct Block(&'static mut BlockHdr, &'static mut BlockHdr);

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Block({:x} {:?}, {:x} {:?})", self.0 as *const BlockHdr as usize, self.0, self.1 as *const BlockHdr as usize, self.1)
    }
}

impl Block {
    pub fn get_size(&self) -> u64 {
        self.0.get_size()
    }
    pub fn set_size(&mut self, size: u64) {
        self.0.set_size(size);
        self.1.set_size(size);
    }

    pub fn is_end(&self) -> bool {
        self.0.is_end()
    }
    pub fn set_end(&mut self, end: bool) {
        self.0.set_end(end);
        self.1.set_end(end);
    }

    pub fn is_free(&self) -> bool {
        self.0.is_free()
    }
    pub fn set_free(&mut self, free: bool) {
        self.0.set_free(free);
        self.1.set_free(free);
    }

    pub fn split(self, size: usize) -> (Self, Self) {
        //writeln!(&mut ::loader::Logger, "Splitting block {:?} to size {}", self, size);
        if !self.is_free() || self.get_size() < size as u64 + 16 {
            panic!("WTF");
        }
        //use core::fmt::Write;
        let cursize = self.get_size();
        unsafe {
            let newend = (self.0 as *mut BlockHdr as *mut u8).offset(8 + size as isize) as *mut BlockHdr;
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
        unsafe { (self.0 as *const BlockHdr as *mut BlockHdr).offset(1) as *mut u8 }
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
        let block_end = unsafe { (block_start as *mut u8).offset(8 + (*block_start).get_size() as isize) as *mut BlockHdr };
        let block = unsafe { Block(&mut *block_start, &mut *block_end) };
        unsafe {
            if *block_start != *block_end {
                use core::fmt::Write;
                writeln!(&mut ::loader::Logger, "WHAT THE FUCK: {:?}", block);
                ::loader::exit(0);
            }
        }
        if block.is_end() {
            self.0 = core::ptr::null_mut();
        } else {
            self.0 = unsafe { block_end.offset(1) };
        }
        Some(block)
    }
}

/// A very simple allocator. It's not very smart or efficient, but it tries its
/// best.
pub struct Allocator { base: Once<usize>, strategy: Once<HeapStrategy> }

unsafe impl<'a> Alloc for &'a Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        //use core::fmt::Write;
        //writeln!(&mut ::loader::Logger, "Allocating {:?}", layout);
        let strategy = self.strategy.call_once(|| loader::acquire_heap_strategy().unwrap());

        match strategy {
            &HeapStrategy::OverrideHeap(mut ptr) => {
                let ptr = (*self.base.call_once(|| {
                    //writeln!(&mut ::loader::Logger, "Initializing");
                    let mut initial_hdr = BlockHdr(0);
                    initial_hdr.set_end(true);
                    initial_hdr.set_free(true);
                    let ptr_len = ptr.as_ref().len();
                    initial_hdr.set_size(ptr_len as u64 - 16);
                    LE::write_u64(&mut ptr.as_mut()[..8], initial_hdr.0);
                    LE::write_u64(&mut ptr.as_mut()[ptr_len - 8..], initial_hdr.0);
                    ptr.as_ptr() as *mut u8 as usize
                })) as *mut BlockHdr;
                let mut found = Err(AllocErr::Exhausted { request: layout.clone() });
                for block in BlockIter::new(ptr) {
                    //writeln!(&mut ::loader::Logger, "Got block {:?}", block);
                    if found.is_err() && block.is_free() {
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
                            found = Ok(block.get_content_ptr());
                        }
                    }
                }
                found
                //Err(AllocErr::Exhausted { request: layout })
            },
            &HeapStrategy::SetHeapSize => {
                Err(AllocErr::Unsupported { details: "We don't support setheapsize yet" })
            }
        }
    }
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        use core::fmt::Write;
        //writeln!(&mut ::loader::Logger, "Deallocating {:p} {:?}", ptr, layout);
        // TODO: Handle big alignments
        let strategy = self.strategy.call_once(|| loader::acquire_heap_strategy().unwrap());

        let base = match strategy {
            &HeapStrategy::OverrideHeap(mut ptr) => {
                (*self.base.call_once(|| {
                    let mut initial_hdr = BlockHdr(0);
                    initial_hdr.set_end(true);
                    initial_hdr.set_free(true);
                    let ptr_len = ptr.as_ref().len();
                    initial_hdr.set_size(ptr_len as u64 - 16);
                    LE::write_u64(&mut ptr.as_mut()[..8], initial_hdr.0);
                    LE::write_u64(&mut ptr.as_mut()[ptr_len - 8..], initial_hdr.0);
                    ptr.as_ptr() as *mut u8 as usize
                })) as *mut BlockHdr
            },
            &HeapStrategy::SetHeapSize => unimplemented!()
        };

        let mut start : *mut BlockHdr = ptr.offset(-8) as *mut BlockHdr;
        let mut end : *mut BlockHdr = ptr.offset((*start).get_size() as isize) as *mut BlockHdr;

        if start > base {
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
        Allocator { base: Once::new(), strategy: Once::new() }
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
