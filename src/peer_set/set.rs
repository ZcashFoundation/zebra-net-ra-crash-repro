use std::hash::Hash;
use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

// pub trait Load {
//     /// A comparable load metric. Lesser values are "preferable" to greater values.
//     type Metric: PartialOrd;

//     /// Obtains a service's load.
//     fn load(&self) -> Self::Metric;
// }

/// A change in the service set
#[derive(Debug)]
pub enum Change<K, V> {
    /// A new service identified by key `K` was identified.
    Insert(K, V),
    /// The service identified by key `K` disappeared.
    Remove(K),
}

pub trait Discover {
    /// NewService key
    type Key: Hash + Eq;

    /// The type of `Service` yielded by this `Discover`.
    type Service;

    /// Error produced during discovery
    type Error;

    /// Yields the next discovery change set.
    fn poll_discover(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Change<Self::Key, Self::Service>, Self::Error>>;
}

pub trait Service<Request> {
    /// Responses given by the service.
    type Response;

    /// Errors produced by the service.
    type Error;

    /// The future response value.
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    /// Returns `Poll::Ready(Ok(()))` when the service is able to process requests.
    ///
    /// If the service is at capacity, then `Poll::Pending` is returned and the task
    /// is notified when the service becomes ready again. This function is
    /// expected to be called while on a task. Generally, this can be done with
    /// a simple `futures::future::poll_fn` call.
    ///
    /// If `Poll::Ready(Err(_))` is returned, the service is no longer able to service requests
    /// and the caller should discard the service instance.
    ///
    /// Once `poll_ready` returns `Poll::Ready(Ok(()))`, a request may be dispatched to the
    /// service using `call`. Until a request is dispatched, repeated calls to
    /// `poll_ready` must return either `Poll::Ready(Ok(()))` or `Poll::Ready(Err(_))`.
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;

    /// Process the request and return the response asynchronously.
    ///
    /// This function is expected to be callable off task. As such,
    /// implementations should take care to not call `poll_ready`.
    ///
    /// Before dispatching a request, `poll_ready` must be called and return
    /// `Poll::Ready(Ok(()))`.
    ///
    /// # Panics
    ///
    /// Implementations are permitted to panic if `call` is invoked without
    /// obtaining `Poll::Ready(Ok(()))` from `poll_ready`.
    fn call(&mut self, req: Request) -> Self::Future;
}

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
    D::Service: Service<Request, Response = Response>,
    D::Error: Into<BoxedStdError>,
    <D::Service as Service<Request>>::Error: Into<BoxedStdError> + 'static,
    <D::Service as Service<Request>>::Future: Send + 'static,
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
