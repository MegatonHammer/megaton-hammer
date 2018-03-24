use ipc::{Request, Response};
use kernel::svc::send_sync_request;
use kernel::{FromKObject, KObject};
use error::*;
use tls::TlsStruct;
use spin::RwLock;

// TODO: Create a type-level Session/Domain API.
/// A Kernel IPC session. Can be either a domain or a normal session.
#[derive(Debug)]
pub struct Session(RwLock<(KObject, bool)>);

impl Session {
    pub unsafe fn from_raw(handle: KObject) -> Session {
        Session(RwLock::new((handle, false)))
    }
    // TODO: This is basically CMIF, instead of being a true low-level session.
    pub fn send<T: Clone, Y: Clone>(&self, req: Request<T>) -> Result<Response<Y>> {
        let lock = self.0.read();
        let mut ipc_buf = TlsStruct::borrow_ipc_mut();
        req.pack(&mut *ipc_buf, lock.1);
        let err = unsafe { send_sync_request((lock.0).0) };
        if err != 0 {
            return Err(Error(err));
        }
        Response::unpack(&mut ipc_buf[..])
    }

    pub fn to_domain(&self) -> Result<()> {
        use ipc::{Request, MessageType};

        let mut lock = self.0.write();
        let req = Request::new(0)
            .ty(MessageType::Control)
            .args(());

        // TODO: Don't copy paste send method.
        let mut ipc_buf = TlsStruct::borrow_ipc_mut();
        req.pack(&mut *ipc_buf, lock.1);
        let err = unsafe { send_sync_request((lock.0).0) };
        if err != 0 {
            return Err(Error(err));
        }
        let _res : Response<()> = Response::unpack(&mut ipc_buf[..])?;
        lock.1 = true;
        Ok(())
    }
}

impl AsRef<KObject> for Session {
    fn as_ref(&self) -> &KObject {
        &self.0.read().0
    }
}

// TODO: Impl from instead
impl Into<KObject> for Session {
    fn into(self) -> KObject {
        self.0.into_inner().0
    }
}

impl FromKObject for Session {
    unsafe fn from_kobject(obj: KObject) -> Session {
        Session(RwLock::new((obj, false)))
    }
}
