//! Homebrew ABI Loader handling
//!
//! According to the Homebrew ABI, the loader is supposed to pass a
//! configuration structure to the entrypoint, with information such as how to
//! allocate heap, and other core information.
//!
//! This module contains all the loader-related logic. The crt0 should call
//! `init_loader` with the LoaderConfigEntry pointer. Libraries and homebrew can
//! then use the provided getters to access the various configuration options.

// I am a terrible human being. But I *swear* it's safe. The only extern static
// should be LOADER.
#![allow(safe_extern_statics)]

#[cfg(any(feature = "crt0", feature = "std"))]
#[doc(hidden)]
pub mod crt0;

use arrayvec::ArrayVec;
use kernel::sync::InternalMutex;
use core::ptr::Unique;
use core::mem::ManuallyDrop;
use kernel::{Session, KObject};
use bit_field::BitField;
use utils::CursorWrite;
use ipcdefs::twili::IPipe;

#[doc(hidden)]
pub struct LoaderConfig {
    main_thread: u32,
    heap_strategy: InternalMutex<Option<HeapStrategy>>,
    override_services: ArrayVec<[(u64, u32); 32]>,
    stdio_sockets: Option<(u32, u32, u32, SocketKind)>,
    log: Option<InternalMutex<CursorWrite<'static>>>,
    exit: extern fn(u64) -> !,
    twili: Option<IPipe<::kernel::Session>>,
}

pub enum HeapStrategy{
    OverrideHeap(Unique<[u8]>),
    SetHeapSize,
}

pub(crate) fn get_main_thread_handle() -> Option<u32> {
    LOADER.try().map(|x| x.main_thread)
}

/// The allocator should use this to figure out where to get its heap from.
pub fn acquire_heap_strategy() -> Option<HeapStrategy> {
    // Avoid panicking here as much as possible. We're called with locks held.
    match LOADER.try() {
        Some(x) => x.heap_strategy.lock().take(),
        None => None
    }
}

/// Cleanly exits. Used by std.
pub fn exit(u: u64) -> ! {
    // This function should never panic, as it is called by the panic handler.
    match LOADER.try().map(|ldr| ldr.exit) {
        Some(exit) => exit(u),
        None => unsafe { ::kernel::svc::exit_process() }
    }
}

pub fn get_override_service(service_name: [u8; 8]) -> Option<ManuallyDrop<Session>> {
    let loader = match LOADER.try() {
        Some(loader) => loader,
        None => return None
    };
    let service_name : u64 = unsafe { ::core::mem::transmute(service_name) };
    for &(cur_service_name, service) in loader.override_services.iter() {
        if cur_service_name == service_name {
            unsafe {
                return Some(ManuallyDrop::new(Session::from(KObject::new(service))));
            }
        }
    }
    None
}

#[derive(Debug, Clone, Copy)]
pub enum SocketKind {
    BsdU,
    BsdS
}

pub fn get_stdin_socket() -> Option<(SocketKind, u32)> {
    LOADER.try()
        .and_then(|ldr_cfg| (&ldr_cfg.stdio_sockets).as_ref())
        .map(|&(stdin, _, _, kind)| (kind, stdin))
}

pub fn get_stdout_socket() -> Option<(SocketKind, u32)> {
    LOADER.try()
        .and_then(|ldr_cfg| (&ldr_cfg.stdio_sockets).as_ref())
        .map(|&(_, stdout, _, kind)| (kind, stdout))
}

pub fn get_stderr_socket() -> Option<(SocketKind, u32)> {
    LOADER.try()
        .and_then(|ldr_cfg| (&ldr_cfg.stdio_sockets).as_ref())
        .map(|&(_, _, stderr, kind)| (kind, stderr))
}

pub struct Logger;

// TODO: Provide some space in TLS for this
lazy_static! {
    static ref SVC_LOG_SPACE: InternalMutex<ArrayVec<[u8; 4096]>> = InternalMutex::new(ArrayVec::new());
}

impl Logger {
    pub fn write(&self, data: &[u8]) {
        use kernel::svc;

        {
            let mut svc_log_space = SVC_LOG_SPACE.lock();
            let available_capacity = svc_log_space.capacity() - svc_log_space.len();
            if data.len() > available_capacity {
                // Worse-case. Just print it all out and start fresh.
                let _ = unsafe { svc::output_debug_string(svc_log_space.as_ptr(), svc_log_space.len()) };
                let _ = unsafe { svc::output_debug_string(data.as_ptr(), data.len()) };
                let _ = svc_log_space.drain(..);
            } else {
                svc_log_space.extend(data.iter().cloned());
                if let Some(pos) = svc_log_space.iter().cloned().rposition(|i| i == b'\n') {
                    let _ = unsafe { svc::output_debug_string(svc_log_space.as_ptr(), pos) };
                    svc_log_space.drain(..pos + 1);
                }
            }
        }
        if let Some(cursor) = LOADER.try().and_then(|ldr_cfg| (&ldr_cfg.log).as_ref()) {
            cursor.lock().write(data);
        }
        if let Some(pipe) = LOADER.try().and_then(|ldr_cfg| (&ldr_cfg.twili).as_ref()) {
            let _ = pipe.write(data);
        }
    }
}

