// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `client` module provides the Yobicash network client type and methods.

use network::traits::Connection;
use network::network_type::NetworkType;
use network::session::Session;

use std::io::{Read, Write};
use std::net::ShutDown;

pub struct Client<C: Connection> {
    session: Session,
    transport: C
}

impl<C: Connection> Client<C> {
    pub fn new(network_type: NetworType, transport: C) -> Client {
        Client {
            session: Session::new(network_type),
            transport: transport,
        }
    }

    fn raw_call(&mut self, addr: &str, data: &[u8]) -> Result<()> {
        // 1: connection setups
        
        let shutdown = Shutdown::Both;

        // 2: connect

        let mut stream = self.transport.connect(addr)?;

        // 3: send the syn message

        let syn = Syn::new(&self.session)?;

        let _ = stream.write(&syn.to_bytes()?)?;

        // 4: read the ack message

        let mut ack_buf = [0u8; 1024];
        let amt = stream.read(&mut ack_buf)?;
        let ack = Ack::from_bytes(&ack_buf[0..amt])?;
        ack.validate()?;

        // 5: check the ack message

        if ack.id != session.id {
            stream.shutdown(shutdown)?;
            return Err(ErrorKind::InvalidMessage.into());
        }

        if ack.network_type != session.network_type {
            stream.shutdown(shutdown).unwrap();
            return Err(ErrorKind::InvalidMessage.into());
        }

        if ack.public_key == syn.public_key {
            stream.shutdown(shutdown).unwrap();
            return Err(ErrorKind::InvalidMessage.into());
        }

        // 6: update the session

        session.public_key = Some(ack.public_key);
        session.max_size = Some(ack.max_size);
        session.pow_difficulty = Some(ack.pow_difficulty);
        session.fee_witness = ack.fee_witness;
        session.fee_per_byte = ack.fee_per_byte;
        
        // 7: prepare the data

        let mut plain = Vec::from(data);
       
        let plain_size = plain.len() as u32;
        let plain_digest = hash(&plain);

        // 8: prepare the cyph

        let plain_padding = data_size % 16;

        if plain_size + plain_padding > ack.max_size as usize {
            stream.shutdown(shutdown).unwrap();
            return Err(ErrorKind::InvalidLength.into());
        }
        
        for _ in 0..plain_padding {
            plain.push(0);
        }

        let sk = session.secret_key;
        let pk = ack.public_key;
        
        let mut cyph = assym_encrypt(sk, pk, &plain).unwrap();

        let cyph_size = cyph.len() as u32;
        let cyph_digest = hash(&cyph);
        
        // 9: send the syn-ack message

        let padding = cyph_size % MAX_CHUNK_SIZE;
        let chunks_count = (cyph_size + padding) / MAX_CHUNK_SIZE;

        for _ in 0..padding {
            cyph.push(0u8);
        }

        let syn_ack = SynAck::new(&self.session, plain_size, cyph_size, padding, plain_digest, cyph_digest)?;

        let _ = stream.write(&syn_ack.to_bytes()?)?;

        // 10: send the chunks stream

        for i in 0..chunks_count as usize {
            let start = i*MAX_CHUNK_SIZE;
            let stop = (i+1)*MAX_CHUNK_SIZE;
            let chunk = &cyph[start..stop];
            let _ = stream.write(&chunk)?;
        }

        // 11: read the chunks stream [max_size as max buffer] [missing]
        
        // 12: shutdown the connection.

        stream.shutdown(shutdown)?;

        Ok(())
    }
}
