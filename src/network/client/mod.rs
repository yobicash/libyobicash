// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `client` module provides the Yobicash network client type and methods.

use constants::MAX_CHUNK_SIZE;
use result::Result;
use error::ErrorKind;
use traits::{Validate, Serialize};
use crypto::{Digest, assym_encrypt, assym_decrypt};
use utils::NetworkType;
use store::Store;
use node::Node;
use network::traits::Connection;
use network::session::Session;
use network::message::*;

use std::io::{Read, Write};
use std::net::Shutdown;

mod ping;
mod peer;
mod unspent_coin;
mod spent_coin;
mod unspent_output;
mod spent_output;
mod undeleted_data;
mod transaction;
mod write_op;

/// A network client in the Yobicash protocol.
pub struct Client<S: Store, T: Connection + Read + Write> {
    /// The client session.
    session: Session,
    /// The client transport.
    transport: T,
    /// The client node.
    node: Node<S>,
}

impl<S: Store, T: Connection + Read + Write> Client<S, T> {
    /// Creates a new `Client`.
    pub fn new(network_type: NetworkType, transport: T, node: Node<S>) -> Client<S, T> {
        Client {
            session: Session::new(network_type),
            transport: transport,
            node: node
        }
    }

    /// Sends a binary request to a `Server`.
    fn raw_send(&mut self,
                addr: &str,
                timeout: Option<u64>,
                read_timeout: Option<u64>,
                write_timeout: Option<u64>,
                builder: &mut FnMut(&mut Node<S>, &Session) -> Result<Vec<u8>>) -> Result<Vec<u8>> {
        // 1: connection setups
        
        let shutdown = Shutdown::Both;

        // 2: connect

        if timeout.is_none() {
            self.transport.connect(addr)?
        } else {
            self.transport.connect_timeout(addr, timeout.unwrap())?
        };

        if let Some(read_timeout) = read_timeout {
            self.transport.set_read_timeout(read_timeout)?;
        }

        if let Some(write_timeout) = write_timeout {
            self.transport.set_write_timeout(write_timeout)?;
        }

        // 3: send the syn message

        let syn = Syn::new(&self.session)?;

        let _ = self.transport.write(&syn.to_bytes()?)?;

        // 4: read the ack message

        let mut ack_buf = [0u8; 1024];
        let amt = self.transport.read(&mut ack_buf)?;
        let ack = Ack::from_bytes(&ack_buf[0..amt])?;
        ack.validate()?;

        // 5: check the ack message

        if ack.id != self.session.id {
            self.transport.shutdown(shutdown)?;
            return Err(ErrorKind::InvalidMessage.into());
        }

        if ack.network_type != self.session.network_type {
            self.transport.shutdown(shutdown).unwrap();
            return Err(ErrorKind::InvalidMessage.into());
        }

        if ack.public_key == syn.public_key {
            self.transport.shutdown(shutdown).unwrap();
            return Err(ErrorKind::InvalidMessage.into());
        }

        // 6: update the session

        self.session.update(&ack)?;
        
        // 7: prepare the data

        let mut plain = builder(&mut self.node, &self.session)?;
       
        let plain_size = plain.len() as u32;
        let plain_digest = Digest::hash(&plain);

        // 8: prepare the cyph

        let plain_padding = plain_size % 16;

        for _ in 0..plain_padding {
            plain.push(0);
        }

        let sk = self.session.secret_key;
        let pk = ack.public_key;
        
        let mut cyph = assym_encrypt(sk, pk, &plain).unwrap();

        let cyph_size = cyph.len() as u32;
        let cyph_digest = Digest::hash(&cyph);
        
        // 9: send the syn-ack message

        let padding = cyph_size % MAX_CHUNK_SIZE;
        let chunks_count = (cyph_size + padding) / MAX_CHUNK_SIZE;

        for _ in 0..padding {
            cyph.push(0u8);
        }

        let syn_ack = SynAck::new(&self.session, plain_size, cyph_size, padding, plain_digest, cyph_digest)?;

        let _ = self.transport.write(&syn_ack.to_bytes()?)?;

        // 10: send the chunks conn

        for i in 0..chunks_count as usize {
            let start = i*(MAX_CHUNK_SIZE as usize);
            let stop = (i+1)*(MAX_CHUNK_SIZE as usize);
            let chunk = &cyph[start..stop];
            let _ = self.transport.write(&chunk)?;
        }

        // 11: read the response message header

        let mut res_header_buf = [0u8; 1024];
        let amt = self.transport.read(&mut res_header_buf)?;
        let res_header = ResponseHeader::from_bytes(&ack_buf[0..amt])?;
        res_header.validate()?;

        // 12: check the response header message

        if res_header.id != self.session.id {
            self.transport.shutdown(shutdown)?;
            return Err(ErrorKind::InvalidMessage.into());
        }

        if res_header.network_type != self.session.network_type {
            self.transport.shutdown(shutdown).unwrap();
            return Err(ErrorKind::InvalidMessage.into());
        }

        // 13: read the incoming chunks

        let res_plain_size = res_header.plain_size;
        let res_cyph_size = res_header.cyph_size;
        let res_chunks_count = res_header.chunks_count;

        let mut res_cyph = Vec::new();

        for _ in 0..res_chunks_count {
            let mut res_chunk_buf = [0u8; MAX_CHUNK_SIZE as usize];
            let amt = self.transport.read(&mut res_chunk_buf)?;
            let res_chunk = &res_chunk_buf[0..amt];
            res_cyph.extend_from_slice(res_chunk);
        }

        res_cyph = Vec::from(&res_cyph[0..res_cyph_size as usize]);

        // 14: shutdown the connection

        self.transport.shutdown(shutdown)?;

        // 15: check the cypher

        let res_cyph_digest = Digest::hash(&res_cyph);

        if res_cyph_digest != res_header.cyph_digest {
            return Err(ErrorKind::InvalidDigest.into());
        }
        
        // 15: decrypt the received cyphertext

        let res_plain = assym_decrypt(sk, pk, &res_cyph, res_plain_size)?;

        let res_plain_digest = Digest::hash(&res_plain);

        if res_plain_digest != res_header.plain_digest {
            return Err(ErrorKind::InvalidDigest.into());
        }

        Ok(res_plain)
    }
}
