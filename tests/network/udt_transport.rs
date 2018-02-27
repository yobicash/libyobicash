// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `transport` module tests.

use libyobicash::network::transports::udtnet::*;
use std::thread::spawn;
use std::sync::mpsc::sync_channel;

#[test]
fn udt_server_listen_bad_addr() {
   let mut my_server: server::UDTServer = server::UDTServer::new();

   let mut listen_result: server::ListenResult = my_server.listen_on("This is definitely not a valid endpoint string!");
   assert!(listen_result.is_err());

   let listen_err: server::ListenError = listen_result.err().unwrap();
   assert_eq!(listen_err, server::ListenError::InvalidEndpoint);
   
   my_server.shutdown();
}

#[test]
fn udt_server_listen_ok() {
   let mut my_server: server::UDTServer = server::UDTServer::new();

   let mut listen_result: server::ListenResult = my_server.listen_on("127.0.0.1:2112");
   assert!(listen_result.is_ok());

   let listened_ok: bool = listen_result.unwrap();
   assert!(listened_ok);
   
   my_server.shutdown();
}

#[test]
fn accept_before_listen_fails() {
   let mut my_server: server::UDTServer = server::UDTServer::new();

   let mut accept_result: server::AcceptResult = my_server.accept();

   assert!(accept_result.is_err());
   let accept_error: server::AcceptError = accept_result.err().unwrap();

   assert_eq!(accept_error, server::AcceptError::NotListening);
   
   my_server.shutdown();
}


#[test]
fn accept_after_shutdown_fails() {
   let mut my_server: server::UDTServer = server::UDTServer::new();

   let listen_result: server::ListenResult = my_server.listen_on("127.0.0.1:2112");
   assert!(listen_result.is_ok());
   let listened_ok: bool = listen_result.unwrap();
   assert!(listened_ok);

   let shutdown_result: server::ShutdownResult = my_server.shutdown();
   assert!(shutdown_result.is_ok());
   let shutdown_ok: bool = shutdown_result.unwrap();
   assert!(shutdown_ok);

   let mut accept_result: server::AcceptResult = my_server.accept();
   assert!(accept_result.is_err());

   let accept_error: server::AcceptError = accept_result.err().unwrap();
   assert_eq!(accept_error, server::AcceptError::NotRunning);
}

#[test]
fn double_shutdown_fails() {
   let mut my_server: server::UDTServer = server::UDTServer::new();

   let first_shutdown_result: server::ShutdownResult = my_server.shutdown();
   assert!(first_shutdown_result.is_ok());
   let first_shutdown_ok: bool = first_shutdown_result.unwrap();
   assert!(first_shutdown_ok);

   let second_shutdown_result: server::ShutdownResult = my_server.shutdown();
   assert!(second_shutdown_result.is_err());

   let second_shutdown_error: server::ShutdownError = second_shutdown_result.err().unwrap();
   assert_eq!(second_shutdown_error, server::ShutdownError::NotRunning);
}

#[test]
fn send_and_recv_msg() {
   // we use this to block threads on each other to ensure there's no race conditions in the test
   let (tx,rx) = sync_channel(1);

   let server = spawn(move || {
       let mut my_server: server::UDTServer = server::UDTServer::new();

       let mut listen_result: server::ListenResult = my_server.listen_on("127.0.0.1:2112");
       assert!(listen_result.is_ok());
       let mut listened_ok: bool = listen_result.unwrap();
       assert!(listened_ok);

       tx.send(1).unwrap(); // we block here until the client is ready to send us a connection

       // accept the connection     
       let mut accept_result: server::AcceptResult = my_server.accept();
       assert!(accept_result.is_ok());
       let mut server_session: transport_session::UDTTransportSession = accept_result.unwrap();
       
       tx.send(1).unwrap(); // block until the client has sent us a message

       // receive the message
       let mut recv_result: transport_session::RecvMsgResult = server_session.recv_msg();
       assert!(recv_result.is_ok());
       let mut recv_msg: Vec<u8> = recv_result.unwrap();

       // check the message is of right length and content
       assert_eq!(recv_msg.len(), 5);
       let hello_msg = "Hello";
       let mut recv_str: String = String::from_utf8(recv_msg).unwrap();
       assert_eq!(recv_str, hello_msg);

       // finally, cleanup
       server_session.shutdown();
       my_server.shutdown();
   });

   let client = spawn(move || {
       let mut my_client: client::UDTClient = client::UDTClient::new();

       rx.recv().unwrap(); // unblock server thread

       // send connection
       let mut connect_result: client::ConnectResult = my_client.connect_to("127.0.0.1:2112");
       assert!(connect_result.is_ok());
       let mut client_session: transport_session::UDTTransportSession = connect_result.unwrap();

       rx.recv().unwrap(); // unblock server thread

       // try to send a message from client to server
       let mut hello_msg = "Hello";
       let mut send_result: transport_session::SendMsgResult = client_session.send_msg(hello_msg.as_bytes(),false);
       assert!(send_result.is_ok());
   
       // we verify it sent the correct number of bytes (should send 5 bytes for the "hello" string)
       let mut sent_bytes: usize = send_result.unwrap();
       assert_eq!(sent_bytes, 5);

       // finally, cleanup
       client_session.shutdown();
       my_client.shutdown();
   });
}
