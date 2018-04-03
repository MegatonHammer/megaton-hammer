//#![no_std]
#![feature(termination_trait)]

extern crate megaton_hammer;
extern crate megaton_ipc;
extern crate byteorder;
extern crate image;
extern crate math;

use byteorder::{ReadBytesExt, WriteBytesExt, LE, ByteOrder};
use megaton_hammer::kernel::{TransferMemory, KObject, svc};
use megaton_ipc::{nn, nns};
use std::io::{Seek, SeekFrom, Cursor};
use image::png::PNGDecoder;
use image::{Pixel, ImageDecoder};

// TODO: This kind of sucks. And is only a problem because my IPC bindings don't
// have a concept of strings yet. We need to fix this.
pub fn u8_slice_to_i8_slice(slice: &[u8]) -> &[i8] {
    unsafe { &*(slice as *const _  as *const [i8]) }
}

#[derive(Debug)]
enum MyError {
    MegatonError(megaton_hammer::error::Error),
    ImageError(image::ImageError),
}

impl From<image::ImageError> for MyError {
    fn from(err: image::ImageError) -> MyError {
        MyError::ImageError(err)
    }
}

impl From<megaton_hammer::error::Error> for MyError {
    fn from(err: megaton_hammer::error::Error) -> MyError {
        MyError::MegatonError(err)
    }
}

