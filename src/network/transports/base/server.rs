// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `server` module provides the base transport server implementation for yobicash network

use network::transports::base::transport_session::*;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ListenError {
    InvalidEndpoint,  // the endpoint string provided was not valid
    AlreadyListening, // the server was already listening
    NotRunning,       // the server was shutdown before listening
    SystemError,      // some error was returned from the OS
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum AcceptError {
    NotListening,   // the server isn't setup to listen
    NotPending,     // there is not an inbound connection waiting
    NotRunning,     // the server was shutdown before trying to accept
    SystemError,    // some error was returned from the OS
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ShutdownError {
    NotRunning,   // the server was already shutdown
    UnknownError, // something went wrong in a generic way
}

pub type ListenResult   = Result<bool, ListenError>;
pub type AcceptResult   = Result<BaseTransportSession, AcceptError>;
pub type ShutdownResult = Result<bool, ShutdownError>;

pub struct BaseServer {
    pub is_listening: bool, // set true once the server is listening
    pub has_pending: bool,  // only used by this base implementation, specifies if accept() should return a session or not
    pub is_active: bool,    // set true at startup when server is created, cleared by shutdown()
}

impl BaseServer {
     /// Creates a new `BaseServer`
     pub fn new() -> BaseServer {
         // by default, we always have pending connections for mock-based testing
         let retval = BaseServer{is_listening: false,
                                 has_pending:  true,
                                 is_active:    true,};
         return retval;
     }

     /// Listens to the specified endpoint, preparing the server for inbound connections
     pub fn listen_on<S: Into<String>>(&mut self, endpoint: S) -> ListenResult{
         if(self.is_active) {
            if(self.is_listening) {
               Err(ListenError::AlreadyListening)
            } else {
               self.is_listening = true;
               Ok(true)
            }
         } else {
            Err(ListenError::NotRunning)
         }
     }

     /// Accepts a pending connection, if there is one, otherwise returns none
     pub fn accept(&mut self) -> AcceptResult {
         if(self.is_active) {
            if(self.is_listening == false) {
               Err(AcceptError::NotListening)
            } else {
               if(self.has_pending) {
                  // If there's a new connection waiting, we accept it and return a new session
                  let retval = BaseTransportSession::new("localhost:1337", true);
                  Ok(retval)
               } else {
                  Err(AcceptError::NotPending)
               }
            }
         } else {
            Err(AcceptError::NotRunning)
         }
     }

     /// Shuts down the server, attempting to use it after this results in undefined behaviour
     pub fn shutdown(&mut self) -> ShutdownResult {
         if(self.is_active) {
            self.has_pending  = false;
            self.is_listening = false;
            self.is_active    = false;
            Ok(true)
         } else {
            Err(ShutdownError::NotRunning)
         }
     }
}
