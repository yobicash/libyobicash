// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `transport` module tests.

use libyobicash::network::transports::base::*;

#[test]
fn base_client_connect_ok() {
   let mut my_client: client::BaseClient = client::BaseClient::new();

   let mut connect_result: client::ConnectResult = my_client.connect_to("127.0.0.1:2112");

   assert!(connect_result.is_ok());

   let mut my_session: transport_session::BaseTransportSession = connect_result.unwrap();

   assert!(my_session.is_open);
   
   my_session.shutdown();
   my_client.shutdown();
}

#[test]
fn base_server_listen_ok() {
   let mut my_server: server::BaseServer = server::BaseServer::new();

   let mut listen_result: server::ListenResult = my_server.listen_on("127.0.0.1:2112");

   assert!(listen_result.is_ok());

   let listened_ok: bool = listen_result.unwrap();
   assert!(listened_ok);

   my_server.shutdown();
}

#[test]
fn accept_before_listen_fails() {
   let mut my_server: server::BaseServer = server::BaseServer::new();

   let mut accept_result: server::AcceptResult = my_server.accept();

   assert!(accept_result.is_err());
   let accept_error: server::AcceptError = accept_result.err().unwrap();

   assert_eq!(accept_error, server::AcceptError::NotListening);
   
   my_server.shutdown();
}

#[test]
fn accept_after_listen_ok() {
   let mut my_server: server::BaseServer = server::BaseServer::new();

   let listen_result: server::ListenResult = my_server.listen_on("127.0.0.1:2112");
   assert!(listen_result.is_ok());
   let listened_ok: bool = listen_result.unwrap();
   assert!(listened_ok);

   let mut accept_result: server::AcceptResult = my_server.accept();
   assert!(accept_result.is_ok());

   let mut my_session: transport_session::BaseTransportSession = accept_result.unwrap();
   assert!(my_session.is_open);

   my_session.shutdown();
   my_server.shutdown();
}

#[test]
fn accept_without_pending() {
   let mut my_server: server::BaseServer = server::BaseServer::new();

   let listen_result: server::ListenResult = my_server.listen_on("127.0.0.1:2112");
   assert!(listen_result.is_ok());

   my_server.has_pending = false;

   let accept_result: server::AcceptResult = my_server.accept();
   assert!(accept_result.is_err());

   let accept_error: server::AcceptError = accept_result.err().unwrap();
   assert_eq!(accept_error, server::AcceptError::NotPending);

   my_server.shutdown();
}

#[test]
fn accept_after_shutdown_fails() {
   let mut my_server: server::BaseServer = server::BaseServer::new();

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
   let mut my_server: server::BaseServer = server::BaseServer::new();

   let first_shutdown_result: server::ShutdownResult = my_server.shutdown();
   assert!(first_shutdown_result.is_ok());
   let first_shutdown_ok: bool = first_shutdown_result.unwrap();
   assert!(first_shutdown_ok);

   let second_shutdown_result: server::ShutdownResult = my_server.shutdown();
   assert!(second_shutdown_result.is_err());

   let second_shutdown_error: server::ShutdownError = second_shutdown_result.err().unwrap();
   assert_eq!(second_shutdown_error, server::ShutdownError::NotRunning);
}

// the below is quite a large test, more of an integration test than unit test
// it's needed to verify the actual functionality though
#[test]
fn send_and_recv_msg() {
   let mut my_server: server::BaseServer = server::BaseServer::new();
   
   let mut listen_result: server::ListenResult = my_server.listen_on("127.0.0.1:2112");
   assert!(listen_result.is_ok());
   let mut listened_ok: bool = listen_result.unwrap();
   assert!(listened_ok);

   let mut my_client: client::BaseClient = client::BaseClient::new();

   let mut connect_result: client::ConnectResult = my_client.connect_to("127.0.0.1:2112");
   assert!(connect_result.is_ok());

   let mut accept_result: server::AcceptResult = my_server.accept();
   assert!(accept_result.is_ok());

   let mut client_session: transport_session::BaseTransportSession = connect_result.unwrap();
   let mut server_session: transport_session::BaseTransportSession = accept_result.unwrap();

   // when using the base implementation, we must manually configure the send/recv channels
   client_session.set_send_to(&mut server_session);
   server_session.set_send_to(&mut client_session);

   // first we try to send a message from client to server
   let mut hello_msg = "Hello";
   let mut send_result: transport_session::SendMsgResult = client_session.send_msg(hello_msg.as_bytes(),false);
   assert!(send_result.is_ok());
   
   // we verify it sent the correct number of bytes (should send 5 bytes for the "hello" string)
   let mut sent_bytes: usize = send_result.unwrap();
   assert_eq!(sent_bytes, 5);
   
   // now we try and receive it
   let mut recv_result: transport_session::RecvMsgResult = server_session.recv_msg();
   assert!(recv_result.is_ok());
   
   // check we received a message of 5 bytes
   let mut recv_msg: Vec<u8> = recv_result.unwrap();
   assert_eq!(recv_msg.len(), 5);

   // convert it to a string and check it matches the "Hello" string
   let mut recv_str: String = String::from_utf8(recv_msg).unwrap(); // convert it into a string
   assert_eq!(hello_msg, recv_str);

   // finally, cleanup
   client_session.shutdown();
   server_session.shutdown();
   my_client.shutdown();
   my_server.shutdown();
}
