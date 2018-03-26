//#![no_std]

extern crate megaton_hammer;
extern crate megaton_ipc;
extern crate byteorder;

use byteorder::{WriteBytesExt, NativeEndian};
use megaton_hammer::kernel::{TransferMemory, KObject, Domain};
use megaton_ipc::{nn, nns};
use std::sync::Arc;

// TODO: This kind of sucks. And is only a problem because my IPC bindings don't
// have a concept of strings yet. We need to fix this.
pub fn u8_slice_to_i8_slice(slice: &[u8]) -> &[i8] {
    unsafe { &*(slice as *const _  as *const [i8]) }
}

fn main() {
    // Let's get ferris to show up on my switch.

    // Init the nv stuff
    println!("Create nv");
    let nvdrv = nns::nvdrv::INvDrvServices::new_nvdrv_a().unwrap();

    // Initialize nvdrv
    println!("Create transfer memory");
    let transfer_mem = TransferMemory::new(0x30000).unwrap();
    let temporary_process = unsafe { KObject::new(megaton_hammer::kernel::svc::CURRENT_PROCESS) };
    println!("Initialize nv");
    nvdrv.initialize(0x30000, &temporary_process, transfer_mem.as_ref()).unwrap();
    std::mem::drop(temporary_process);

    println!("Open /dev/nvhost-as-gpu");
    let nvasgpu = nvdrv.open(u8_slice_to_i8_slice(&b"/dev/nvhost-as-gpu"[..])).unwrap();
    println!("Open /dev/nvmap");
    let nvmap = nvdrv.open(u8_slice_to_i8_slice(&b"/dev/nvmap"[..])).unwrap();

    // Init the vi stuff.
    println!("new IManagerRootService");
    let vi_m = nn::visrv::sf::IManagerRootService::new().unwrap();
    //println!("Duplicate IManagerRootService");
    //let vi_m = Arc::try_unwrap(vi_m).unwrap();
    //println!("ToDomain IManagerRootService");
    //let vi_m = vi_m.to_domain().unwrap();
    println!("get_display_service");
    let disp_svc = vi_m.get_display_service(1).unwrap();
    println!("get_relay_service");
    let relay_svc = disp_svc.get_relay_service().unwrap();
    println!("get_system_display_service");
    let system_disp_svc = disp_svc.get_system_display_service().unwrap();
    println!("get_manager_display_service");
    let manager_disp_svc = disp_svc.get_manager_display_service().unwrap();

    // Open display
    let display_id = {
        let mut display = [0u8; 64];
        display[..b"Default".len()].copy_from_slice(b"Default");
        disp_svc.open_display(display).unwrap()
    };

    // Open a layer
    let layer_id = manager_disp_svc.create_managed_layer(1, display_id, 0).unwrap();
    let mut parcel = [0u8; 0x100];
    let binder = {
        let mut display = [0u8; 64];
        display[..b"Default".len()].copy_from_slice(b"Default");
        disp_svc.open_layer(display, layer_id, 0, &mut parcel);
    };

    println!("So, erm, does this work? I'm in rust btw")
}

/*struct IGraphicBufferProducer(Arc<nns::hosbinder::IHOSBinderDriver>);


impl IGraphicBufferProducer {
    pub fn connect(binder: Arc<nns::hosbinder::IHOSBinderDriver>, api: u32, producer_controlled_by_app: bool) -> (u32, IGraphicBufferProducer) {
        let parcel = WriteParcel::new();
        parcel.write_interface_token("android.gui.IGraphicBufferProducer");
        parcel.write_u32(0);
        parcel.write_u32(api);
        parcel.write_u32(if producer_controlled_by_app { 1 } else { 0 });

        // TODO: Implement binder
        //binder.
    }
}

struct ReadParcel<'a>(&'a [u8]);

impl<'a> ReadParcel<'a> {
    fn new(data: &[u8]) -> ReadParcel {
        ReadParcel(data)
    }

    fn read_binder() -> IGraphicBufferProducer {
        
    }
}

struct WriteParcel(Vec<u8>);

impl WriteParcel {
    pub fn new() -> WriteParcel {
        WriteParcel(Vec::new())
    }

    pub fn write_interface_token(&mut self, token: &str) {
        self.write_u32(0x100); // TODO: Is this the strict-mode policy?
        self.write_string16(token);
    }

    pub fn write_string16(&mut self, s: &str) {
        // TODO: Surely there's an easier way.
        struct Iter([u8; 2], usize);
        impl Iterator for Iter {
            type Item = u8;
            fn next(&mut self) -> Option<u8> {
                if self.1 >= 2 {
                    None
                } else {
                    let ret = Some(self.0[self.1]);
                    self.1 += 1;
                    ret
                }
            }
        }
        self.write_u32(s.len());
        self.0.extend(self.s.encode_utf16().flat_map(|n| {
            let x = [0; 2];
            NativeEndian::write_u16(&x[..]);
            Iter(x, 0)
        }));
    }

    pub fn write_u32(&mut self, val: u32) {
        self.0.write_u32::<NativeEndian>(val).expect("Write bigger than usize");
    }
}*/
