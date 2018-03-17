#![allow(unused_variables)]

// For compat reasons
extern crate cty;

use core::intrinsics::unreachable;

// TODO: not sure I like those
type Result = u32;
type Handle = u32;
type Debug = Handle;
type Process = Handle;
type Thread = Handle;
type DevAddrSpace = Handle;
type TransferMemory = Handle;
type SharedMemory = Handle;
type Session = Handle;
type Port = Handle;
type WEvent = Handle;
type Event = Handle;
type ThreadEntry = ::core::option::Option<unsafe extern "C" fn(arg1: *mut cty::c_void)>;

pub const CURRENT_PROCESS: u32 = 0xFFFF8001;
pub const CURRENT_THREAD: u32 = 0xFFFF8000;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct memory_info_t {
    pub base_addr: *mut cty::c_void,
    pub size: u64,
    pub memory_type: u32,
    pub memory_attribute: u32,
    pub permission: u32,
    pub device_ref_count: u32,
    pub ipc_ref_count: u32,
    pub padding: u32,
}

macro_rules! define_out00_svc {
    ($(#[$meta:meta])* $name:ident ( $id:expr, $($args:tt)* )) => {
        #[naked]
        $(#[$meta])*
        pub unsafe extern fn $name($($args)*) -> Result {
            asm!("svc $0
                 ret" : : "i"($id) : : "volatile");
            unreachable()
        }
    };
    ($(#[$meta:meta])* $name:ident ( $id:expr, $($args:tt)* ) -> $t:ty) => {
        #[naked]
        $(#[$meta])*
        pub unsafe extern fn $name($($args)*) -> $t {
            asm!("
                 svc $0
                 ret" : : "i"($id) : : "volatile");
            unreachable()
        }
    };
}

macro_rules! define_out32_svc {
    ($(#[$meta:meta])* $name:ident ( $id:expr, $($args:tt)* )) => {
        #[naked]
        $(#[$meta])*
        pub unsafe extern fn $name($($args)*) -> Result {
            asm!("
                str x0, [sp, #-0x10]!
                svc $0
                ldr x2, [sp]
                str w1, [x2]
                add sp, sp, #0x10
                ret" : : "i"($id) : : "volatile");
            unreachable()
        }
    };
}

macro_rules! define_out64_svc {
    ($(#[$meta:meta])* $name:ident ( $id:expr, $($args:tt)* )) => {
        #[naked]
        $(#[$meta])*
        pub unsafe extern fn $name($($args)*) -> Result {
            asm!("
                str x0, [sp, #-0x10]!
                svc $0
                ldr x2, [sp]
                str x1, [x2]
                add sp, sp, #0x10
                ret" : : "i"($id) : : "volatile");
            unreachable()
        }
    };
}

macro_rules! define_out32_arg2_svc {
    ($(#[$meta:meta])* $name:ident ( $id:expr, $($args:tt)* )) => {
        #[naked]
        $(#[$meta])*
        pub unsafe extern fn $name($($args)*) -> Result {
            asm!("
                str x1, [sp, #-0x10]!
                svc $0
                ldr x2, [sp]
                str w1, [x2]
                add sp, sp, #0x10
                ret" : : "i"($id) : : "volatile");
            unreachable()
        }
    };
}

macro_rules! define_out32_pair_svc {
    ($(#[$meta:meta])* $name:ident ( $id:expr, $($args:tt)* )) => {
        #[naked]
        $(#[$meta])*
        pub unsafe extern fn $name($($args)*) -> Result {
            asm!("
                stp x0, x1, [sp, #-0x10]!
                svc $0
                ldr x3, [sp]
                str w1, [x3]
                ldr x3, [sp, #8]
                str w2, [x3]
                add sp, sp, #0x10
                ret" : : "i"($id) : : "volatile");
            unreachable()
        }
    };
}


// TODO: Documentation for all syscalls. Let's try to do better than switchbrew.
// TODO: Rewrite the syscalls to wrap them in an idiomatic rust style.

define_out64_svc! {
    /// Sets the size of the heap. Equivalent to a brk syscall on unix.
    ///
    /// Note that you should almost never use this method! Use the system
    /// allocator instead, like Box, to allocate heap memory!
    /// @brief Sets the size of the heap
    ///
    /// @param outAddr Output for address of the heap
    /// @param size Size of the heap
    set_heap_size(0x01, outAddr: *mut *mut cty::c_void, size: u32)
}

define_out00_svc! {
    /// Sets memory permissions. Takes the address of the region to reprotect,
    /// its size, and the new permission of that region.
    ///
    /// Address and size must be page-aligned, and execute bit is not allowed.
    set_memory_permission(0x02, addr: *mut cty::c_void, size: u64, permission: u32)
}

define_out00_svc! {
    /// Sets memory attributes
    set_memory_attribute(0x03,
        addr: *mut cty::c_void,
        size: u64,
        state0: u32,
        state1: u32,)
}

define_out00_svc! {
    /// Map memory
    map_memory(0x04, dest: *mut cty::c_void, src: *mut cty::c_void, size: u64)
}

define_out00_svc! {
    /// Unmap memory
    unmap_memory(0x05, dest: *mut cty::c_void, src: *mut cty::c_void, size: u64)
}

define_out32_arg2_svc! {
    /// Query memory
    query_memory(0x06,
        memory_info: *mut memory_info_t,
        page_info: *mut u32,
        addr: *mut cty::c_void)
}

define_out00_svc! {
    /// Exit the process
    exit_process(0x07,) -> !
}

define_out32_svc! {
    /// Create a new thread
    create_thread(0x08,
        out: *mut Thread,
        entry: ThreadEntry,
        arg: u64,
        stacktop: *mut cty::c_void,
        priority: i32,
        processor_id: i32,)
}

define_out00_svc! {
    /// Start a thread. Takes a handle to the thread to be started.
    start_thread(0x09, thread: Thread)
}

define_out00_svc! {
    /// Exit thread
    exit_thread(0x0A,) -> !
}

define_out00_svc! {
    /// Sleep thread for specified time, in nanoseconds.
    sleep_thread(0x0B, nanos: u64)
}

define_out32_svc! {
    /// Get the thread priority
    get_thread_priority(0x0C, priority: *mut u32, thread: Thread)
}

define_out00_svc! {
    /// Set the thread priority
    set_thread_priority(0x0D, thread: Thread, priority: u32)
}

/// Get affinity mask of provided thread handle
#[naked]
pub unsafe extern fn get_thread_core_mask(mask1: *mut u32, mask2: *mut u64, thread: Thread) -> Result {
    asm!("
        stp x0, x1, [sp, #-0x10]!
        svc 0x0E
        ldr x3, [sp]
        str w1, [x3]
        ldr x3, [sp, #8]
        str x2, [x3]
        add sp, sp, #0x10
        ret" : : : : "volatile");
    unreachable()
}

define_out00_svc! {
    /// Set affinity mask of provided thread handle
    set_thread_core_mask(0x0F, thread: Thread, in1: u32, in2: u64)
}

define_out00_svc! {
    /// Get the current processor number
    get_current_processor_number(0x10,) -> u32
}

define_out00_svc! {
    /// Signal an event
    signal_event(0x11, event: WEvent)
}

define_out00_svc! {
    /// Clear an event
    clear_event(0x12, event: Event)
}

define_out00_svc! {
    /// Map shared memory
    map_shared_memory(0x13,
        block: SharedMemory,
        addr: *mut cty::c_void,
        size: u64,
        permission: u32)
}

define_out00_svc! {
    /// Unmap shared memory
    unmap_shared_memory(0x14,
        block: SharedMemory,
        addr: *mut cty::c_void,
        size: u64)
}

define_out32_svc! {
    /// Create transfer memory
    create_transfer_memory(0x15,
        out: *mut TransferMemory,
        addr: *mut cty::c_void,
        size: u64,
        permission: u32)
}

define_out00_svc! {
    /// Closes the specified handle
    close_handle(0x16, handle: Handle)
}

define_out00_svc! {
    /// Resets a signal. Takes a revent or process.
    reset_signal(0x17, signal: Handle)
}

define_out32_svc! {
    /// Wait synchronization
    wait_synchronization(0x18,
        handle_index: *mut u32,
        handles: *mut Handle,
        num_handles: u32,
        timeout: u64)
}

define_out00_svc! {
    /// Cancel synchronization
    cancel_synchronization(0x19, handle: Handle)
}

define_out00_svc! {
    /// Arbitrate lock
    arbitrate_lock(0x1A,
        current_thread: Thread,
        lock: *mut cty::c_void,
        requesting_thread: Thread) -> ()
}

define_out00_svc! {
    /// Arbitrate unlock
    arbitrate_unlock(0x1B, lock: *mut cty::c_void) -> ()
}

define_out00_svc! {
    /// Wait process wide key atomic
    wait_process_wide_key_atomic(0x1C,
        ptr0: *mut cty::c_void,
        ptr1: *mut cty::c_void,
        thread: Thread,
        timeout: u64)
}

define_out00_svc! {
    /// Signal process wide key atomic
    signal_process_wide_key(0x1D, ptr: *mut cty::c_void, value: u32)
}

define_out00_svc! {
    /// Get the system time tick
    get_system_tick(0x1E,) -> u64
}

define_out32_svc! {
    /// Connect to a named port
    connect_to_named_port(0x1F, out: *mut Session, name: *const cty::c_char)
}

// TODO:
//define_out00_svc! {
//    send_sync_requestion_light(0x20, session: Session, unknown: *mut ())
//}

define_out00_svc! {
    /// Send sync request
    send_sync_request(0x21, session: Session)
}


define_out00_svc! {
    /// Send sync request with user buffer
    send_sync_request_with_user_buffer(0x22,
        buffer: *mut cty::c_void,
        size: u64,
        session: Session)
}

// TODO:
//define_out32_svc! {
//    /// Send sync request with user buffer
//    send_async_request_with_user_buffer(0x23,
//        event: *mut Event,
//        buffer: *mut cty::c_void,
//        size: u64,
//        session: Session)
//}

define_out64_svc! {
    /// Get a process's ID. Takes a handle to a process or thread, and returns
    /// its process ID.
    get_process_id(0x24, pid: *mut u32, thread_or_Processandle: Handle)
}

define_out64_svc! {
    /// Get a thread's ID. Takes a handle to that thread, and returns its ID.
    get_thread_id(0x25, handle_out: *mut Thread, handle_in: Thread)
}

define_out00_svc! {
    /// Real svc name is break, but that's a keyword in rust. So we renamed it
    /// to crash.
    // TODO: It... might return. Sometimes.
    crash(0x26, break_reason: u64, unk1: u64, info: u64) -> ()
}

define_out00_svc! {
    /// Output a debug string
    output_debug_string(0x27, str: *const u8, size: usize) -> ()
}

define_out00_svc! {
    /// Return from exception
    return_from_exception(0x28, result: u64) -> ()
}

define_out64_svc! {
    /// Get info about a handle.
    ///
    /// Handle Type | `info_id` | `info_sub_id`         | Description
    /// ------------|-----------|-----------------------|-----------------------
    /// Process     | 0         | 0                     | AllowedCpuIdBitmask
    /// Process     | 1         | 0                     | AllowedThreadPrioBitmask
    /// Process     | 2         | 0                     | MapRegionBaseAddr
    /// Process     | 3         | 0                     | MapRegionSize
    /// Process     | 4         | 0                     | HeapRegionBaseAddr
    /// Process     | 5         | 0                     | HeapRegionSize
    /// Process     | 6         | 0                     | TotalMemoryAvailable. Total memory available(free+used).
    /// Process     | 7         | 0                     | TotalMemoryUsage. Total used size of codebin memory + main-thread stack + allocated heap.
    /// Zero        | 8         | 0                     | IsCurrentProcessBeingDebugged
    /// Zero        | 9         | 0                     | Returns ResourceLimit handle for current process. Used by PM.
    /// Zero        | 10        | -1, {current coreid}  | IdleTickCount
    /// Zero        | 11        | 0-3                   | RandomEntropy from current process. TRNG. Used to seed usermode PRNGs.
    /// Process     | 12        | 0                     | [2.0.0+] AddressSpaceBaseAddr
    /// Process     | 13        | 0                     | [2.0.0+] AddressSpaceSize
    /// Process     | 14        | 0                     | [2.0.0+] NewMapRegionBaseAddr
    /// Process     | 15        | 0                     | [2.0.0+] NewMapRegionSize
    /// Process     | 16        | 0                     | [3.0.0+] IsVirtualAddressMemoryEnabled
    /// Process     | 17        | 0                     | [3.0.0+] Some size in bytes.
    /// Process     | 18        | 0                     | [3.0.0+] TitleId
    /// Zero        | 19        | 0                     | [4.0.0+] PrivilegedProcessId_LowerBound
    /// Zero        | 19        | 1                     | [4.0.0+] PrivilegedProcessId_UpperBound
    /// Thread      | 0xF0000002| 0                     | Performance counter related. 
    get_info(0x29, info: *mut u64, info_id: u64, handle: Handle, info_sub_id: u64)
}

define_out00_svc! {
    flush_entire_data_cache(0x2A,) -> ()
}

define_out00_svc! {
    flush_data_cache(0x2B, addr: *mut (), size: usize)
}

define_out00_svc! {
    #[cfg(feature = "switch-3.0.0")]
    map_physical_memory(0x2C, addr: *mut (), size: usize)
}

define_out00_svc! {
    #[cfg(feature = "switch-3.0.0")]
    unmap_physical_memory(0x2D, addr: *mut (), size: usize)
}

// TODO: Apparently returns a bunch of crap
//define_out00_svc! {
//    #[cfg(feature = "switch-5.0.0")]
//    get_next_thread_info(0x2E)
//}

// TODO: will have to be written manually.
//get_last_thread_info(0x2F)

define_out64_svc! {
    get_resource_limit_limit_value(0x30, val: *mut u64, reslimit_handle: u32, limitable_resource: u32)
}

define_out64_svc! {
    get_resource_limit_current_value(0x31, val: *mut u64, reslimit_handle: u32, limitable_resource: u32)
}

// TODO:
// define_out00_svc! {
//     set_thread_activity(0x32)
// }
//
// define_out00_svc! {
//     get_thread_context3(0x33)
// }
//
// define_out00_svc! {
//     #[cfg(feature = "switch-4.0.0")]
//     wait_for_address(0x34)
// }
//
// define_out00_svc! {
//     #[cfg(feature = "switch-4.0.0")]
//     signal_to_address(0x35)
// }

// Hole until 0x3C

// define_out00_svc! {
//     dump_info(0x3C)
// }
//
// define_out00_svc! {
//     #[cfg(feature = "switch-4.0.0")]
//     dump_info_new(0x3D)
// }

// Hole until 0x40

define_out32_pair_svc! {
    create_session(0x40,
        server: *mut Session,
        client: *mut Session,
        is_light: bool,
        unknown: u32)
}

define_out32_svc! {
    /// Accept session
    accept_session(0x41, out: *mut Session, port: Port)
}

// TODO: Loads of registers to get the output
// define_out00_svc! {
//     reply_and_receive_light(0x42, hande: Session)
// }

define_out32_svc! {
    /// Reply and recieve
    reply_and_receive(0x43,
        handle_idx: *mut u32,
        handles: *mut Session,
        num_handles: u32,
        reply_session: Session,
        timeout: u64)
}

define_out32_svc! {
    /// Reply and recieve with user buffer
    reply_and_receive_with_user_buffer(0x44,
        handle_idx: *mut u32,
        buffer: *mut cty::c_void,
        size: u64,
        handles: *mut Session,
        num_handles: u32,
        reply_session: Session,
        timeout: u64)
}

// create_event(0x45)
//
// Hole until 0x48
//
// [5.0.0+] allocate_user_heap_memory(0x48)
// [5.0.0+] free_user_heap_memory(0x49)
// [5.0.0+] set_user_heap_memory_allocation_max(0x4A)
// [4.0.0+] create_code_memory(0x4B)
// [4.0.0+] control_code_memory(0x4C)
// sleep_system(0x4D)

define_out32_svc! {
    /// Read/Write register
    read_write_register(0x4E,
        out_value: *mut u32,
        addr: u64,
        rw_mask: u32,
        in_value: u32)
}

// set_process_activity(0x4F)

define_out32_svc! {
    /// Create a block of shared memory
    create_shared_memory(0x50,
        out: *mut SharedMemory,
        size: u64,
        self_permissions: u32,
        foreign_permissions: u32)
}

/// Map transfer memory
define_out00_svc! {
    map_transfer_memory(0x51,
        handle: TransferMemory,
        addr: *mut cty::c_void,
        size: u64,
        perm: u32)
}

/// Unmap transfer memory
define_out00_svc! {
    unmap_transfer_memory(0x52,
        handle: TransferMemory,
        addr: *mut cty::c_void,
        size: u64)
}

// create_interrupt_event(0x53)
// query_physical_address(0x54)

define_out64_svc! {
    /// Query IO mapping
    query_io_mapping(0x55, virt_addr: *mut cty::c_void, phys_addr: u64, size: u64)
}

// create_device_address_space

define_out00_svc! {
    /// Attach device address space
    attach_device_address_space(0x57, device: u32, space: DevAddrSpace)
}

define_out00_svc! {
    /// Detach device address space
    detach_device_address_space(0x58, device: u32, space: DevAddrSpace)
}

define_out00_svc! {
    /// Map device address space by force
    map_device_address_space_by_force(0x59,
        space: DevAddrSpace,
        process: Process,
        dev_addr: u64,
        dev_size: u64,
        map_addr: u64,
        perm: u32)
}

define_out00_svc! {
    /// Map device address space aligned
    map_device_address_space_aligned(0x5A,
        space: DevAddrSpace,
        process: Process,
        dev_addr: u64,
        dev_size: u64,
        map_addr: u64,
        perm: u32)
}

// map_device_address_space(0x5B)

define_out00_svc! {
    /// Unmap device address space
    unmap_device_address_space(0x5C,
        space: DevAddrSpace,
        process: Process,
        map_addr: u64,
        map_size: u64,
        perm: u32)
}

// invalidate_process_data_cache(0x5D)
// store_process_data_cache(0x5E)
// flush_process_data_cache(0x5F)

define_out00_svc! {
    /// Debug active process
    debug_active_process(0x60, out: *mut Debug, process_id: u64)
}

// break_debug_process(0x61)
// terminate_debug_process(0x62)
// get_debug_event(0x63)
// continue_debug_event(0x64)
// get_process_list(0x65)
// get_thread_list(0x66)
// get_debug_thread_context(0x67)
// set_debug_thread_context(0x68)

define_out00_svc! {
    /// Query debug process memory
    query_debug_process_memory(0x69,
        memory_info: *mut memory_info_t,
        page_info: *mut u32,
        debug: Debug,
        addr: u64)
}

define_out00_svc! {
    /// Read debug process memory
    read_debug_process_memory(0x6A,
        buffer: *mut cty::c_void,
        debug: Debug,
        addr: u64,
        size: u64)
}

define_out00_svc! {
    /// Write debug process memory
    write_debug_process_memory(0x6B,
        debug: Debug,
        buffer: *mut cty::c_void,
        addr: u64,
        size: u64)
}

// set_hardware_break_point(0x6C)
// get_debug_thread_param(0x6D)
//
// Hole on 0x6E
//
// [5.0.0+] get_memory_info(0x6F)
//
// create_port(0x70)
// manage_named_port(0x71)
// connect_to_port(0x72)
//
// set_process_memory_permission(0x73)
// map_process_memory(0x74)
// unmap_process_memory(0x75)
// query_process_memory(0x76)
// map_process_code_memory(0x77)
// unmap_process_code_memory(0x78)
// create_process(0x79)
// start_process(0x7A)
// terminate_process(0x7B)
// get_process_info(0x7C)
// create_resource_limit(0x7D)
// set_resource_limit_limit_value(0x7E)
// call_secure_monitor(0x7F)
