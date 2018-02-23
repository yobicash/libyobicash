// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `message` module provides the Yobicash network message types and methods.

pub mod handshake;
pub mod error;
pub mod response_header;
pub mod ping;
pub mod list;
pub mod sample;
pub mod lookup;
pub mod get;
pub mod put;

pub use self::handshake::*;
pub use self::error::*;
pub use self::response_header::*;
pub use self::ping::*;
pub use self::list::*;
pub use self::sample::*;
pub use self::lookup::*;
pub use self::get::*;
pub use self::put::*;

/// The network message type.
pub enum Message {
    /// An error response message.
    ErrorResponse(ErrorResponse),
    /// A response header response.
    ResponseHeader(ResponseHeader),
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
