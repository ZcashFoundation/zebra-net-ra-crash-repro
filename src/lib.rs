//! Networking code for Zebra. 🦓
//!
//! The Zcash network protocol is inherited from Bitcoin, which uses a
//! stateful network protocol in which messages can arrive in any
//! order (even before a handshake is complete!), and the same message
//! may be a request or a response depending on context.
//!
//! This crate translates the legacy Bitcoin/Zcash network protocol
//! into a stateless, request-response oriented protocol defined by
//! the [`Request`] and [`Response`] enums, and completely
//! encapsulates all peer handling code behind a single
//! [`tower::Service`] representing "the network" which load-balances
//! outbound [`Request`]s over available peers.
//!
//! Each peer connection is a distinct task, which interprets incoming
//! Bitcoin/Zcash messages either as [`Response`]s to pending
//! [`Request`]s, or as an inbound [`Request`]s to an internal
//! [`tower::Service`] representing "this node".  All connection state
//! is isolated to the peer in question, so as a side effect the
//! design is structurally immune to the recent `ping` attack.
//!
//! Because [`tower::Service`]s provide backpressure information, we
//! can dynamically manage the size of the connection pool according
//! to inbound and outbound demand.  The inbound service can shed load
//! when it is not ready for requests, causing those peer connections
//! to close, and the outbound service can connect to additional peers
//! when it is overloaded.

#![doc(html_logo_url = "https://www.zfnd.org/images/zebra-icon.png")]
#![doc(html_root_url = "https://doc.zebra.zfnd.org/zebra_network")]
#![deny(missing_docs)]
// Tracing causes false positives on this lint:
// https://github.com/tokio-rs/tracing/issues/553
#![allow(clippy::cognitive_complexity, clippy::try_err, clippy::type_complexity)]

/// Type alias to make working with tower traits easier.
///
/// Note: the 'static lifetime bound means that the *type* cannot have any
/// non-'static lifetimes, (e.g., when a type contains a borrow and is
/// parameterized by 'a), *not* that the object itself has 'static lifetime.
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

mod peer_set {
    mod set;
}
