#[cfg(feature = "crt0")]
#[doc(hidden)]
pub mod crt0;

use arrayvec::ArrayVec;

struct LoaderConfig {
    main_thread: u32,
    override_heap: Option<(*mut (), usize)>,
    override_services: ArrayVec<[(&'static str, u32); 32]>,
}

// TODO: Fuck.
unsafe impl Send for LoaderConfig {}
unsafe impl Sync for LoaderConfig {}


pub enum HeapStrategy{
    OverrideHeap(*mut (), usize),
    SetHeapSize,
}

pub fn get_heap_method() -> HeapStrategy {
    match LOADER.try().and_then(|ldr_cfg| ldr_cfg.override_heap) {
        Some((addr, size)) => HeapStrategy::OverrideHeap(addr, size),
        None => HeapStrategy::SetHeapSize
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
pub struct LoaderConfigEntry {
    tag: u32,
    flags: u32,
    data: (u64, u64)
}
mod LoaderConfigTag {
    pub const EndOfList: u32 = 0;
    pub const MainThreadHandle: u32 = 1;
    pub const NextLoadPath: u32 = 2;
    pub const OverrideHeap: u32 = 3;
    pub const OverrideService: u32 = 4;
    pub const Argv: u32 = 5;
    pub const SyscallAvailableHint: u32 = 6;
    pub const AppletType: u32 = 7;
    pub const AppletWorkaround: u32 = 8;
    pub const StdioSockets: u32 = 9;
    pub const ProcessHandle: u32 = 10;
    pub const LastLoadResult: u32 = 11;
    pub const AllocPages: u32 = 12;
    pub const LockRegion: u32 = 13;
    pub const Log: u32 = 51;
}

/// ⚠  FOR INTERNAL USE ONLY. YOU SHOULD NOT HAVE TO CALL THIS.
///
/// The pointer should point to a *valid* linked list of LoaderConfigEntry.
pub unsafe fn init_loader(mut entry: *mut LoaderConfigEntry) -> Result<(), i32> {
    let mut config = LoaderConfig {
        main_thread: 0,
        override_heap: None,
        override_services: ArrayVec::new()
    };

    if !entry.is_null() {
        loop {
            match (*entry).tag {
                LoaderConfigTag::EndOfList => break,
                LoaderConfigTag::Log => {
                    // TODO: Reenable log
                },
                LoaderConfigTag::MainThreadHandle => config.main_thread = (*entry).data.0 as u32,
                LoaderConfigTag::OverrideHeap =>
                    config.override_heap = Some(((*entry).data.0 as _, (*entry).data.1 as usize)),
                LoaderConfigTag::AppletWorkaround => {},
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
