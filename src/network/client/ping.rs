// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `ping` module provides the Yobicash network client ping method.

use result::Result;
use traits::{Validate, Serialize};
use store::Store;
use node::Node;
use network::traits::Connection;
use network::session::Session;
use network::message::Ping;
use network::client::Client;

use std::io::{Read, Write};

impl<S: Store, T: Connection + Read + Write> Client<S, T> {
    /// Ping a node server.
    pub fn ping(&mut self,
               addr: &str,
               timeout: Option<u64>,
               read_timeout: Option<u64>,
               write_timeout: Option<u64>) -> Result<()> {

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            Ping::new(session)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = Ping::from_bytes(&_res)?;

        res.validate()
    }
}
