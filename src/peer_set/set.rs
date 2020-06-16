use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{discover::Discover, Service};
use tower_load::Load;

use crate::BoxedStdError;

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
    D::Service: Service<Request, Response = Response> + Load,
    D::Error: Into<BoxedStdError>,
    <D::Service as Service<Request>>::Error: Into<BoxedStdError> + 'static,
    <D::Service as Service<Request>>::Future: Send + 'static,
    <D::Service as Load>::Metric: Debug,
{
    type Response = Response;
    type Error = BoxedStdError;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, _req: Request) -> Self::Future {
        todo!()
    }
}
