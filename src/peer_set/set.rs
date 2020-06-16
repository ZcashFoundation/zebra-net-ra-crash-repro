use std::hash::Hash;
use std::{fmt::Debug, future::Future, pin::Pin};

pub trait Discover {
    type Key: Hash + Eq;
}

pub trait Service<Request> {
    type Error;
    type Future: Future<Output = Result<Response, Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;
}

use crate::Error;
pub struct Request;
pub struct Response;

pub struct PeerSet<D>
where
    D: Discover,
{
    _discover: D,
}

impl<D> Service<Request> for PeerSet<D>
where
    D: Discover + Unpin,
    D::Key: Clone + Debug + ToString,
{
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Response, Self::Error>> + Send + 'static>>;

    fn call(&mut self, _req: Request) -> Self::Future {
        todo!()
    }
}
