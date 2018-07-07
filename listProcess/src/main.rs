extern crate megaton_hammer;

use megaton_hammer::kernel::{TransferMemory, KObject, FromKObject, Event, svc};
use megaton_hammer::ipcdefs as megaton_ipc;
use megaton_ipc::{nn, nns};
use std::io::{Write, Seek, SeekFrom, Cursor};

#[derive(Debug)]
enum MyError {
    MegatonError(megaton_hammer::error::Error),
    IoError(std::io::Error)
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::IoError(err)
    }
}

impl From<megaton_hammer::error::Error> for MyError {
    fn from(err: megaton_hammer::error::Error) -> MyError {
        MyError::MegatonError(err)
    }
}

fn main() -> std::result::Result<(), MyError> {
    use megaton_ipc::nn::pm::detail::IInformationInterface;

    // Sleep 2 seconds, for sm to be ready
    //unsafe { svc::sleep_thread(2_000_000_000); }

    //let mut stream = std::net::TcpStream::connect("roblab.la:2991")?;
    //let mut stream = std::net::TcpStream::connect((std::net::Ipv4Addr::new(91, 121, 81, 160), 2991))?;

    let mut stream = std::net::TcpStream::connect((std::net::Ipv4Addr::new(91, 121, 81, 160), 2991))?;

    writeln!(stream, "Helloooo");
    let mut processes = [0; 64];
    //let (res, num) = (0, 0);
    let num = unsafe { svc::get_process_list(&mut processes as *mut _ as *mut u64, 64)? };

    let pm_info = IInformationInterface::new()?;
    for i in 0..num {
        writeln!(stream, "{:?}", pm_info.get_title_id(processes[i]));
    }

    Ok(())
}
