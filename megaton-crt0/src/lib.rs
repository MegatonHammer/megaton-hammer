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

use core::{ptr, slice, intrinsics};
use core::fmt::Write;

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

pub extern fn __svcLog(s: &str) -> usize {
    let str_bytes = s.as_ptr();
    let str_len = s.len();
    let out;
    // TODO: Clobbers
    unsafe { asm!("svc 0x27" : "={x0}"(out) : "{x0}"(str_bytes), "{x1}"(str_len) : : "volatile"); }
    out
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
    writeln!(SvcLog, "Returning with value {}", ret);
    ret
}

// TODO: I should try to get rid of that one last bit of global asm.
global_asm!(r#"
.section .data.mod0
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
#[macro_use]
mod util;

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
    pub const AppletWorkaround: u32 = 8;
    pub const StdioSockets: u32 = 9;
    pub const ProcessHandle: u32 = 10;
    pub const LastLoadResult: u32 = 11;
    pub const AllocPages: u32 = 12;
    pub const LockRegion: u32 = 13;
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
#[cfg(feature = "log")]
pub struct LoaderLog(Option<io::Cursor<&'static mut [u8]>>);

// TODO: Don't leave it uninitialized. Stick it in an Option<> or something.
#[cfg(feature = "log")]
pub static LOG : spin::Mutex<LoaderLog> = spin::Mutex::new(LoaderLog(None));

#[cfg(feature = "log")]
impl core::fmt::Write for LoaderLog {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        use io::Write;
        self.0.as_mut().unwrap().write(s.as_bytes()).unwrap();
        Ok(())
    }
}

pub struct SvcLog;

impl core::fmt::Write for SvcLog {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        __svcLog(s);
        Ok(())
    }
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

    if !config.is_null() {
        loop {
            match (*config).tag {
                LoaderConfigTag::EndOfList => break,
                LoaderConfigTag::Log => {
                    #[cfg(feature = "log")]
                    {
                        (*LOG.lock()).0 = Some(io::Cursor::new(slice::from_raw_parts_mut((*config).data.0 as _, (*config).data.1 as _)));
                    }
                },
                // TODO: Make MainThreadHandle not mandatory, because it *really* isn't.
                LoaderConfigTag::MainThreadHandle => {},
                LoaderConfigTag::OverrideHeap => {},
                LoaderConfigTag::AppletWorkaround => {},
                tag => {
                    if (*config).flags & 1 == 1 {
                        // Flag is mandatory! If we don't know about it, we
                        // should commit suicide.
                        return 346 | ((100 + tag) << 9) as i32;
                    }
                }
            }
            config = config.offset(1);
        }
    } else {
        // There was no LoaderConfig, we're on our own to set ourselves up in a
        // sane way.
        // TODO: Seems totally safe. TOTALLY. SAFE.
        #[cfg(feature = "log")]
        {
            (*LOG.lock()).0 = Some(io::Cursor::new(slice::from_raw_parts_mut(ptr::null_mut(), 0)));
        }
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
fn panic_fmt(msg: core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> ! {
    // We panic'd, locks might already be taken. Let's avoid infinite looping there.
    if let Some(mut lock) = LOG.try_lock() {
        writeln!(lock, "PANIC: {} in {}:{}:{}", msg, file, line, column);
    }
    // Let's also send it to the debug svc
    writeln!(SvcLog, "PANIC: {} in {}:{}:{}", msg, file, line, column);

    // TODO: Exit the program. Turns out this is surprisingly difficult.
    // NOTE: This will not unwind the stack. If you panic, we'll almost
    // certainly leak resources.
    loop {}
}
