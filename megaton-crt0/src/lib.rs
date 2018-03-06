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

#![feature(global_asm, naked_functions, core_intrinsics, asm, lang_items, compiler_builtins_lib)]
#![no_std]

extern crate compiler_builtins;

use core::{ptr, slice, intrinsics};

#[naked]
#[no_mangle]
#[link_section = ".text.jmp"]
pub unsafe extern fn _start() -> ! {
    asm!("b start
          .word _mod_header - _start" : : : : "volatile");
    intrinsics::unreachable();
}

#[naked]
#[no_mangle]
pub unsafe extern fn __svcExitProcess() -> ! {
    asm!("svc 0x07" : : : : "volatile");
    intrinsics::unreachable();
}

#[naked]
#[no_mangle]
pub unsafe extern fn start() -> ! {
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
            adrp x2, _start // aslr base

            // set LR to svcExitProcess if it's null
            adrp x3, __svcExitProcess
            add x3, x3, #:lo12:__svcExitProcess
            cmp x30, xzr
            csel x30, x3, x30, eq

            sub sp, sp, 0x10
            stp x29, x30, [sp]
            bl megaton_start
            ldp x29, x30, [sp], 0x10
            ret" : : : : "volatile");
    intrinsics::unreachable();
}

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

pub struct LoaderConfig;

#[derive(Debug)]
struct DynInfo<'a, 'b> {
    init_array: Option<&'a [extern fn()]>,
    fini_array: Option<&'b [extern fn()]>,
}

extern {
    fn main(argc: isize, argv: *const *const u8) -> u32;
}

#[no_mangle]
pub unsafe extern fn megaton_start(_config: *mut LoaderConfig, _thread_handle: u64, aslr_base: *mut ()) {
    let mut dyn_info = DynInfo {
        init_array: None,
        fini_array: None
    };

    if let Err(_) = relocate(aslr_base as _, &mut dyn_info) {
        return;
    }

    // TODO: Might want to run init_array 👀

    // The arguments are in the config, but I'm a bit too lazy to try it right
    // now.
    let _ret = main(0, core::ptr::null());
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

// -- Relocation stuff --
#[repr(C)]
struct ModuleHeader {
    magic: u32,
    dynamic_off: u32,
    bss_start_off: u32,
    bss_end_off: u32,
    unwind_start_off: u32,
    unwind_end_off: u32,
    module_object_off: u32
}

#[repr(C)]
struct Elf64Dyn {
    tag: i64,
    val: u64, // Who cares about the union ಠ_ಠ
}

#[repr(C)]
struct Elf64Rela {
    offset: u64,
    reloc_type: u32,
    symbol: u32,
    addend: u64
}

const DT_RELA: i64 = 7;
const DT_RELASZ: i64 = 8;
const DT_RELAENT: i64 = 9;
const DT_INIT_ARRAY: i64 = 25;
const DT_FINI_ARRAY: i64 = 26;
const DT_INIT_ARRAYSZ: i64 = 27;
const DT_FINI_ARRAYSZ: i64 = 28;
const DT_RELACOUNT: i64 = 0x6ffffff9;

const R_AARCH64_RELATIVE: u32 = 0x403;

/// Do the dynamic linker's job. Should be the first function called.
unsafe fn relocate(aslr_base: *mut u8, dyn_info: &mut DynInfo) -> Result<(), ()> {
    let mod_header = aslr_base.offset(*(aslr_base.offset(4) as *const u32) as isize) as *mut ModuleHeader;
    let mod_ptr = mod_header as *const u8;


    let mut rela_offset = None;
    let mut rela_size = 0;
    let mut rela_ent = 0;
    let mut rela_count = 0;
    let mut init_array = ptr::null() as *const extern fn();
    let mut init_array_sz = None;
    let mut fini_array = ptr::null() as *const extern fn();
    let mut fini_array_sz = None;

    // Ensure we have a valid MOD0 header
    let mod_magic = 0x30444f4d; // 'MOD0'
    if (*mod_header).magic != mod_magic {
        return Err(());
    }

    // Traverse the dynamic section, extract all relevant information.
    let mut dynamic = mod_ptr.offset((*mod_header).dynamic_off as isize) as *const Elf64Dyn;
    while (*dynamic).tag > 0 {
        match (*dynamic).tag {
            DT_RELA => {
                if rela_offset.is_some() {
                    return Err(());
                }
                rela_offset = Some((*dynamic).val);
            },
            DT_RELASZ => {
                rela_size = (*dynamic).val;
            }
            DT_RELAENT => {
                rela_ent = (*dynamic).val;
            }
            DT_INIT_ARRAY => {
                if !init_array.is_null() {
                    return Err(());
                }
                init_array = aslr_base.offset((*dynamic).val as isize) as _;
            },
            DT_FINI_ARRAY => {
                if !fini_array.is_null() {
                    return Err(());
                }
                fini_array = aslr_base.offset((*dynamic).val as isize) as _;
            }
            DT_INIT_ARRAYSZ => {
                if init_array_sz.is_some() {
                    return Err(())
                }
                init_array_sz = Some((*dynamic).val);
            }
            DT_FINI_ARRAYSZ => {
                if fini_array_sz.is_some() {
                    return Err(())
                }
                fini_array_sz = Some((*dynamic).val);
            },
            DT_RELACOUNT => {
                rela_count = (*dynamic).val;
            },
            _ => {
                // UNHANDLED! We *might* want to log this, but we can't safely
                // access anything.
            }
        }
        dynamic = dynamic.offset(1);
    }

    // Save init_array/fini_array somewhere the crt0 can find.
    match (init_array, init_array_sz) {
        (p, Some(sz)) if !p.is_null() => {
            dyn_info.init_array = Some(slice::from_raw_parts(p, sz as usize));
        },
        (p, None) if p.is_null() => (),
        _ => {
            // We had an INIT_ARRAY but no INIT_ARRAYSZ. What ?
            return Err(())
        }
    }
    match (fini_array, fini_array_sz) {
        (p, Some(sz)) if !p.is_null() => {
            dyn_info.fini_array = Some(slice::from_raw_parts(p, sz as usize));
        },
        (p, None) if p.is_null() => (),
        _ => {
            // We had a FINI_ARRAY but no FINI_ARRAYSZ. What ?
            return Err(())
        }
    }

    // TODO: Comment explaining why ?
    if rela_ent != 0x18 {
        return Err(());
    }

    if rela_size != rela_count * rela_ent {
        return Err(());
    }

    // Handle relocations. On the switch, there is no dynamic linker: it's up to
    // the process to link itself correctly.
    if let Some(rela_offset) = rela_offset {
        let rela_base = (aslr_base.offset(rela_offset as isize)) as *const Elf64Rela;
        for i in 0..rela_count {
            let rela = rela_base.offset(i as isize);

            match (*rela).reloc_type {
                R_AARCH64_RELATIVE => {
                    if (*rela).symbol != 0 {
                        return Err(());
                    }
                    *(aslr_base.offset((*rela).offset as isize) as *mut *mut ()) = aslr_base.offset((*rela).addend as isize) as _;
                    break;
                },
                _ => return Err(())
            }
        }
    } else {
        return Err(())
    }

    return Ok(());
}
