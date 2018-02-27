// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `server` module provides the UDT transport server implementation for yobicash network

extern crate udt;

use std::str::FromStr;
use std::net::{SocketAddr,SocketAddrV4,SocketAddrV6};

use network::transports::udtnet::transport_session::*;

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
pub enum ShutdownError {
    NotRunning,   // the server was already shutdown
    UnknownError, // something went wrong in a generic way
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum AcceptError {
    NotListening,   // the server isn't setup to listen
    NotPending,     // there is not an inbound connection waiting
    NotRunning,     // the server was shutdown before trying to accept
    SystemError,    // some error was returned from the OS
}

pub type ListenResult   = Result<bool, ListenError>;
pub type ShutdownResult = Result<bool, ShutdownError>;
pub type AcceptResult   = Result<UDTTransportSession, AcceptError>;

pub struct UDTServer {
    pub is_listening: bool,   // set true once the server is listening
    pub is_active: bool,      // set true at startup when server is created, cleared by shutdown()

    udt_sock: Option<udt::UdtSocket>, // the underlying UDT socket used by the server
}

impl UDTServer {
     /// Creates a new `UDTServer`
     pub fn new() -> UDTServer {
         udt::init(); 
         let retval = UDTServer{is_listening: false,
                                is_active:    true,
                                udt_sock:     None,};
         return retval;
     }

     /// Listens to the specified endpoint, preparing the server for inbound connections
     pub fn listen_on<S: Into<String>>(&mut self, endpoint: S) -> ListenResult{
         if(self.is_active) {
            if(self.is_listening) {
               Err(ListenError::AlreadyListening)
            } else {
               let mut endpoint_str  = endpoint.into();
               let mut sock_addr_ret = SocketAddr::from_str(&endpoint_str);
               if(sock_addr_ret.is_ok()) {
                  let mut sock_addr = sock_addr_ret.unwrap();
                  if(sock_addr.is_ipv4()) {
                     self.udt_sock = Some(udt::UdtSocket::new(udt::SocketFamily::AFInet,  udt::SocketType::Datagram).unwrap());
                  } else { // if it's not IPv4, it's gotta be IPv6, or something really weird is going on
                     self.udt_sock = Some(udt::UdtSocket::new(udt::SocketFamily::AFInet6, udt::SocketType::Datagram).unwrap());
                  }
                  self.udt_sock.unwrap().bind(sock_addr);
                  Ok(true)
               } else {
                  Err(ListenError::InvalidEndpoint)
               }
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
               // TODO - add UDT accept() here
               Err(AcceptError::NotPending)
            }
         } else {
            Err(AcceptError::NotRunning)
         }
     }


     /// Shuts down the server, attempting to use it after this results in undefined behaviour
     pub fn shutdown(&mut self) -> ShutdownResult {
         if(self.is_active) {
            self.is_listening = false;
            self.is_active    = false;
            match(self.udt_sock) {
               Some(s) => {s.close();
                           Ok(true)},
               None    => {Ok(true)},
            }
         } else {
            Err(ShutdownError::NotRunning)
         }
     }
}