fn main() -> std::result::Result<(), MyError> {
    // Let's get ferris to show up on my switch.

    // Init the nv stuff
    println!("Create nv");
    let nvdrv = nns::nvdrv::INvDrvServices::new_nvdrv_a()?;

    // Initialize nvdrv
    println!("Create transfer memory");
    let transfer_mem = TransferMemory::new(0x30000)?;
    let temporary_process = unsafe { KObject::new(megaton_hammer::kernel::svc::CURRENT_PROCESS) };
    println!("Initialize nv");
    nvdrv.initialize(0x30000, &temporary_process, transfer_mem.as_ref())?;
    std::mem::drop(temporary_process);

    println!("Open /dev/nvhost-as-gpu");
    let (nvasgpu, err) = nvdrv.open(u8_slice_to_i8_slice(&b"/dev/nvhost-as-gpu"[..]))?;
    if err != 0 {
        panic!("Failed to open");
    }
    println!("Open /dev/nvmap");
    let (nvmap, err) = nvdrv.open(u8_slice_to_i8_slice(&b"/dev/nvmap"[..]))?;
    if err != 0 {
        panic!("Failed to open");
    }
    // Init the vi stuff.
    println!("new IManagerRootService");
    let vi_m = nn::visrv::sf::IManagerRootService::new()?;
    println!("get_display_service");
    let disp_svc = vi_m.get_display_service(1)?;
    println!("get_relay_service");
    let relay_svc = disp_svc.get_relay_service()?;
    println!("get_system_display_service");
    let system_disp_svc = disp_svc.get_system_display_service()?;
    println!("get_manager_display_service");
    let manager_disp_svc = disp_svc.get_manager_display_service()?;

    println!("Open display");
    let display_id = {
        let mut display = [0u8; 64];
        display[..b"Default".len()].copy_from_slice(b"Default");
        disp_svc.open_display(display)?
    };

    println!("Open a layer");
    let layer_id = manager_disp_svc.create_managed_layer(1, display_id, 0)?;
    let mut parcel = [0u8; 0x100];
    let binder_id = {
        let mut display = [0u8; 64];
        display[..b"Default".len()].copy_from_slice(b"Default");
        disp_svc.open_layer(display, layer_id, 0, &mut parcel)?
    };

    // Connect to the IGBP. Take a look at the following link for reference.
    // https://android.googlesource.com/platform/frameworks/native/+/e2786ea5aec3a12d948feb85ffbb535fc89c0fe6/libs/gui/IGraphicBufferProducer.cpp#297
    println!("Connect to the IGBP");
    let queue_buffer_output = {
        let mut parcel = OwnedParcel::new();
        parcel.write_interface_token("android.gui.IGraphicBufferProducer");
        parcel.write_u32(0); // IProducerListener is null because we don't support it in MegatonHammer (nor in libt) yet.
        parcel.write_u32(2); // API
        parcel.write_u32(0); // ProducerControlledByApp.
        let mut parcel_out = RawParcel::default();
        relay_svc.transact_parcel(binder_id as i32, CONNECT, 0, parcel.build().as_bytes(), parcel_out.as_bytes_mut())?;

        let mut reader = parcel_out.into_parcel_reader();
        let qbo = QueueBufferOutput::from_parcel(&mut reader);
        //if reader.read_u32() != 0 {
        //    panic!("Failed to connect to igbp");
        //}
        qbo
    };

    println!("Allocate framebuffers");
    let mut mem : Vec<BufferMemory> = Vec::with_capacity(3);
    unsafe { mem.set_len(3); }
    // Disables caching when talking to the gpu.
    if unsafe { svc::set_memory_attribute(mem.as_mut_ptr() as _, mem.len() * std::mem::size_of::<BufferMemory>(), 0x8, 0x8) } != 0 {
        panic!("Failed to set memory attribute");
    }

    let gpu_buffer = {
        let mut create = NvMapIocCreateArgs {
            size: (mem.len() * std::mem::size_of::<BufferMemory>()) as u32,
            handle: 0
        };
        let ret = nvdrv.ioctl(nvmap, NVMAP_IOC_CREATE, 0,
                    // TODO: This is unsafe. And illegal. Rust assumes aliasing
                    // doesn't happen with references, which is exactly what we're
                    // doing. In theory, because we never *read* the content of
                    // those, I believe this is, erm, "mostly OK" ? But I should
                    // find a better way to deal with it.
                    unsafe { std::slice::from_raw_parts(&create as *const NvMapIocCreateArgs as *const u8, std::mem::size_of::<NvMapIocCreateArgs>()) },
                    unsafe { std::slice::from_raw_parts_mut(&mut create as *mut NvMapIocCreateArgs as *mut u8, std::mem::size_of::<NvMapIocCreateArgs>()) })?;
        if ret != 0 {
            panic!("Ioctl failed");
        }
        GpuBuffer {
            nvmap_handle: create.handle,
            size: mem.len() * std::mem::size_of::<BufferMemory>(),
            alignment: 0x1000,
            kind: 0
        }
    };

    let buffers = {
        let mut alloc = NvMapIocAllocArgs {
            handle: gpu_buffer.nvmap_handle,
            heapmask: 0,
            flags: 0,
            align: gpu_buffer.alignment,
            kind: gpu_buffer.kind,
            pad: [0; 7],
            addr: mem.as_mut_ptr() as u64
        };

        let ret = nvdrv.ioctl(nvmap, NVMAP_IOC_ALLOC, 0,
                    // TODO: This is unsafe. And illegal. Rust assumes aliasing
                    // doesn't happen with references, which is exactly what we're
                    // doing. In theory, because we never *read* the content of
                    // those, I believe this is, erm, "mostly OK" ? But I should
                    // find a better way to deal with it.
                    unsafe { std::slice::from_raw_parts(&alloc as *const NvMapIocAllocArgs as *const u8, std::mem::size_of::<NvMapIocAllocArgs>()) },
                    unsafe { std::slice::from_raw_parts_mut(&mut alloc as *mut NvMapIocAllocArgs as *mut u8, std::mem::size_of::<NvMapIocAllocArgs>()) })?;

        if ret != 0 {
            panic!("Ioctl alloc failed");
        }

        let mut buffers = Vec::with_capacity(3);

        for i in 0..3 {
            buffers.push(GraphicBuffer {
                width: queue_buffer_output.width,
                height: queue_buffer_output.height,
                stride: queue_buffer_output.width,
                format: 1, // RGBA_8888
                usage: 0xb00, // TODO: Wat?
                gpu_buffer: &gpu_buffer,
                index: i,
                offset_gpu_buffer: 0x3c0000 * i
            });
        }
        buffers
    };

    println!("Tell IGBP about the buffers");
    for (i, buf) in buffers.iter().enumerate() {
        let mut parcel = OwnedParcel::new();
        parcel.write_interface_token("android.gui.IGraphicBufferProducer");
        parcel.write_u32(i as u32); // slot
        parcel.write_u32(1); // Unknown
        parcel.write_u32(0x16c); // Flattened GraphicsBuffer
        parcel.write_u32(0); // Unknown
        buf.write_to_parcel(&mut parcel);
        let mut parcel_out = RawParcel::default();
        relay_svc.transact_parcel(binder_id as i32, SET_PREALLOCATED_BUFFER, 0, parcel.build().as_bytes(), parcel_out.as_bytes_mut())?;
    }

    // TODO: Set scaling mode
    // TODO: Add layer to stack
    // TODO: Set layer z
    // TODO: Get vsync event
    println!("Dequeue buffer");
    let slot = {
        let mut parcel = OwnedParcel::new();
        parcel.write_interface_token("android.gui.IGraphicBufferProducer");
        parcel.write_u32(1); // Pixel format
        parcel.write_u32(1280); // width
        parcel.write_u32(720); // height
        parcel.write_u32(0); // get_frame_timestamp
        parcel.write_u32(0xb00); // usage
        let mut parcel_out = RawParcel::default();
        relay_svc.transact_parcel(binder_id as i32, DEQUEUE_BUFFER, 0, parcel.build().as_bytes(), parcel_out.as_bytes_mut())?;
        let mut parcel_out = parcel_out.into_parcel_reader();
        let slot = parcel_out.read_u32();
        // Read fence
        parcel_out.0.seek(SeekFrom::Current(44));
        let status = parcel_out.read_u32();
        if status != 0 {
            //panic!("WTF");
        }
        slot
    };

    // Request buffer if it hasn't been requested already.
    println!("Request buffer");
    {
        let mut parcel = OwnedParcel::new();
        parcel.write_interface_token("android.gui.IGraphicBufferProducer");
        parcel.write_u32(slot); // Slot
        let mut parcel_out = RawParcel::default();
        let res = relay_svc.transact_parcel(binder_id as i32, REQUEST_BUFFER, 0, parcel.build().as_bytes(), parcel_out.as_bytes_mut())?;
        let mut parcel_out = parcel_out.into_parcel_reader();
        let non_null = parcel_out.read_u32() != 0;
        if non_null {
            //let len = parcel_out.read_u32();
            //if len != 0x16c {
            //    panic!("Invalid length");
            //}
            //let unk = parcel_out.read_u32();
            // TODO: Get graphicbuffer.
        }
        let status = parcel_out.read_u32();
        if status != 0 {
            //panic!("WTF");
        }
    }

    println!("Loading image from FERRIS");
    let image = PNGDecoder::new(Cursor::new(&FERRIS[..]));
    println!("Getting a frame");
    let frame = image.into_frames()?.next().unwrap().into_buffer();

    //println!("Resizing FERRIS");
    //let frame = image::imageops::resize(&image.into_frames()?.next()?.into_buffer(), 1280, 760, image::FilterType::Lanczos3);


    // Blit
    println!("Blit");
    {
        fn pdep(mask: u32, mut value: u32) -> u32 {
            let mut out = 0;
            for shift in 0..32 {
                let bit = 1 << shift;
                if mask & bit != 0 {
                    if value & 1 != 0 {
                        out |= bit
                    }
                    value >>= 1;
                }
            }
            out
        }
        fn swizzle_x(v: u32) -> u32 { pdep(!0x7B4, v) }
        fn swizzle_y(v: u32) -> u32 { pdep(0x7B4, v) }
        let x0 = 0;
        let y0 = 0;
        let mut offs_x0 = swizzle_x(x0);
        let mut offs_y = swizzle_y(y0);
        let x_mask = swizzle_x(!0);
        let y_mask = swizzle_y(!0);
        let incr_y = swizzle_x(128 * 10);
        let tile_height = 128;

        offs_x0 += incr_y * (y0 / tile_height);

        // TODO: Add clipping.
        for y in 0..frame.height() {
            let mut offs_x = offs_x0;
            for x in 0..frame.width() {
                let pixel = frame.get_pixel(x, y);
                mem[slot as usize][offs_y as usize + offs_x as usize] = LE::read_u32(pixel.channels());
                offs_x = (offs_x - x_mask) & x_mask;
            }
            offs_y = (offs_y - y_mask) & y_mask;
            if offs_y == 0 {
                offs_x0 += incr_y;
            }
        }
    }

    // Enqueue buffer
    println!("Enqueue buffer");
    {
        let mut parcel = OwnedParcel::new();
        parcel.write_interface_token("android.gui.IGraphicBufferProducer");
        parcel.write_u32(slot); // Slot
        parcel.write_u32(0x54); parcel.write_u32(0); // unknown, but always those values
        parcel.write_u32(0x588bbba9); parcel.write_u32(0); // Timestamp, u64
        parcel.write_u32(1); // unknown, but always those values
        parcel.write_u32(0);
        parcel.write_u32(0);

        parcel.write_u32(0); // sometimes zero
        parcel.write_u32(0);

        parcel.write_u32(0);

        parcel.write_u32(0); // Also seen 2

        parcel.write_u32(0);
        parcel.write_u32(0);

        parcel.write_u32(1); // fence?
        parcel.write_u32(1);
        parcel.write_u32(0xa3);
        parcel.write_u32(0);
        parcel.write_u32(-1i32 as u32);
        parcel.write_u32(0);
        parcel.write_u32(-1i32 as u32);
        parcel.write_u32(0);
        parcel.write_u32(-1i32 as u32);
        parcel.write_u32(0);

        let mut parcel_out = RawParcel::default();
        let res = relay_svc.transact_parcel(binder_id as i32, QUEUE_BUFFER, 0, parcel.build().as_bytes(), parcel_out.as_bytes_mut())?;
        let mut parcel_out = parcel_out.into_parcel_reader();

        let _ = QueueBufferOutput::from_parcel(&mut parcel_out);

        let status = parcel_out.read_u32();
        if status != 0 {
            panic!("WTF");
        }
    }

    // TODO: Wait sync
    // TODO: Reset event
    loop {}
}

