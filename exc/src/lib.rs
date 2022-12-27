//! Exc: Abstractions for exchanges.
#![deny(missing_docs)]

/// Market.
pub mod market;

/// Types.
pub mod types;

/// Utils for using low-level apis ([`exc::core`](crate::core)).
pub mod util;

pub use self::core::{Adaptor, Exc, ExcService, ExchangeError, IntoExc, Request};
pub use exc_core as core;
pub use util::{
    book::SubscribeBidAskService,
    fetch_candles::FetchCandlesService,
    instrument::{FetchInstrumentsService, SubscribeInstrumentsService},
    subscribe_tickers::SubscribeTickersService,
    trade::SubscribeTradesService,
    trading::{CheckOrderService, SubscribeOrdersService, TradingService},
    ExcExt,
};

/// Prelude.
pub mod prelude {
    pub use crate::core::{
        types::{Period, Place, PlaceOrderOptions},
        Adaptor, Exc, ExcService, ExchangeError, Request,
    };
    pub use crate::util::{
        book::SubscribeBidAskService,
        fetch_candles::FetchCandlesService,
        instrument::{FetchInstrumentsService, SubscribeInstrumentsService},
        reconnect::ReconnectService,
        subscribe_tickers::SubscribeTickersService,
        trade::SubscribeTradesService,
        trading::{CheckOrderService, SubscribeOrdersService, TradingService},
        ExcExt,
    };

    #[cfg(feature = "okx")]
    pub use crate::Okx;

    #[cfg(feature = "binance")]
    pub use crate::Binance;
}

#[cfg(feature = "okx")]
/// Okx exchange service.
pub mod okx {
    pub use exc_okx::*;
}

#[cfg(feature = "binance")]
/// Binance exchange service.
pub mod binance {
    pub use exc_binance::*;
}

#[cfg(feature = "okx")]
pub use crate::okx::Okx;

#[cfg(feature = "binance")]
pub use crate::binance::Binance;
