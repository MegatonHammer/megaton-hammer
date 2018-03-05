//! This will go into a separate crate, it's just to test my stuff for now.

use kernel::session::Session;
use ipc::ll::{Request, Response};
use error::*;

pub struct RoInterface(Session);

impl RoInterface {
    unsafe fn from_session(sess: Session) -> RoInterface {
        RoInterface(sess)
    }
    pub fn load_nro(&self, unk: u64, nro_base: u64, nro_size: u64, bss_base: u64, bss_size: u64) -> Result<usize> {
        //let old = std::mem::replace(&mut self.0, None).expect("Multithreaded access happened. This should never have happened...");
        let req = Request::new(0)
            .send_pid()
            .args((unk, nro_base, nro_size, bss_base, bss_size));
        let res : Response<u64> = self.0.send(req)?;
        Ok(0)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        // Make sure I can't do shit.
        let sess = unsafe { Session::from_raw(0) };
        let iface = unsafe { RoInterface::from_session(sess) };
        println!("{:?}", iface.load_nro(0x4050405040504050, 0xF0F0F0F0F0F0F0F0, 0, 0xFFFFFFFFFFFFFFFF, 0x7F7F7F7F7F7F7F7F));
    }
}