//static FERRIS : &'static [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

static FERRIS: &'static [u8; 33061] = include_bytes!("../img/ferris.png");
// Graphic buffer stuff

//struct IGraphicBufferProducer(Arc<IHOSBinderDriver>, u32);

//impl IGraphicBufferProducer {
//    pub fn dequeue_buffer(&self) {
//
//    }
//}

// Returned by igbp_connect
#[repr(C)]
struct QueueBufferOutput {
    width: u32,
    height: u32,
    transform_hint: u32,
    num_pending_buffers: u32
}

impl QueueBufferOutput {
    fn from_parcel(parcel: &mut ParcelReader) -> QueueBufferOutput {
        let width = parcel.read_u32();
        let height = parcel.read_u32();
        let transform_hint = parcel.read_u32();
        let num_pending_buffers = parcel.read_u32();
        QueueBufferOutput {
            width, height, transform_hint, num_pending_buffers
        }
    }
}

#[repr(C)]
struct GraphicBuffer<'a> {
    width: u32,
    height: u32,
    stride: u32,
    format: u32,
    usage: u32,
    gpu_buffer: &'a GpuBuffer,
    index: u32,
    offset_gpu_buffer: u32,
}

impl<'a> GraphicBuffer<'a> {
    fn write_to_parcel(&self, parcel: &mut OwnedParcel) {

    }
}

