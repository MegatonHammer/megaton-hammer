use ipc::{Request, Response};
use kernel::svc::send_sync_request;
use kernel::{FromKObject, KObject};
use error::*;
use tls::TlsStruct;

////////////////////////////////////////////////////////////////////////////////
// Object
////////////////////////////////////////////////////////////////////////////////

// Trait sealing.
mod private {
    pub trait Sealed {}
    impl Sealed for super::Domain {}
    impl Sealed for super::Session {}
}

pub trait Object : private::Sealed {
    fn send<T: Clone, Y: Clone>(&self, req: Request<T>) -> Result<Response<Y>>;
}

////////////////////////////////////////////////////////////////////////////////
// Session
////////////////////////////////////////////////////////////////////////////////

// TODO: This currently represents either a Session or a Domain, out of
// convenience. That's... not a great idea. We should have separate types for
// each, and have an Object trait implemented by both.
#[derive(Debug)]
pub struct Session(KObject);

impl Session {
    pub unsafe fn from_raw(handle: KObject) -> Session {
        Session(handle)
    }

    pub fn duplicate(&self) -> Result<Session> {
        use ipc::{Request, MessageType};

        let req = Request::new(2)
            .ty(MessageType::Control)
            .args(());
        if let Err(err) = self.send::<(), ()>(req) {
            Err((self, err))
        } else {
            Ok(Session(self.0))
        }
    }

    pub fn send<T: Clone, Y: Clone>(&self, req: Request<T>) -> Result<Response<Y>> {
        let mut ipc_buf = TlsStruct::borrow_ipc_mut();
        req.pack(&mut *ipc_buf, false);
        let err = unsafe { send_sync_request((self.0).0) };
        if err != 0 {
            return Err(Error(err));
        }
        Response::unpack(&mut ipc_buf[..])
    }

    pub fn to_domain(self) -> ::core::result::Result<Domain, (Session, Error)> {
        use ipc::{Request, MessageType};

        let req = Request::new(0)
            .ty(MessageType::Control)
            .args(());
        if let Err(err) = self.send::<(), ()>(req) {
            Err((self, err))
        } else {
            Ok(Domain(self.0))
        }
    }
}

impl Object for Session {
    // TODO: This is basically CMIF, instead of being a true low-level session.
    fn send<T: Clone, Y: Clone>(&self, req: Request<T>) -> Result<Response<Y>> {
        Session::send(self, req)
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

////////////////////////////////////////////////////////////////////////////////
// Domain
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Domain(KObject);


impl Domain {
    pub unsafe fn from_raw(handle: KObject) -> Domain {
        Domain(handle)
    }

    pub fn send<T: Clone, Y: Clone>(&self, req: Request<T>) -> Result<Response<Y>> {
        let mut ipc_buf = TlsStruct::borrow_ipc_mut();
        req.pack(&mut *ipc_buf, true);
        let err = unsafe { send_sync_request((self.0).0) };
        if err != 0 {
            return Err(Error(err));
        }
        Response::unpack(&mut ipc_buf[..])
    }
}

impl Object for Domain {
    // TODO: This is basically CMIF, instead of being a true low-level Domain.
    fn send<T: Clone, Y: Clone>(&self, req: Request<T>) -> Result<Response<Y>> {
        Domain::send(self, req)
    }
}

impl AsRef<KObject> for Domain {
    fn as_ref(&self) -> &KObject {
        &self.0
    }
}

// TODO: Impl from instead
impl Into<KObject> for Domain {
    fn into(self) -> KObject {
        self.0
    }
}

impl FromKObject for Domain {
    unsafe fn from_kobject(obj: KObject) -> Domain {
        Domain(obj)
    }
}
