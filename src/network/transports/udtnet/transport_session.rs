// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `transport_session` module provides the UDT transport session implementation for yobicash network

extern crate udt;

use std::iter::FromIterator;

pub struct UDTTransportSession {
    pub udt_sock: udt::UdtSocket, // the underlying UDT socket
    pub is_ipv4: bool,            // is this an IPv4 socket?
    pub is_ipv6: bool,            // is this an IPv6 socket?
    pub is_open: bool,           // set true when the connection is live, serves same role as is_active
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum SendMsgError {
    NotConnected, // the transport session is disconnected
    TimedOut,     // we timed out while sending the message
    Unknown,      // an unknown error occurred with the underlying transport
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum RecvMsgError {
    NotConnected, // the transport session is disconnected
    NoMessages,   // there are no messages waiting
    Unknown,      // an unknown error occurred with the underlying transport
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ShutdownError {
    NotRunning, // the transport was already shutdown
    Unknown,    // an unknown error occurred while shutting down
}
pub type SendMsgResult  = Result<usize, SendMsgError>;
pub type RecvMsgResult  = Result<Vec<u8>, RecvMsgError>;
pub type ShutdownResult = Result<bool, ShutdownError>;

// TODO - fix udt lib so we don't need to manually specify enums here
mod UDTErrors {
    pub const ECONNLOST: i32 = 2001;
    pub const ENOCONN:   i32 = 2002;
    pub const EASYNCRCV: i32 = 6002;
    pub const ETIMEOUT:  i32 = 6003;
}


impl UDTTransportSession {
     /// initializes a transport session, essentially a wrapper around a UDT socket
     pub fn new(sock: udt::UdtSocket) -> UDTTransportSession {
         let remote_addr = sock.getpeername().unwrap();
         let mut _is_ipv4: bool = false;
         let mut _is_ipv6: bool = false;
         if(remote_addr.is_ipv4()) {
            _is_ipv4 = true;
         } else {
            _is_ipv6 = true;
         }

         let retval = UDTTransportSession { udt_sock: sock,
                                            is_ipv4: _is_ipv4,
                                            is_ipv6: _is_ipv6,
                                            is_open: true};
         return retval;
     }

     // TODO - import udt-rs and fix it so we can send unreliable packets
     pub fn send_msg(&mut self, msg_data: &[u8], reliable: bool) -> SendMsgResult {
         if(self.is_open) {
            let sendmsg_ret = self.udt_sock.sendmsg(msg_data);
            if(sendmsg_ret.is_ok()) {
               let sent_len = sendmsg_ret.unwrap() as usize;
               Ok(sent_len)
            } else {
               // TODO - add proper error handling here
               Err(SendMsgError::Unknown)
            }
         } else {
            Err(SendMsgError::NotConnected)
         }
     }

     // TODO - use epoll to check for messages instead of blocking
     pub fn recv_msg(&mut self) -> RecvMsgResult {
         if(self.is_open) {
           let recv_buf = &mut [0u8; 4096]; // TODO - sort out dynamic buffer size
           let mut recv_ret = self.udt_sock.recvmsg(recv_buf);
           if(recv_ret.is_ok()) {
              let recv_len:   usize = recv_ret.unwrap() as usize;
              let mut retval = Vec::from_iter(recv_buf[0..recv_len].iter().cloned());
              Ok(retval)
           } else {
              let mut recv_err = recv_ret.err().unwrap();
              match(recv_err.err_code) {
                 UDTErrors::ECONNLOST => {self.shutdown();
                                          Err(RecvMsgError::NotConnected)},
                 UDTErrors::ENOCONN   => {self.shutdown();
                                          Err(RecvMsgError::NotConnected)},
                 UDTErrors::EASYNCRCV => {Err(RecvMsgError::NoMessages)},
                 UDTErrors::ETIMEOUT  => {Err(RecvMsgError::NoMessages)},
                 _                    => {Err(RecvMsgError::Unknown)},
              }
           }

         } else {
           Err(RecvMsgError::NotConnected)
         }
     }

     pub fn shutdown(&mut self) -> ShutdownResult {
         if(!self.is_open) {
            Err(ShutdownError::NotRunning)
         } else {
            Ok(true)
         }
     }
}
