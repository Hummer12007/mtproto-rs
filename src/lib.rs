// `error_chain!` can nest quite deeply
#![recursion_limit = "165"]

#[macro_use]
extern crate arrayref;
extern crate byteorder;
extern crate chrono;
extern crate crc;
extern crate envy;
extern crate erased_serde;
#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate futures;
extern crate http;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate num_traits;
extern crate openssl;
extern crate rand;
extern crate select;
extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde_mtproto;
#[macro_use]
extern crate serde_mtproto_derive;
extern crate tokio_io;
extern crate tokio_tcp;
extern crate toml;


mod manual_types;
#[macro_use]
mod utils;

pub mod bigint;
pub mod crypto;
pub mod error;
pub mod network;
pub mod protocol;
pub mod rpc;
pub mod tl;

// Use this hack because `include!` and top-level inner attributes don't mix well
include!(concat!(env!("OUT_DIR"), "/schema.rs"));


pub use error::{Error, ErrorKind, Result, ResultExt};
pub use tl::TLObject;
