//! WebSocket to TCP proxy server.
//!
//! # References
//!
//! - [RFC 6455] The WebSocket Protocol
//!
//! [RFC 6455]: https://tools.ietf.org/html/rfc6455
#![warn(missing_docs)]
#[macro_use]
extern crate bytecodec;
#[macro_use]
extern crate trackable;

use async_std::net::TcpListener;
use std::net::SocketAddr;

pub use error::{Error, ErrorKind};
pub use server::ProxyServer;

mod channel;
mod error;
mod frame;
mod opcode;
mod server;
mod util;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

/// This function will start a WS to TCP Proxy
pub async fn run_proxy(
  bind_addr: SocketAddr,
  tcp_server_addr: SocketAddr,
) -> trackable::result::TopLevelResult {
  let listener = track!(TcpListener::bind(bind_addr).await.map_err(Error::from))
    .expect("failed to start listening on the given proxy address");

  let proxy = ProxyServer::new(listener.incoming(), tcp_server_addr)
    .await
    .unwrap_or_else(|e| panic!("{}", e));
  proxy.await.unwrap_or_else(|e| panic!("{}", e));
  Ok(())
}
