use exc_core::ExchangeError;
use http::Request;
use hyper::Body;
use serde::Serialize;

use crate::key::OkxKey as Key;

use self::earn::{EarnActiveOrders, EarnOffers, EarnPurchase, EarnRedeem};
use self::history_candles::HistoryCandles;
use self::instruments::Instruments;
use self::trading::Order;

/// History candles.
pub mod history_candles;

/// Instruments query.
pub mod instruments;

/// Trading.
pub mod trading;

/// Earn.
pub mod earn;

/// Okx HTTP API request types.
#[derive(Debug, Clone)]
pub enum HttpRequest {
    /// Get (public requests).
    Get(Get),
    /// Private Get.
    PrivateGet(PrivateGet),
    /// Private Post.
    PrivatePost(PrivatePost),
}

/// Okx HTTP API get request types.
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Get {
    /// History candles.
    HistoryCandles(HistoryCandles),
    /// Get instruments.
    Instruments(Instruments),
    /// Earn offers.
    EarnOffers(EarnOffers)
}

impl Get {
    pub(crate) fn uri(&self) -> &'static str {
        match self {
            Self::HistoryCandles(_) => "/api/v5/market/history-candles",
            Self::Instruments(_) => "/api/v5/public/instruments",
            Self::EarnOffers(_) => "/api/v5/finance/staking-defi/offers",
        }
    }
}

/// Okx HTTP API get request types.
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum PrivateGet {
    /// Order.
    Order(Order),
    /// Earn active orders.
    EarnActiveOrders(EarnActiveOrders)
}

impl PrivateGet {
    pub(crate) fn uri(&self) -> &'static str {
        match self {
            Self::Order(_) => "/api/v5/trade/order",
            Self::EarnActiveOrders(_) => "/api/v5/finance/staking-defi/orders-active",
        }
    }

    pub(crate) fn to_request(&self, host: &str, key: &Key) -> Result<Request<Body>, ExchangeError> {
        serde_qs::to_string(self)
            .map_err(|err| ExchangeError::Other(err.into()))
            .and_then(|q| {
                let uri = format!("{}?{q}", self.uri());
                let sign = key
                    .sign_now("GET", &uri, false)
                    .map_err(|e| ExchangeError::KeyError(anyhow::anyhow!("{e}")))?;
                Request::get(format!("{host}{uri}"))
                    .header("OK-ACCESS-KEY", key.apikey.as_str())
                    .header("OK-ACCESS-SIGN", sign.signature.as_str())
                    .header("OK-ACCESS-TIMESTAMP", sign.timestamp.as_str())
                    .header("OK-ACCESS-PASSPHRASE", key.passphrase.as_str())
                    .body(Body::empty())
                    .map_err(|err| ExchangeError::Other(err.into()))
            })
    }
}

/// Okx HTTP POST get request types.
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum PrivatePost {
    /// EarnPurchase.
    EarnPurchase(EarnPurchase),
    /// EarnRedeem.
    EarnRedeem(EarnRedeem),
}

impl PrivatePost {
    pub(crate) fn uri(&self) -> &'static str {
        match self {
            Self::EarnPurchase(_) => "/api/v5/finance/staking-defi/purchase",
            Self::EarnRedeem(_) => "/api/v5/finance/staking-defi/redeem",
        }
    }

    pub(crate) fn to_request(&self, host: &str, key: &Key) -> Result<Request<Body>, ExchangeError> {
        serde_qs::to_string(self)
            .map_err(|err| ExchangeError::Other(err.into()))
            .and_then(|q| {
                let uri = format!("{}?{q}", self.uri());
                let sign = key
                    .sign_now("post", &uri, false)
                    .map_err(|e| ExchangeError::KeyError(anyhow::anyhow!("{e}")))?;
                Request::post(format!("{host}{uri}"))
                    .header("OK-ACCESS-KEY", key.apikey.as_str())
                    .header("OK-ACCESS-SIGN", sign.signature.as_str())
                    .header("OK-ACCESS-TIMESTAMP", sign.timestamp.as_str())
                    .header("OK-ACCESS-PASSPHRASE", key.passphrase.as_str())
                    .body(Body::empty())
                    .map_err(|err| ExchangeError::Other(err.into()))
            })
    }
}
