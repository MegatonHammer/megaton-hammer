use core::{ptr, slice};

#[derive(Debug)]
pub struct DynInfo<'a, 'b> {
    pub init_array: Option<&'a [extern fn()]>,
    pub fini_array: Option<&'b [extern fn()]>,
}

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
#[inline(always)]
pub unsafe fn relocate(aslr_base: *mut u8, dyn_info: &mut DynInfo) -> Result<(), u32> {
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
        return Err(13);
    }


    // Traverse the dynamic section, extract all relevant information.
    let mut dynamic = mod_ptr.offset((*mod_header).dynamic_off as isize) as *const Elf64Dyn;
    while (*dynamic).tag > 0 {
        match (*dynamic).tag {
            DT_RELA => {
                if rela_offset.is_some() {
                    return Err(1);
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
                    return Err(2);
                }
                init_array = aslr_base.offset((*dynamic).val as isize) as _;
            },
            DT_FINI_ARRAY => {
                if !fini_array.is_null() {
                    return Err(3);
                }
                fini_array = aslr_base.offset((*dynamic).val as isize) as _;
            }
            DT_INIT_ARRAYSZ => {
                if init_array_sz.is_some() {
                    return Err(4)
                }
                init_array_sz = Some((*dynamic).val);
            }
            DT_FINI_ARRAYSZ => {
                if fini_array_sz.is_some() {
                    return Err(5)
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

    // TODO: Comment explaining why ?
    if rela_ent != 0x18 {
        return Err(6);
    }

    if rela_size != rela_count * rela_ent {
        return Err(7);
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
                        return Err(8);
                    }

                    *(aslr_base.offset((*rela).offset as isize) as *mut *mut ()) = aslr_base.offset((*rela).addend as isize) as _;
                },
                _ => return Err(9)
            }
        }
    } else {
        return Err(10)
    }

    // Relocations are handled! Calling functions should be fine, so long as
    // they do not rely on data being initialized.

    // Save init_array/fini_array somewhere the crt0 can find.
    match (init_array, init_array_sz) {
        (p, Some(sz)) if !p.is_null() => {
            dyn_info.init_array = Some(slice::from_raw_parts(p, sz as usize));
        },
        (p, None) if p.is_null() => (),
        _ => {
            // We had an INIT_ARRAY but no INIT_ARRAYSZ. What ?
            return Err(11)
        }
    }
    match (fini_array, fini_array_sz) {
        (p, Some(sz)) if !p.is_null() => {
            dyn_info.fini_array = Some(slice::from_raw_parts(p, sz as usize));
        },
        (p, None) if p.is_null() => (),
        _ => {
            // We had a FINI_ARRAY but no FINI_ARRAYSZ. What ?
            return Err(12)
        }
    }

    return Ok(());
}
