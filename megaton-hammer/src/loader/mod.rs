//! Homebrew ABI Loader handling
//!
//! According to the Homebrew ABI, the loader is supposed to pass a
//! configuration structure to the entrypoint, with information such as how to
//! allocate heap, and other core information.
//!
//! This module contains all the loader-related logic. The crt0 should call
//! `init_loader` with the LoaderConfigEntry pointer. Libraries and homebrew can
//! then use the provided getters to access the various configuration options.

#[doc(hidden)]
pub mod crt0;

extern crate core_io as io;

use arrayvec::ArrayVec;
use spin::Mutex;
use core::ptr::Unique;

struct LoaderConfig {
    main_thread: u32,
    override_heap: Mutex<Option<HeapStrategy>>,
    override_services: ArrayVec<[(&'static str, u32); 32]>,
    log: Option<Mutex<io::Cursor<&'static mut [u8]>>>
}

pub enum HeapStrategy{
    OverrideHeap(Unique<[u8]>),
    SetHeapSize,
}

/// The allocator should use this to figure out where to get its heap from.
pub fn acquire_heap_strategy() -> Option<HeapStrategy> {
    // Avoid panicking here as much as possible. We're called with locks held.
    match LOADER.try() {
        Some(x) => x.override_heap.lock().take(),
        None => None
    }
}

pub struct Logger;

impl ::core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> Result<(), ::core::fmt::Error> {
        use kernel::svc;
        use loader::io::Write;

        if let Some(cursor) = LOADER.try().and_then(|ldr_cfg| (&ldr_cfg.log).as_ref()) {
            cursor.lock().write(s.as_bytes());
        } else {
            unsafe { svc::output_debug_string(s.as_ptr(), s.len()) };
        }
        Ok(())
    }
}

// TODO: Reenable logger.
// This should probably write in as many places as possible (bsd log if
// there is one, static LOG if there is one, SvcLog)
/*pub fn get_logger_locked() -> &'static mut ::core::fmt::Write {
    
}*/

use spin::Once;

// TODO: For fuck's sake I just want interior mutability.
static LOADER: Once<LoaderConfig> = Once::new();

#[repr(C)]
#[doc(hidden)]
pub struct LoaderConfigEntry {
    tag: u32,
    flags: u32,
    data: (u64, u64)
}

struct LoaderConfigTag;

#[allow(dead_code)]
impl LoaderConfigTag {
    pub const END_OF_LIST: u32 = 0;
    pub const MAIN_THREAD_HANDLE: u32 = 1;
    pub const NEXT_LOAD_PATH: u32 = 2;
    pub const OVERRIDE_HEAP: u32 = 3;
    pub const OVERRIDE_SERVICE: u32 = 4;
    pub const ARGV: u32 = 5;
    pub const SYSCALL_AVAILABLE_HINT: u32 = 6;
    pub const APPLET_TYPE: u32 = 7;
    pub const APPLET_WORKAROUND: u32 = 8;
    pub const STDIO_SOCKET: u32 = 9;
    pub const PROCESS_HANDLE: u32 = 10;
    pub const LAST_LOAD_RESULT: u32 = 11;
    pub const ALLOC_PAGES: u32 = 12;
    pub const LOCK_REGION: u32 = 13;
    pub const LOG: u32 = 51;
}

/// ⚠  FOR INTERNAL USE ONLY. YOU SHOULD NOT HAVE TO CALL THIS.
///
/// The pointer should point to a *valid* linked list of LoaderConfigEntry.
pub unsafe fn init_loader(mut entry: *mut LoaderConfigEntry) -> Result<(), i32> {
    use core::slice;

    let mut config = LoaderConfig {
        main_thread: 0,
        override_heap: Mutex::new(Some(HeapStrategy::SetHeapSize)),
        override_services: ArrayVec::new(),
        log: None
    };

    if !entry.is_null() {
        loop {
            match (*entry).tag {
                LoaderConfigTag::END_OF_LIST => break,
                LoaderConfigTag::LOG => {
                    config.log = Some(Mutex::new(io::Cursor::new(slice::from_raw_parts_mut((*entry).data.0 as _, (*entry).data.1 as _))));
                },
                LoaderConfigTag::MAIN_THREAD_HANDLE => config.main_thread = (*entry).data.0 as u32,
                LoaderConfigTag::OVERRIDE_HEAP =>
                    *config.override_heap.lock() = Some(HeapStrategy::OverrideHeap(Unique::new(slice::from_raw_parts_mut((*entry).data.0 as _, (*entry).data.1 as usize)).unwrap())),
                LoaderConfigTag::APPLET_WORKAROUND => {},
                tag => {
                    if (*entry).flags & 1 == 1 {
                        // Flag is mandatory! If we don't know about it, we
                        // should commit suicide.
                        return Err(346 | ((100 + tag) << 9) as i32);
                    }
                }
            }
            entry = entry.offset(1);
        }
    } else {
        // TODO: What to do about the main thread then ?
    }
    LOADER.call_once(|| config);
    Ok(())
}
