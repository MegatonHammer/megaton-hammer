use ipc::{Request, Response};
use kernel::svc::send_sync_request;
use kernel::{FromKObject, KObject};
use error::*;
use tls::TlsStruct;

#[derive(Debug)]
pub struct Session(KObject);

impl Session {
    pub unsafe fn from_raw(handle: KObject) -> Session {
        Session(handle)
    }
    // TODO: This is basically CMIF, instead of being a true low-level session.
    pub fn send<T: Clone, Y: Clone>(&self, req: Request<T>) -> Result<Response<Y>> {
        let mut tls = TlsStruct::borrow_mut();
        let ipc_buf = &mut tls.ipc_buf;
        req.pack(ipc_buf);
        let err = unsafe { send_sync_request((self.0).0) };
        if err != 0 {
            return Err(Error(err));
        }
        Response::unpack(&mut ipc_buf[..])
    }
}

impl AsRef<KObject> for Session {
    fn as_ref(&self) -> &KObject {
        &self.0
    }
}

// TODO: Impl from instead
impl Into<KObject> for Session {
    fn into(self) -> KObject {
        self.0
    }
}

impl FromKObject for Session {
    unsafe fn from_kobject(obj: KObject) -> Session {
        Session(obj)
    }
}
