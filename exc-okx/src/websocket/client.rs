use super::{
    transport::connection::Connection,
    types::{request::Request, response::Response},
};
use crate::error::OkxError;
use futures::{future::BoxFuture, FutureExt, TryFutureExt};
use tower::{buffer::Buffer, Service, ServiceExt};

/// Okx websocket client.
#[derive(Clone)]
pub struct Client {
    pub(crate) svc: Buffer<Connection, Request>,
}

impl Client {
    /// Send request.
    pub async fn send(
        &mut self,
        request: Request,
    ) -> Result<<Self as Service<Request>>::Future, OkxError> {
        self.ready().await?;
        let fut = self.call(request);
        Ok(fut)
    }
}

impl tower::Service<Request> for Client {
    type Response = Response;
    type Error = OkxError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.svc.poll_ready(cx).map_err(OkxError::Buffer)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.svc.call(req).map_err(OkxError::Buffer).boxed()
    }
}