#[repr(C)]
struct GpuBuffer {
    nvmap_handle: u32,
    size: usize,
    alignment: u32,
    kind: u8
}

// nvmap stuff

#[repr(C, align(4096))]
struct BufferMemory([u32; 0x3c0000/4]);

impl std::ops::Deref for BufferMemory {
    type Target = [u32];
    fn deref(&self) -> &[u32] {
        &self.0[..]
    }
}

impl std::ops::DerefMut for BufferMemory {
    fn deref_mut(&mut self) -> &mut [u32] {
        &mut self.0[..]
    }
}

const NVMAP_IOC_CREATE: u32 = 0xC0080101;
const NVMAP_IOC_FROM_ID: u32 = 0xC0080103;
const NVMAP_IOC_ALLOC: u32 = 0xC0200104;
const NVMAP_IOC_FREE: u32 = 0xC0180105;
const NVMAP_IOC_PARAM: u32 = 0xC00C0109;
const NVMAP_IOC_GET_ID: u32 = 0xC008010E;

#[repr(C)]
struct NvMapIocCreateArgs{
    /// In, size of the buffer in bytes
    size: u32,
    /// Out, handle to use for other operations
    handle: u32
}

#[repr(C)]
struct NvMapIocAllocArgs {
    handle: u32,
    heapmask: u32,
    /// (0=read-only, 1=read-write)
    flags: u32,
    align: u32,
    kind: u8,
    pad: [u8; 7],
    addr: u64,
}

