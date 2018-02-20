// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `client` module provides the base transport session implementation for yobicash network


pub struct BaseTransportSession {
    pub remote_endpoint: String, // the endpoint of the remote peer
    pub is_open: bool,           // set true when the connection is live
}

