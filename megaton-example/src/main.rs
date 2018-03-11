#![no_std]

extern crate megaton_hammer;
extern crate megaton_ipc;

use core::fmt::Write;
use megaton_ipc::nn::socket::BsdBufferConfig;
use megaton_hammer::kernel::{svc, KObject};

static mut BSD_MEM: [u8; 0x234000] = [0; 0x234000];

#[derive(Debug)]
#[repr(C)]
struct SockAddrIn {
    sin_len: u8,
    sin_family: u8,
    sin_port: u16,
    sin_addr: u32,
    sin_zero: [u8; 8]
}

const AF_INET: u8 = 2;
const AF_INET6: u8 = 17;
const AF_ROUTE: u8 = 28;

fn main() {
    //writeln!(megaton_crt0::LOG.lock(), "We are rust, and we are in the main!").unwrap();

    let socket_ipc = megaton_ipc::nn::socket::sf::IClient::new().expect("Failed to get bsd service");

    //writeln!(megaton_crt0::SvcLog, "The socket is {:?}", socket_ipc).unwrap();

    let bsd_config = BsdBufferConfig {
        version: 1,
        tcp_tx_buf_size: 0x8000,
        tcp_rx_buf_size: 0x10000,
        tcp_tx_buf_max_size: 0x40000,
        tcp_rx_buf_max_size: 0x40000,
        udp_tx_buf_size: 0x2400,
        udp_rx_buf_size: 0xA500,
        sb_efficiency: 4,
    };

    let mut mem_handle = 0;
    let r = unsafe { svc::create_transfer_memory(&mut mem_handle, BSD_MEM.as_mut_ptr() as _, core::mem::size_of_val(&BSD_MEM) as u64, 0) };
    // TODO: Really need to turn the SVCs into a proper API...
    if r != 0 {
        //writeln!(megaton_crt0::LOG.lock(), "Failed to create transfer memory: {}", r);
        return;
    }
    let mem_handle = unsafe { KObject::new(mem_handle) };
    //writeln!(megaton_crt0::LOG.lock(), "Initializing {:?} - {:?}", socket_ipc, mem_handle);
    socket_ipc.initialize(bsd_config, 0, unsafe { core::mem::size_of_val(&BSD_MEM) as u64 }, &mem_handle).expect("Failed to initialize bsd");


    let (socket, bsd_errno) = socket_ipc.socket(AF_INET as u32, 1, 6).expect("Failed to create Socket");
    if socket == -1 {
        //writeln!(megaton_crt0::LOG.lock(), "Failed to create Socket: {}", bsd_errno);
        //writeln!(megaton_crt0::SvcLog, "Failed to create Socket: {}", bsd_errno);
        return;
    }

    let sockaddrin = SockAddrIn {
        sin_len: core::mem::size_of::<SockAddrIn>() as u8,
        sin_family: AF_INET,
        sin_port: 2991u16.to_be(),
        sin_addr: (91 << 24 | 121 << 16 | 81 << 8 | 160u32).to_be(),
        sin_zero: [0; 8]
    };

    let sockaddr = unsafe { core::mem::transmute(sockaddrin) };

    let (ret, bsd_errno) = socket_ipc.connect(socket as u32, &sockaddr).expect("Failed to connect");
    if ret == -1 {
        //writeln!(megaton_crt0::LOG.lock(), "Failed to connect: {}", bsd_errno);
        //writeln!(megaton_crt0::SvcLog, "Failed to connect: {}", bsd_errno);
        return;
    }

    // Transmute ? Really ?
    let (ret, bsd_errno) = socket_ipc.write(socket as u32, unsafe { core::mem::transmute("This is a simple socket test from rust".as_bytes()) } ).expect("Failed to write");
    if ret == -1 {
        //writeln!(megaton_crt0::SvcLog, "Failed to write: {}", bsd_errno);
    }
    //writeln!(megaton_crt0::LOG.lock(), "I'm done !");
}

/*fn bsd_Get_transfer_mem_size(config: &BsdBufferConfig) -> usize {
    let tcp_tx_buf_max_size = if config.tcp_tx_buf_max_size != 0 { config.tcp_tx_buf_max_size } else { config.tcp_tx_buf_size };
    let tcp_rx_buf_max_size = if config.tcp_rx_buf_max_size != 0 { config.tcp_rx_buf_max_size } else { config.tcp_rx_buf_size };
    let sum = tcp_tx_buf_max_size + tcp_rx_buf_max_size + config.udp_tx_buf_size + config.udp_rx_buf_size;

    let sum = ((sum + 0xFFF) >> 12) << 12; // Page round-up
    return config.sb_efficiency * sum;
}*/
