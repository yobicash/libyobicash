// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `transport_session` module provides the base transport session implementation for yobicash network

use std::collections::VecDeque;
use std::sync::mpsc::{channel,Sender,Receiver,TryRecvError};

pub struct BaseTransportSession {
    pub remote_endpoint: String, // the endpoint of the remote peer
    pub is_open: bool,           // set true when the connection is live, serves same role as is_active

    // the below queues are only used if no channels are configured
    // the idea is to have some other process take care of running the queues
    pub send_q: VecDeque<Vec<u8>>, // messages awaiting transmission to the other side of the connection
    pub recv_q: VecDeque<Vec<u8>>, // messages received from the other side of the connection, awaiting reading

    // the below is used only by the base implementation, to simulate real peers
    pub send_channel: Option<Sender<Vec<u8>>>,   // used by send_msg(), should be set to the receive channel of the other peer
    pub recv_channel: Option<Receiver<Vec<u8>>>, // used by recv_msg(), should be set to the send channel of the other peer
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

impl BaseTransportSession {
     /// initializes a session and sets up the queues etc
     pub fn new<S: Into<String>>(endpoint: S, start_open: bool) -> BaseTransportSession {
         let retval = BaseTransportSession { remote_endpoint: endpoint.into(),
                                             is_open: start_open,
                                             send_channel: None,
                                             recv_channel: None,
                                             send_q: VecDeque::new(),
                                             recv_q: VecDeque::new(),};
         return retval;
     }

     /// used for testing/debugging, configures the send/recv channels
     pub fn set_send_to(&mut self, other: &mut BaseTransportSession) {
         let (tx,rx) = channel();
         self.send_channel  = Some(tx);
         other.recv_channel = Some(rx);
     }

     pub fn send_msg(&mut self, msg_data: &[u8], reliable: bool) -> SendMsgResult {
         if(self.is_open) {
            let mut sent_msg_data = msg_data.to_vec();
            let sent_len: usize   = sent_msg_data.len();
            match(self.send_channel) {
               None                   => {self.send_q.push_back(sent_msg_data);
                                          Ok(sent_len)},
               Some(ref mut channel)  => {channel.send(sent_msg_data);
                                          Ok(sent_len)},
               _                      => {Err(SendMsgError::Unknown)},
            }
         } else {
            Err(SendMsgError::NotConnected)
         }
     }

     pub fn recv_msg(&mut self) -> RecvMsgResult {
         if(self.is_open) {
           match(self.recv_channel) {
              None                   => {if(self.recv_q.is_empty()) {
                                            Err(RecvMsgError::NoMessages)
                                         } else {
                                            let retval = self.recv_q.pop_front();
                                            match(retval) {
                                               None            => Err(RecvMsgError::Unknown),
                                               Some(recv_data) => Ok(recv_data),
                                            }
                                         }},
              Some(ref mut channel)  => {let retval = channel.try_recv();
                                         if(retval.is_ok()) {
                                            Ok(retval.unwrap())
                                         } else {
                                            match(retval.err().unwrap()) {
                                               TryRecvError::Empty        => Err(RecvMsgError::NoMessages),
                                               TryRecvError::Disconnected => {self.is_open=false;
                                                                              Err(RecvMsgError::NotConnected)},
                                            }
                                        }},
              _                      => {Err(RecvMsgError::Unknown)},
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
