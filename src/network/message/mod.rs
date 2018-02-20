// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `message` module provides the Yobicash network message types and methods.

pub mod handshake;
pub mod ping;
pub mod list;
pub mod sample;
pub mod lookup;
pub mod get;
pub mod put;

use self::handshake::*;
use self::ping::*;
use self::list::*;
use self::sample::*;
use self::lookup::*;
use self::get::*;
use self::put::*;

use network::message::list::request::*;
use network::message::list::response::*;
use network::message::sample::request::*;
use network::message::sample::response::*;
use network::message::lookup::request::*;
use network::message::lookup::response::*;
use network::message::get::request::*;
use network::message::get::response::*;
use network::message::put::request::*;
use network::message::put::response::*;

/// The network message type.
pub enum Message {
    /// A ping request or response message.
    Ping(Ping),
    /// A list request message.
    ListRequest(ListRequest),
    /// A list response message.
    ListResponse(ListResponse),
    /// A sample request message.
    SampleRequest(SampleRequest),
    /// A sample request message.
    SampleResponse(SampleResponse),
    /// A get request message.
    GetRequest(GetRequest),
    /// A get response message.
    GetResponse(GetResponse),
    /// A lookup request message.
    LookupRequest(LookupRequest),
    /// A lookup response message.
    LookupResponse(LookupResponse),
    /// A put request message.
    PutRequest(PutRequest),
    /// A put response message.
    PutResponse(PutResponse),
}
