use ipc::{Request, Response};
use kernel::svc::send_sync_request;
use kernel::{FromKObject, KObject};
use error::*;
use tls::TlsStruct;

pub struct Session(KObject);

impl Session {
    pub unsafe fn from_raw(handle: KObject) -> Session {
        Session(handle)
    }
    // TODO: This is basically CMIF, instead of being a true low-level session.
    pub fn send<T: Clone, Y: Clone>(&self, req: Request<T>) -> Result<Response<Y>> {
        let tls = TlsStruct::borrow_mut();
        let mut ipc_buf = tls.ipc_buf;
        req.pack(&mut ipc_buf[..]);
        let err = unsafe { send_sync_request((self.0).0) };
        if err != 0 {
            return Err(Error(err));
        }
        Response::unpack(&mut ipc_buf[..])
    }
}

impl FromKObject for Session {
    unsafe fn from_kobject(obj: KObject) -> Session {
        Session(obj)
    }
}