// vi stuff. I should reuse some code from rust-binder, instead of rolling my
// own again.

const REQUEST_BUFFER: u32 = 0x1;
const SET_BUFFER_COUNT: u32 = 0x2;
const DEQUEUE_BUFFER: u32 = 0x3;
const DETACH_BUFFER: u32 = 0x4;
const DETACH_NEXT_BUFFER: u32 = 0x5;
const ATTACH_BUFFER: u32 = 0x6;
const QUEUE_BUFFER: u32 = 0x7;
const CANCEL_BUFFER: u32 = 0x8;
const QUERY: u32 = 0x9;
const CONNECT: u32 = 0xA;
const DISCONNECT: u32 = 0xB;
// 0xC might be SET_SIDEBAND_STREAM but I'm not sure
const ALLOCATE_BUFFERS: u32 = 0xD;
const SET_PREALLOCATED_BUFFER: u32 = 0xE;

#[derive(Debug)]
struct OwnedParcel(Vec<u8>);

impl OwnedParcel {
    pub fn new() -> OwnedParcel {
        OwnedParcel(Vec::new())
    }
    pub fn write_u32(&mut self, data: u32) {
        self.0.write_u32::<LE>(data).unwrap();
    }
    pub fn write_string16(&mut self, s: &str) {
        self.write_u32((s.encode_utf16().count() * 2) as u32);
        for c in s.encode_utf16() {
            self.0.write_u16::<LE>(c).unwrap();
        }
    }
    pub fn write_interface_token(&mut self, token: &str) {
        self.write_u32(0x100);
        self.write_string16(token);
    }

    pub fn build(self) -> RawParcel {
        let mut parcel = RawParcel {
            data_size: self.0.len() as u32,
            data_offset: 0x10,
            objects_size: 0,
            objects_offset: 0,
            payload: [0; 0x200]
        };
        parcel.payload[..self.0.len()].copy_from_slice(&self.0[..]);
        parcel
    }
}

struct ParcelReader(std::io::Cursor<Vec<u8>>);

impl ParcelReader {
    pub fn read_u32(&mut self) -> u32 {
        self.0.read_u32::<LE>().unwrap()
    }
}

#[repr(C)]
struct RawParcel {
    data_size: u32,
    data_offset: u32,
    objects_size: u32,
    objects_offset: u32,
    payload: [u8; 0x200]
}

impl Default for RawParcel {
    fn default() -> RawParcel {
        RawParcel {
            data_size: 0x200,
            data_offset: 0x10,
            objects_size: 0,
            objects_offset: 0,
            payload: [0; 0x200]
        }
    }
}

impl RawParcel {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const RawParcel as *const u8, std::mem::size_of::<RawParcel>()) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut RawParcel as *mut u8, std::mem::size_of::<RawParcel>()) }
    }

    pub fn into_parcel_reader(self) -> ParcelReader {
        ParcelReader(std::io::Cursor::new(Vec::from(&self.payload[..])))
    }
}