impl ::core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> Result<(), ::core::fmt::Error> {
        self.write(s.as_bytes());
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
#[cfg(feature = "std")]
#[doc(hidden)]
#[no_mangle]
pub static LOADER: Once<LoaderConfig> = Once::new();

#[cfg(not(feature = "std"))]
extern "Rust" { static LOADER: Once<LoaderConfig>; }

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
    pub const TWILI_PRESENT: u32 = 52;
}

/// ⚠  FOR INTERNAL USE ONLY. YOU SHOULD NOT HAVE TO CALL THIS.
///
/// The pointer should point to a *valid* linked list of LoaderConfigEntry.
pub unsafe fn init_loader(entry: *mut LoaderConfigEntry, exit: extern fn(u64) -> !) -> Result<(), i32> {
    use core::slice;
    use core::marker::PhantomData;

    struct LoaderConfigEntryIterator<'a>(*mut LoaderConfigEntry, PhantomData<&'a ()>);
    impl<'a> Iterator for LoaderConfigEntryIterator<'a> {
        type Item = &'a LoaderConfigEntry;
        fn next(&mut self) -> Option<&'a LoaderConfigEntry> {
            unsafe {
                if self.0.is_null() {
                    None
                } else if (*self.0).tag == LoaderConfigTag::END_OF_LIST {
                    None
                } else {
                    let ptr = self.0;
                    self.0 = self.0.offset(1);
                    ptr.as_ref()
                }
            }
        }
    }

    let mut config = LoaderConfig {
        main_thread: 0,
        heap_strategy: InternalMutex::new(Some(HeapStrategy::SetHeapSize)),
        override_services: ArrayVec::new(),
        log: None,
        stdio_sockets: None,
        exit: exit,
        twili: None,
    };

    for entry in LoaderConfigEntryIterator(entry, PhantomData) {
        match entry.tag {
            LoaderConfigTag::END_OF_LIST => break,
            LoaderConfigTag::LOG => {
                config.log = Some(InternalMutex::new(CursorWrite::new(slice::from_raw_parts_mut((*entry).data.0 as _, (*entry).data.1 as _))));
            },
            LoaderConfigTag::STDIO_SOCKET => {
                let stdin = (*entry).data.0.get_bits(0..32);
                let stdout = (*entry).data.0.get_bits(32..64);
                let stderr = (*entry).data.1.get_bits(0..32);
                let kind = (*entry).data.1.get_bits(32..64);
                let kind = if kind == 0 {
                    SocketKind::BsdU
                } else if kind == 1 {
                    SocketKind::BsdS
                } else {
                    // Skip this value, we don't know it. Maybe error out?
                    continue;
                };
                config.stdio_sockets = Some((stdin as u32, stdout as u32, stderr as u32, kind));
            },
            LoaderConfigTag::OVERRIDE_SERVICE => {
                config.override_services.push(((*entry).data.0, (*entry).data.1 as u32));
            },
            LoaderConfigTag::MAIN_THREAD_HANDLE => config.main_thread = (*entry).data.0 as u32,
            LoaderConfigTag::OVERRIDE_HEAP => {
                *config.heap_strategy.lock() = Some(HeapStrategy::OverrideHeap(Unique::new(slice::from_raw_parts_mut((*entry).data.0 as _, (*entry).data.1 as usize)).unwrap()))
            },
            LoaderConfigTag::APPLET_WORKAROUND => {},
            LoaderConfigTag::TWILI_PRESENT => {
                let twili = match ::ipcdefs::twili::ITwiliService::raw_new() {
                    Ok(twili) => twili,
                    Err(_err) => continue, // TODO: log the error
                };
                let pipe = match twili.open_stdout() {
                    Ok(pipe) => pipe,
                    Err(_err) => continue,
                };
                config.twili = Some(pipe)
            },
            tag => {
                if (*entry).flags & 1 == 1 {
                    // Flag is mandatory! If we don't know about it, we
                    // should commit suicide.
                    return Err(346 | ((100 + tag) << 9) as i32);
                }
            }
        }
    }
    LOADER.call_once(|| config);
    Ok(())
}
