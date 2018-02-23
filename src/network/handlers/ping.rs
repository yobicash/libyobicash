// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `ping` module provides the Yobicash network ping handler type and methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use network::session::Session;
use network::message::{Message, Ping};

/// The type used for handling `Ping` messages.
#[derive(Clone, Debug)]
pub struct PingHandler;

impl PingHandler {
    /// Handles a ping request.
    pub fn handle(session: &Session, req: &Ping) -> Result<Message> {
        session.validate()?;

        if req.id != session.id {
            return Err(ErrorKind::InvalidID.into());
        }

        if req.network_type != session.network_type {
            return Err(ErrorKind::InvalidSession.into());
        }

        req.validate()?;

        let res = req.clone();

        let message = Message::Ping(res);

        Ok(message)
    }
}
