use ipc::{Request, IRequest, Response, IPCBuffer};
use kernel::svc::send_sync_request;
use kernel::{KObject};
use error::*;
use tls::TlsStruct;
use alloc::arc::Arc;
use arrayvec::Array;

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
    fn send<REQ: IRequest, Y: Clone>(&self, req: REQ) -> Result<Response<Y>>;
    fn from_res<Y: Clone>(res: &mut Response<Y>) -> Self;
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

        let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
            .ty(MessageType::Control)
            .args(());
        let mut res = self.send::<_, ()>(req)?;
        Ok(Session(res.pop_handle()))
    }

    pub fn send<REQ: IRequest, Y: Clone>(&self, req: REQ) -> Result<Response<Y>> {
        let mut ipc_buf = TlsStruct::borrow_ipc_mut();
        req.pack(&mut *ipc_buf, None);
        let err = unsafe { send_sync_request((self.0).0) };
        if err != 0 {
            return Err(Error(err));
        }
        Response::unpack(&mut ipc_buf[..], None)
    }

    pub fn to_domain(self) -> ::core::result::Result<Domain, (Session, Error)> {
        use ipc::{Request, MessageType};

        let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
            .ty(MessageType::Control)
            .args(());
        let res : Result<Response<u32>> = self.send(req);
        match res {
            Ok(res) => Ok(Domain(Arc::new(self.0), *res.get_raw())),
            Err(err) => Err((self, err))
        }
    }
}

impl Object for Session {
    // TODO: This is basically CMIF, instead of being a true low-level session.
    fn send<REQ: IRequest, Y: Clone>(&self, req: REQ) -> Result<Response<Y>> {
        Session::send(self, req)
    }

    fn from_res<Y: Clone>(res: &mut Response<Y>) -> Session {
        Session(res.pop_handle())
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

// TODO: Think about the safety implication of this huge footgun.
impl From<KObject> for Session {
    fn from(obj: KObject) -> Session {
        Session(obj)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Domain
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Domain(Arc<KObject>, u32);


impl Domain {
    pub fn new(ses: Arc<KObject>, domain_id: u32) -> Domain {
        Domain(ses, domain_id)
    }
    fn send<REQ: IRequest, Y: Clone>(&self, req: REQ) -> Result<Response<Y>> {
        let mut ipc_buf = TlsStruct::borrow_ipc_mut();
        req.pack(&mut *ipc_buf, Some(self.1));
        let err = unsafe { send_sync_request((self.0).0) };
        if err != 0 {
            return Err(Error(err));
        }
        Response::unpack(&mut ipc_buf[..], Some(self.0.clone()))
    }
}

// TODO: impl Drop for Domain

impl Object for Domain {
    fn send<REQ: IRequest, Y: Clone>(&self, req: REQ) -> Result<Response<Y>> {
        Domain::send(self, req)
    }

    fn from_res<Y: Clone>(res: &mut Response<Y>) -> Domain {
        res.pop_domain_object()
    }
}

//impl AsRef<KObject> for Domain {
//    fn as_ref(&self) -> &KObject {
//        &self.0
//    }
//}
//
//// TODO: Impl from instead
//impl Into<KObject> for Domain {
//    fn into(self) -> KObject {
//        self.0
//    }
//}

/*impl FromKObject for Domain {
    unsafe fn from_kobject(obj: KObject) -> Domain {
        Domain(obj)
    }
}*/
