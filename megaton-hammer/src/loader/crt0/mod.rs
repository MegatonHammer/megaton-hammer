//! Megaton-CRT0
//!
//! This module is a minimal RT0 that rust application can link against to avoid
//! dependint on the libstd.
//!
//! It handles relocation, LoaderConfig, and finally calls the main function.
//! Note that this module is only required if you wish to not use libstd (which
//! naturally includes its own runtime).

extern crate core_io as io;
use core;

use core::intrinsics;
use loader::LoaderConfigEntry;

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
pub unsafe extern fn __svc_exit_process() -> ! {
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
    adrp x2, __svc_exit_process
    add x2, x2, #:lo12:__svc_exit_process
    cmp x30, xzr
    csel x2, x2, x30, eq
    b start" : : : : "volatile");
    intrinsics::unreachable();
}

static mut EXIT: Option<extern fn(u64) -> !> = None;

#[no_mangle]
pub unsafe extern fn start(config: *mut LoaderConfigEntry, _thread_handle: u64, exit: extern fn(u64) -> !) -> ! {
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

    EXIT = Some(exit);

    let start_addr : *mut ();
    // TODO: Avoid relocations by getting _start address with asm...
    asm!("adrp $0, _start" : "=r"(start_addr));
    let ret = megaton_start(config, _thread_handle, start_addr);
    exit(ret as u64);
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

unsafe extern fn megaton_start(config: *mut LoaderConfigEntry, _thread_handle: u64, aslr_base: *mut ()) -> i32 {
    let mut dyn_info = relocation::DynInfo {
        init_array: None,
        fini_array: None
    };


    if let Err(x) = relocation::relocate(aslr_base as _, &mut dyn_info) {
        return -(x as i32);
    }

    use loader;
    if let Err(err) = loader::init_loader(config) {
        return err;
    }

    // TODO: Might want to run init_array 👀

    // Initialize the main thread's context.
    use tls;
    tls::TlsStruct::init();

    extern {
        fn main(argc: isize, argv: *const *const u8) -> i32;
    }

    // The arguments are in the config, but I'm a bit too lazy to try it right
    // now.
    let _ret = main(0, core::ptr::null());

    _ret
}


#[cfg(feature = "crt0")]
mod langitems {
    extern crate compiler_builtins;
    use core;

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
    fn panic_fmt(_msg: core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! {
        // We panic'd, locks might already be taken. Let's avoid infinite looping there.
        /*if let Some(mut lock) = LOG.try_lock() {
            writeln!(lock, "PANIC: {} in {}:{}:{}", msg, file, line, column);
        }
        // Let's also send it to the debug svc
        writeln!(SvcLog, "PANIC: {} in {}:{}:{}", msg, file, line, column);*/
    
        // TODO: Exit the program. Turns out this is surprisingly difficult.
        // NOTE: This will not unwind the stack. If you panic, we'll almost
        // certainly leak resources.
        unsafe {
            match super::EXIT {
                Some(f) => f(1),
                None => super::__svc_exit_process()
            }
        }
    }
}
