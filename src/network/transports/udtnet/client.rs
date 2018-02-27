// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `client` module provides the UDT transport client implementation for yobicash network
//! Please note that the client implemented here should have 1 instance per remote peer, but that 1 remote peer can be on both IPv4 and IPv6
//! IPv6 will be preferred if available

use network::transports::udtnet::transport_session::UDTTransportSession;

extern crate udt;

use std::str::FromStr;
use std::net::{SocketAddr,SocketAddrV4,SocketAddrV6};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ConnectError {
    InvalidEndpoint,  // the endpoint string provided was not valid
    TimedOut,         // we timed out while trying to connect
    InitSockError,    // Could not init socket
    SystemError,      // some error was returned from the OS
    UnknownError,     // an unknown error occurred while trying to connect
    NotRunning,       // the client has been shutdown
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ShutdownError {
    NotRunning, // the client was already shutdown
    Unknown,    // an unknown error occurred while shutting down
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum InitIP4Error {
    NotRunning,    // the client has been shutdown
    AlreadyInited, // we already have an IPv4 socket ready
    Unknown,       // an unknown error occurred while setting up
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum InitIP6Error {
    NotRunning,    // the client has been shutdown
    AlreadyInited, // we already have an IPv6 socket ready
    LegacyOnly,    // the platform doesn't support IPv6, only IPv4
    Unknown,       // an unknown error occurred while setting up
}

pub type ConnectResult  = Result<UDTTransportSession, ConnectError>;
pub type ShutdownResult = Result<bool, ShutdownError>;
pub type InitIP4Result  = Result<bool, InitIP4Error>;
pub type InitIP6Result  = Result<bool, InitIP6Error>;

pub struct UDTClient {
    pub is_active: bool,
    pub has_6: bool, // IPv6 is active and supported
    pub has_4: bool, // IPv4 is active and supported
    udt_sock_v4: Option<udt::UdtSocket>, // used for IPv4 endpoints
    udt_sock_v6: Option<udt::UdtSocket>, // used for IPv6 endpoints
}

impl UDTClient {
     /// Creates a new `UDTClient`
     pub fn new() -> UDTClient {
         udt::init();
         let retval = UDTClient{is_active:   true,
                                has_6:       false,
                                has_4:       false,
                                udt_sock_v4: None,
                                udt_sock_v6: None};
         return retval;
     } 

     /// Configures the client to be ready for outbound IPv4 connections
     /// This is automatically called by connect_to() and can be combined with IPv6
     pub fn init_v4(&mut self) ->InitIP4Result {
         if(self.is_active) {
            match(self.udt_sock_v4) {
               Some(s) => {Err(InitIP4Error::AlreadyInited)},
               None    => {self.udt_sock_v4 = Some(udt::UdtSocket::new(udt::SocketFamily::AFInet,  udt::SocketType::Datagram).unwrap());
                           self.has_4 = true;
                           Ok(true)},
            }
         } else {
            Err(InitIP4Error::NotRunning)
         }
     }

     /// Same as init_v4, but for IPv6
     // TODO - add proper error checking, including failing if OS does not support IPv6 etc
     pub fn init_v6(&mut self) ->InitIP6Result {
         if(self.is_active) {
            match(self.udt_sock_v6) {
               Some(s) => {Err(InitIP6Error::AlreadyInited)},
               None    => {self.udt_sock_v6 = Some(udt::UdtSocket::new(udt::SocketFamily::AFInet6,  udt::SocketType::Datagram).unwrap());
                           self.has_6 = true;
                           Ok(true)},
            }
         } else {
            Err(InitIP6Error::NotRunning)
         }
     }

     /// Connects the client to the specified endpoint
     pub fn connect_to<S: Into<String>>(&mut self, endpoint: S) -> ConnectResult {
         if(self.is_active) {
            let mut endpoint_str  = endpoint.into();
            let mut sock_addr_ret = SocketAddr::from_str(&endpoint_str);
            if(sock_addr_ret.is_ok()) {
               let mut sock_addr = sock_addr_ret.unwrap();
               if(sock_addr.is_ipv4()) {
                  let mut init4_ret: InitIP4Result = self.init_v4();
                  if(self.has_4) {
                    let mut ip4_sock  = self.udt_sock_v4.unwrap();
                    let mut conn4_ret = ip4_sock.connect(sock_addr);
                    if(conn4_ret.is_ok()) {
                      let mut retval: UDTTransportSession = UDTTransportSession::new(ip4_sock);
                      Ok(retval)
                    } else {
                      Err(ConnectError::UnknownError)
                    }
                  } else {
                    Err(ConnectError::InitSockError)
                  }
               } else {
                  let mut init6_ret: InitIP6Result = self.init_v6(); // we might fail to support IPv6
                  if(self.has_6) {
                    let mut ip6_sock  = self.udt_sock_v6.unwrap();
                    let mut conn6_ret = ip6_sock.connect(sock_addr);
                    if(conn6_ret.is_ok()) {
                       let mut retval: UDTTransportSession = UDTTransportSession::new(ip6_sock);
                       Ok(retval)
                    } else {
                       Err(ConnectError::UnknownError)
                    }
                  } else {
                    Err(ConnectError::InitSockError) // TODO - add specific errors for IPv6 not supported etc
                  }
               }
            } else {
               Err(ConnectError::InvalidEndpoint)
            }
         } else {
            Err(ConnectError::NotRunning)
         }
     } 

     pub fn shutdown(&mut self) -> ShutdownResult {
         if(self.is_active) {
            self.is_active = false;
            if(self.has_4) {
              self.udt_sock_v4.unwrap().close();
              self.has_4 = false;
            }
            if(self.has_6) {
              self.udt_sock_v6.unwrap().close();
              self.has_6 = false;
            }
            Ok(true)
         } else {
            Err(ShutdownError::NotRunning)
         }
     }

}

