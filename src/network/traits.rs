// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `traits` module provides the Yobicash network traits.

use result::Result;

use std::io::{Read, Write};
use std::net::Shutdown;

/// Trait implementend by Yobicash network transports.
pub trait Connection: Sized + Read + Write {
    /// Connects to an address.
    fn connect(addr: &str) -> Result<Self>;

    /// Connects to an address, add a timeout.
    fn connect_timeout(addr: &str, timeout: u64) -> Result<Self>;

    /// Returns the peer address.
    fn peer_addr(&self) -> Result<String>;

    /// Returns the local address.
    fn local_addr(&self) -> Result<String>;

    /// Shuts down the connection.
    fn shutdown(&self, how: Shutdown) -> Result<()>;

    /// Sets a read timeout.
    fn set_read_timeout(&self, duration: u64) -> Result<()>;

    /// Sets a write timeout.
    fn set_write_timeout(&self, duration: u64) -> Result<()>;

    /// Returns the read timeout.
    fn read_timeout(&self) -> Result<u64>;

    /// Returns the write timeout.
    fn write_timeout(&self) -> Result<u64>;
}
