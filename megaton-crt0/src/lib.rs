//! Megaton-CRT0
//!
//! This crate is a minimal CRT0 that rust application can link against. It will
//! handle relocation and call the main function. Note that this crate is only
//! required if you wish to not use libstd (which naturally includes
//! its own runtime).

// TODO: Reducing the amount of global_asm to the absolute bare minimum would
// be nice.
/*#[naked]
#[export_name = "_start"]
#[link_section = ".text.jmp"]
pub extern "C" fn entry() -> ! {
    asm!("b start" : : : : "volatile");
    // TODO: intrinsics::unreachable()
    loop {}
}*/

#![feature(global_asm, naked_functions, core_intrinsics, asm, lang_items, compiler_builtins_lib, untagged_unions)]
#![no_std]

extern crate compiler_builtins;

#[cfg(feature = "log")]
extern crate core_io as io;
#[cfg(feature = "log")]
extern crate spin;

use core::{slice, intrinsics};

#[naked]
#[no_mangle]
#[link_section = ".text.jmp"]
pub unsafe extern fn _start() -> ! {
    asm!("
    b trampoline
   .word _mod_header - _start
    " : : : : "volatile");
    intrinsics::unreachable();
}

#[naked]
#[no_mangle]
pub unsafe extern fn __svcExitProcess() -> ! {
    // TODO: Clobbers
    asm!("svc 0x07" : : : : "volatile");
    intrinsics::unreachable();
}

// Make sure x30 is set to something.
#[naked]
#[no_mangle]
pub unsafe extern fn trampoline() -> ! {
    // TODO: Clobber
    asm!("
    adrp x3, __svcExitProcess
    add x3, x3, #:lo12:__svcExitProcess
    cmp x30, xzr
    csel x30, x3, x30, eq
    b start" : : : : "volatile");
    intrinsics::unreachable();
}

#[no_mangle]
pub unsafe extern fn start(config: *mut LoaderConfigEntry, _thread_handle: u64) -> i32 {
    // Clean the BSS.
    // TODO: Avoid relocations by using asm!. This is **ugly**.
    asm!("
	// clear .bss
	adrp x5, __bss_start
	adrp x6, __bss_end

bssloop:
	cmp x5, x6
	b.eq run
	str xzr, [x5]
	add x5, x5, 8
	b bssloop
run:
    " : : : "x5", "x6" : "volatile");

    let start_addr : *mut ();
    // TODO: Avoid relocations by getting _start address with asm...
    asm!("adrp $0, _start" : "=r"(start_addr));
    let ret = megaton_start(config, _thread_handle, start_addr);
    ret
}

// TODO: I should try to get rid of that one last bit of asm.
#[link_section = ".data.mod0"]
global_asm!(r#"
.global _mod_header
_mod_header:
    .ascii "MOD0"
    .word __dynamic_start - _mod_header
    .word __bss_start - _mod_header
    .word __bss_end - _mod_header
    .word 0, 0 // eh_frame_hdr start/end
    .word 0 // runtime-generated module object offset
.global IS_NRO
IS_NRO:
    .word 1
"#);

mod relocation;

/*pub enum LoaderConfigEntry {
    EndOfList = 0,
    MainThreadHandle(u32),
    NextLoadPath(*const u8, *const u8),
    OverrideHeap(*const (), usize),
    OverrideService(u64, u32),
    Argv(u64, *const *const u8),
    SyscallAvailableHint([u8; 16]),
    AppletType(u64),
    StdioSockets(u32, u32, u32, u64),
    ProcessHandle(u32),
    LastLoadResult(u32),
    AllocPages,
    LockRegion(*const (), usize),
    Log(*const u8, usize) = 51
}*/

mod LoaderConfigTag {
    pub const EndOfList: u32 = 0;
    pub const MainThreadHandle: u32 = 1;
    pub const NextLoadPath: u32 = 2;
    pub const OverrideHeap: u32 = 3;
    pub const OverrideService: u32 = 4;
    pub const Argv: u32 = 5;
    pub const SyscallAvailableHint: u32 = 6;
    pub const AppletType: u32 = 7;
    pub const StdioSockets: u32 = 8;
    pub const ProcessHandle: u32 = 9;
    pub const LastLoadResult: u32 = 10;
    pub const AllocPages: u32 = 11;
    pub const LockRegion: u32 = 12;
    pub const Log: u32 = 51;
}

#[repr(C)]
union LoaderConfigEntryData {
    // TODO: All of them.
    log: (*const u8, usize)

}

#[repr(C)]
pub struct LoaderConfigEntry {
    tag: u32,
    flags: u32,
    data: (u64, u64)
}

extern {
    fn main(argc: isize, argv: *const *const u8) -> i32;
}

// TODO: Should this be here? I need to clean up who is responsible for what.
// TODO: THIS IS TERRIFYING!
#[cfg(feature = "log")]
static mut LOG : spin::Mutex<io::Cursor<&'static mut [u8]>> = unsafe { JUNK.data };

#[cfg(feature = "log")]
union TerrifyingUnion {
    data: spin::Mutex<io::Cursor<&'static mut [u8]>>,
    junk: u32
}

#[cfg(feature = "log")]
const JUNK: TerrifyingUnion = TerrifyingUnion { junk: 0 };

#[cfg(feature = "log")]
pub fn write_to_log(s: &str) {
    use io::Write;

    unsafe { LOG.lock().write(s.as_bytes()); }
}

#[no_mangle]
unsafe extern fn megaton_start(mut config: *mut LoaderConfigEntry, _thread_handle: u64, aslr_base: *mut ()) -> i32 {
    let mut dyn_info = relocation::DynInfo {
        init_array: None,
        fini_array: None
    };

    if let Err(x) = relocation::relocate(aslr_base as _, &mut dyn_info) {
        return -(x as i32);
    }

    loop {
        match (*config).tag {
            LoaderConfigTag::EndOfList => break,
            LoaderConfigTag::Log => {
                #[cfg(feature = "log")]
                {
                    LOG = spin::Mutex::new(io::Cursor::new(slice::from_raw_parts_mut((*config).data.0 as _, (*config).data.1 as _)));
                }
            },
            _ => {
                if (*config).flags & 1 == 0 {
                    // Is mandatory! Let's suicide.
                    return -2;
                }
            }
        }
        config = config.offset(1);
    }
    // Handle Loader Config

    // TODO: Might want to run init_array 👀

    // The arguments are in the config, but I'm a bit too lazy to try it right
    // now.
    let _ret = main(0, core::ptr::null());

    _ret
}

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

#[lang = "start"]
extern "C" fn rust_start<T>(main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize 
where
T: Termination,
{
    main();
    0
}

#[lang = "panic_fmt"]
fn panic_fmt(_msg: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}
