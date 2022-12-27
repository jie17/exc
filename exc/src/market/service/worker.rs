use std::{
    sync::Arc,
    task::{Context, Poll},
};

use exc_core::{ExchangeError, Str};
use futures::{future::BoxFuture, FutureExt};

use super::{state::State, FetchInstrumentSvc, SubscribeInstrumentSvc};

pub(super) struct Worker {
    init: Option<BoxFuture<'static, Result<(), ExchangeError>>>,
    state: Arc<State>,
    inst: SubscribeInstrumentSvc,
}

impl Worker {
    pub(super) fn new(
        state: &Arc<State>,
        inst: SubscribeInstrumentSvc,
        fetch: FetchInstrumentSvc,
    ) -> Self {
        let init = state.clone().init(fetch, Str::new("SPOT")).boxed();
        Self {
            init: Some(init),
            state: state.clone(),
            inst,
        }
    }

    pub(super) fn poll_init(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), ExchangeError>> {
        let Some(fut) = self.init.as_mut() else {
            return Poll::Ready(Ok(()))
        };
        fut.poll_unpin(cx)
    }

    pub(super) async fn start(self) -> Result<(), ExchangeError> {
        let Self { state, inst, .. } = self;
        let inst = state.watch_instruments(inst, Str::new("SPOT"));
        tokio::select! {
            res = inst => {
                res?;
            }
        }
        Ok(())
    }
}
