use super::connection::Connection;
use crate::websocket::Client;
use http::Uri;
use tower::buffer::Buffer;

const DEFAULT_BUFFER_SIZE: usize = 1024;

/// Okx websocket endpoint.
pub struct Endpoint {
    pub(crate) uri: Uri,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            uri: Uri::from_static("wss://wsaws.okex.com:8443/ws/v5/public"),
        }
    }
}

impl Endpoint {
    /// Connect and create a okx websocket channel.
    pub fn connect(&self) -> Client {
        let svc = Connection::new(self);
        let svc = Buffer::new(svc, DEFAULT_BUFFER_SIZE);
        Client { svc }
    }
}