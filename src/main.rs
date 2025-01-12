extern crate clap;

use clap::{Parser, ValueEnum};
use std::net::SocketAddr;
use wstcp::run_proxy;

#[derive(Parser)]
struct Args {
  /// The TCP address of the real server.
  real_server_addr: SocketAddr,

  /// TCP address to which the WebSocket proxy bind.
  #[clap(long, default_value = "0.0.0.0:13892")]
  bind_addr: SocketAddr,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum LogLevelArg {
  Debug,
  Info,
  Warning,
  Error,
}

fn main() -> trackable::result::TopLevelResult {
  env_logger::init();

  let args = Args::parse();
  let bind_addr = args.bind_addr;
  let tcp_server_addr = args.real_server_addr;

  async_std::task::block_on(run_proxy(bind_addr, tcp_server_addr))
}
