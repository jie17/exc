use super::error::RestError;
use anyhow::anyhow;
use exc_core::ExchangeError;
use http::StatusCode;
use serde::{de::DeserializeOwned, Deserialize};

/// Instrument.
pub mod instrument;

/// Candle.
pub mod candle;

/// Listen key.
pub mod listen_key;

/// Error message.
pub mod error_message;

/// Trading.
pub mod trading;

use self::trading::Order;
pub use self::{
    candle::Candle, error_message::ErrorMessage, instrument::ExchangeInfo, listen_key::ListenKey,
};

/// Candles.
pub type Candles = Vec<Candle>;

/// Unknown response.
pub type Unknown = serde_json::Value;

/// Binance rest api response data.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Data {
    /// Candles.
    Candles(Vec<Candle>),
    /// Exchange info.
    ExchangeInfo(ExchangeInfo),
    /// Listen key.
    ListenKey(ListenKey),
    /// Error Message.
    Error(ErrorMessage),
    /// Order.
    Order(Order),
    /// Unknwon.
    Unknwon(Unknown),
}

impl TryFrom<Data> for Unknown {
    type Error = RestError;

    fn try_from(value: Data) -> Result<Self, Self::Error> {
        match value {
            Data::Unknwon(u) => Ok(u),
            _ => Err(RestError::UnexpectedResponseType(anyhow::anyhow!(
                "{value:?}"
            ))),
        }
    }
}

/// Binance rest api response.
#[derive(Debug)]
pub struct RestResponse<T> {
    data: T,
}

impl<T> RestResponse<T> {
    /// Into inner data.
    pub fn into_inner(self) -> T {
        self.data
    }

    /// Convert into a response of the given type.
    pub fn into_response<U>(self) -> Result<U, RestError>
    where
        U: TryFrom<T, Error = RestError>,
    {
        U::try_from(self.into_inner())
    }

    pub(crate) async fn from_http(resp: http::Response<hyper::Body>) -> Result<Self, RestError>
    where
        T: DeserializeOwned,
    {
        let status = resp.status();
        tracing::trace!("http response status: {}", resp.status());
        let value = hyper::body::to_bytes(resp.into_body())
            .await
            .map_err(RestError::from)
            .and_then(|bytes| {
                serde_json::from_slice::<serde_json::Value>(&bytes).map_err(RestError::from)
            });
        match status {
            StatusCode::TOO_MANY_REQUESTS => Err(RestError::Exchange(ExchangeError::RateLimited(
                anyhow!("too many requests"),
            ))),
            StatusCode::IM_A_TEAPOT => Err(RestError::Exchange(ExchangeError::RateLimited(
                anyhow!("I'm a teapot"),
            ))),
            StatusCode::SERVICE_UNAVAILABLE => Err(RestError::Exchange(match value {
                Ok(msg) => ExchangeError::Unavailable(anyhow!("{msg}")),
                Err(err) => ExchangeError::Unavailable(anyhow!("failed to read msg: {err}")),
            })),
            _ => match value {
                Ok(value) => match serde_json::from_value::<T>(value) {
                    Ok(data) => Ok(Self { data }),
                    Err(err) => Err(err.into()),
                },
                Err(err) => Err(err),
            },
        }
    }
}
