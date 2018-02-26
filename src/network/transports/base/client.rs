// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `client` module provides the base transport client implementation for yobicash network

use network::transports::base::transport_session::BaseTransportSession;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ConnectError {
    InvalidEndpoint,  // the endpoint string provided was not valid
    SystemError,      // some error was returned from the OS
    NotRunning,       // the client has been shutdown
}   

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ShutdownError {
    NotRunning, // the client was already shutdown
    Unknown,    // an unknown error occurred while shutting down
}

pub type ConnectResult  = Result<BaseTransportSession, ConnectError>;
pub type ShutdownResult = Result<bool, ShutdownError>;

pub struct BaseClient {
    pub is_active: bool,
}

impl BaseClient {
     /// Creates a new `BaseClient`
     pub fn new() -> BaseClient {
         let retval = BaseClient{is_active: true};
         return retval;
     } 

     // TODO - define some standard errors here, instead of using String
     pub fn connect_to<S: Into<String>>(&mut self, endpoint: S) -> ConnectResult {
         if(self.is_active) {
            let retval = BaseTransportSession::new(endpoint.into(), true);
            Ok(retval)
         } else {
            Err(ConnectError::NotRunning)
         }
     } 

     pub fn shutdown(&mut self) -> ShutdownResult {
         if(self.is_active) {
            self.is_active = false;
            Ok(true)
         } else {
            Err(ShutdownError::NotRunning)
         }
     }

}

