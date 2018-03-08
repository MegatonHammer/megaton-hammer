// For compat reasons

extern crate cty;

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

// TODO: Documentation for all syscalls. Let's try to do better than switchbrew.
// TODO: Rewrite the syscalls to wrap them in an idiomatic rust style.
extern {
    /// Sets the size of the heap. Equivalent to a brk syscall on unix.
    ///
    /// Note that you should almost never use this method! Use the system
    /// allocator instead, like Box, to allocate heap memory!
    /// @brief Sets the size of the heap
    ///
    /// @param outAddr Output for address of the heap
    /// @param size Size of the heap
    // Yes, this takes an u32. Yes, this is odd.
    #[link_name = "svcSetHeapSize"]
    pub fn set_heap_size(outAddr: *mut *mut cty::c_void, size: u32) -> Result;

    /// Sets memory permissions. Takes the address of the region to reprotect,
    /// its size, and the new permission of that region.
    ///
    /// Address and size must be page-aligned, and execute bit is not allowed.
    #[link_name = "svcSetMemoryPermission"]
    pub fn set_memory_permission(addr: *mut cty::c_void, size: u64, permission: u32) -> Result;

    /// Sets memory attributes
    #[link_name = "svcSetMemoryAttribute"]
    pub fn set_memory_attribute(
        addr: *mut cty::c_void,
        size: u64,
        state0: u32,
        state1: u32,
    ) -> Result;

    /// Map memory
    #[link_name = "svcMapMemory"]
    pub fn map_memory(dest: *mut cty::c_void, src: *mut cty::c_void, size: u64) -> Result;

    /// Unmap memory
    #[link_name = "svcUnmapMemory"]
    pub fn unmap_memory(dest: *mut cty::c_void, src: *mut cty::c_void, size: u64) -> Result;

    /// Query memory
    #[link_name = "svcQueryMemory"]
    pub fn query_memory(
        memory_info: *mut memory_info_t,
        page_info: *mut u32,
        addr: *mut cty::c_void,
    ) -> Result;

    /// Exit the process
    #[link_name = "svcExitProcess"]
    pub fn exit_process() -> !;

    /// Create a new thread
    #[link_name = "svcCreateThread"]
    pub fn create_thread(
        out: *mut Thread,
        entry: ThreadEntry,
        arg: u64,
        stacktop: *mut cty::c_void,
        priority: i32,
        processor_id: i32,
    ) -> Result;

    /// Start a thread. Takes a handle to the thread to be started.
    #[link_name = "svcStartThread"]
    pub fn start_thread(thread: Thread) -> Result;

    /// Exit thread
    #[link_name = "svcExitThread"]
    pub fn exit_thread() -> !;

    /// Sleep thread for specified time, in nanoseconds.
    #[link_name = "svcSleepThread"]
    pub fn sleep_thread(nanos: u64) -> Result;

    /// Get the thread priority
    #[link_name = "svcGetThreadPriority"]
    pub fn get_thread_priority(priority: *mut u32, thread: Thread) -> Result;

    /// Set thread core mask
    #[link_name = "svcSetThreadCoreMask"]
    pub fn set_thread_core_mask(thread: Thread, in_: u32, in2: u64) -> Result;

    /// Get the current processor number
    #[link_name = "svcGetCurrentProcessorNumber"]
    pub fn get_current_processor_number() -> u32;

    /// Signal an event
    #[link_name = "svcSignalEvent"]
    pub fn signal_event(event: WEvent) -> Result;

    /// Clear an event
    #[link_name = "svcClearEvent"]
    pub fn clear_event(event: Event) -> Result;

    /// Map shared memory
    #[link_name = "svcMapSharedMemory"]
    pub fn map_shared_memory(
        block: SharedMemory,
        addr: *mut cty::c_void,
        size: u64,
        permission: u32,
    ) -> Result;

    /// Unmap shared memory
    #[link_name = "svcUnmapSharedMemory"]
    pub fn unmap_shared_memory(
        block: SharedMemory,
        addr: *mut cty::c_void,
        size: u64,
    ) -> Result;

    /// Create transfer memory
    #[link_name = "svcCreateTransferMemory"]
    pub fn create_transfer_memory(
        out: *mut TransferMemory,
        addr: *mut cty::c_void,
        size: u64,
        permission: u32,
    ) -> Result;

    /// Closes the specified handle
    #[link_name = "svcCloseHandle"]
    pub fn close_handle(handle: Handle) -> Result;

    /// Resets a signal. Takes a revent or process.
    #[link_name = "svcResetSignal"]
    pub fn reset_signal(signal: Handle) -> Result;

    /// Wait synchronization
    #[link_name = "svcWaitSynchronization"]
    pub fn wait_synchronization(
        handle_index: *mut u32,
        handles: *mut Handle,
        num_handles: u32,
        timeout: u64,
    ) -> Result;

    /// Cancel synchronization
    #[link_name = "svcCancelSynchronization"]
    pub fn cancel_synchronization(handle: Handle) -> Result;

    /// Arbitrate lock
    #[link_name = "svcArbitrateLock"]
    pub fn arbitrate_lock(
        current_thread: Thread,
        lock: *mut cty::c_void,
        requesting_thread: Thread,
    );

    /// Arbitrate unlock
    #[link_name = "svcArbitrateUnlock"]
    pub fn arbitrate_unlock(lock: *mut cty::c_void);

    /// Wait process wide key atomic
    #[link_name = "svcWaitProcessWideKeyAtomic"]
    pub fn wait_process_wide_key_atomic(
        ptr0: *mut cty::c_void,
        ptr1: *mut cty::c_void,
        thread: Thread,
        timeout: u64,
    ) -> Result;

    /// Wait process wide key atomic
    #[link_name = "svcSignalProcessWideKey"]
    pub fn signal_process_wide_key(ptr: *mut cty::c_void, value: u32) -> Result;

    /// Get the system time tick
    #[link_name = "svcGetSystemTick"]
    pub fn get_system_tick() -> u64;

    /// Connect to a named port
    #[link_name = "svcConnectToNamedPort"]
    pub fn connect_to_named_port(out: *mut Session, name: *const cty::c_char) -> Result;

    /// Send sync request
    #[link_name = "svcSendSyncRequest"]
    pub fn send_sync_request(session: Session) -> Result;

    /// Send sync request with user buffer
    #[link_name = "svcSendSyncRequestWithUserBuffer"]
    pub fn send_sync_request_with_user_buffer(
        buffer: *mut cty::c_void,
        size: u64,
        session: Session,
    ) -> Result;

    /// Get a process's ID. Takes a handle to a process or thread, and returns
    /// its process ID.
    #[link_name = "svcGetProcessId"]
    pub fn get_process_id(pid: *mut u32, thread_or_Processandle: Handle) -> Result;

    /// Get a thread's ID. Takes a handle to that thread, and returns its ID.
    #[link_name = "svcGetThreadId"]
    pub fn get_thread_id(handle_out: *mut Thread, handle_in: Thread) -> Result;

    /// Output a debug string
    #[link_name = "svcOutputDebugString"]
    pub fn output_debug_string(str: *mut cty::c_char, size: u64);

    /// Return from exception
    #[link_name = "svcReturnFromException"]
    pub fn return_from_exception(result: u64);

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
    #[link_name = "svcGetInfo"]
    pub fn get_info(info: *mut u64, info_id: u64, handle: Handle, info_sub_id: u64)
        -> Result;

    #[link_name = "svcCreateSession"]
    pub fn create_session(
        server: *mut Session,
        client: *mut Session,
        is_light: bool,
        unknown: u32,
    ) -> Result;

    /// Accept session
    #[link_name = "svcAcceptSession"]
    pub fn accept_session(out: *mut Session, port: Port) -> Result;

    /// Reply and recieve
    #[link_name = "svcReplyAndReceive"]
    pub fn reply_and_receive(
        handle_idx: *mut u32,
        handles: *mut Session,
        num_handles: u32,
        reply_session: Session,
        timeout: u64,
    ) -> Result;

    /// Reply and recieve with user buffer
    #[link_name = "svcReplyAndReceiveWithUserBuffer"]
    pub fn reply_and_receive_with_user_buffer(
        handle_idx: *mut u32,
        buffer: *mut cty::c_void,
        size: u64,
        handles: *mut Session,
        num_handles: u32,
        reply_session: Session,
        timeout: u64,
    ) -> Result;

    /// Read/Write register
    #[link_name = "svcReadWriteRegister"]
    pub fn read_write_register(
        out_value: *mut u32,
        addr: u64,
        rw_mask: u32,
        in_value: u32,
    ) -> Result;

    /// Create a block of shared memory
    #[link_name = "svcCreateSharedMemory"]
    pub fn create_shared_memory(
        out: *mut SharedMemory,
        size: u64,
        self_permissions: u32,
        foreign_permissions: u32,
    ) -> Result;

    /// Map transfer memory
    #[link_name = "svcMapTransferMemory"]
    pub fn map_transfer_memory(
        handle: TransferMemory,
        addr: *mut cty::c_void,
        size: u64,
        perm: u32,
    ) -> Result;

    /// Unmap transfer memory
    #[link_name = "svcUnmapTransferMemory"]
    pub fn unmap_transfer_memory(
        handle: TransferMemory,
        addr: *mut cty::c_void,
        size: u64,
    ) -> Result;

    /// Query IO mapping
    #[link_name = "svcQueryIoMapping"]
    pub fn query_io_mapping(virt_addr: *mut cty::c_void, phys_addr: u64, size: u64) -> Result;

    /// Attach device address space
    #[link_name = "svcAttachDeviceAddressSpace"]
    pub fn attach_device_address_space(device: u32, space: DevAddrSpace) -> Result;

    /// Detach device address space
    #[link_name = "svcDetachDeviceAddressSpace"]
    pub fn detach_device_address_space(device: u32, space: DevAddrSpace) -> Result;

    /// Map device address space by force
    #[link_name = "svcMapDeviceAddressSpaceByForce"]
    pub fn map_device_address_space_by_force(
        space: DevAddrSpace,
        process: Process,
        dev_addr: u64,
        dev_size: u64,
        map_addr: u64,
        perm: u32,
    ) -> Result;

    /// Map device address space aligned
    #[link_name = "svcMapDeviceAddressSpaceAligned"]
    pub fn map_device_address_space_aligned(
        space: DevAddrSpace,
        process: Process,
        dev_addr: u64,
        dev_size: u64,
        map_addr: u64,
        perm: u32,
    ) -> Result;

    /// Unmap device address space
    #[link_name = "svcUnmapDeviceAddressSpace"]
    pub fn unmap_device_address_space(
        space: DevAddrSpace,
        process: Process,
        map_addr: u64,
        map_size: u64,
        perm: u32,
    ) -> Result;

    /// Debug active process
    #[link_name = "svcDebugActiveProcess"]
    pub fn debug_active_process(out: *mut Debug, process_id: u64) -> Result;

    /// Query debug process memory
    #[link_name = "svcQueryDebugProcessMemory"]
    pub fn query_debug_process_memory(
        memory_info: *mut memory_info_t,
        page_info: *mut u32,
        debug: Debug,
        addr: u64,
    ) -> Result;

    /// Read debug process memory
    #[link_name = "svcReadDebugProcessMemory"]
    pub fn read_debug_process_memory(
        buffer: *mut cty::c_void,
        debug: Debug,
        addr: u64,
        size: u64,
    ) -> Result;

    /// Write debug process memory
    #[link_name = "svcWriteDebugProcessMemory"]
    pub fn write_debug_process_memory(
        debug: Debug,
        buffer: *mut cty::c_void,
        addr: u64,
        size: u64,
    ) -> Result;
}
