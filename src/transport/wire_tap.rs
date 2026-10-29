//! A tap on the frames a client writes to TWS.

use std::fmt;
use std::sync::Arc;

use crate::messages::OutgoingMessages;

/// Told of every frame the client writes to TWS, once the write has succeeded.
///
/// Every request, cancel and order the client sends passes through it, including those
/// the client sends on its own: the cancel when a subscription is dropped, and the
/// market-data type it re-sends after a reconnect. The connection handshake, written
/// before the client exists, is not. Install one with `ClientBuilder::wire_tap`.
///
/// It is called on the path that writes to the socket, so it must be quick and must not
/// block: count, do not log.
pub trait WireTap: Send + Sync + 'static {
    /// One frame was written. `kind` is its request kind, or `None` if its message id is
    /// not one this crate knows.
    fn frame_written(&self, kind: Option<OutgoingMessages>);
}

/// The installed tap, shared by the transports.
#[derive(Clone)]
pub(crate) struct SharedWireTap(Arc<dyn WireTap>);

impl SharedWireTap {
    pub(crate) fn new(tap: Arc<dyn WireTap>) -> Self {
        Self(tap)
    }

    pub(crate) fn frame_written(&self, kind: Option<OutgoingMessages>) {
        self.0.frame_written(kind);
    }
}

impl fmt::Debug for SharedWireTap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WireTap")
    }
}
