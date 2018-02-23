// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `write_op` module provides the Yobicash network client `WriteOp` methods.

use result::Result;
use traits::{Validate, Serialize};
use crypto::{Digest, SecretKey, PublicKey};
use crypto::BinarySerialize as CryptoBinarySerialize;
use models::WriteOp;
use store::Store;
use node::Node;
use network::traits::Connection;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::*;
use network::client::Client;

use std::io::{Read, Write};

impl<S: Store, T: Connection + Read + Write> Client<S, T> {
    /// Lists a node write operations.
    pub fn list_write_op(&mut self,
                             addr: &str,
                             timeout: Option<u64>,
                             read_timeout: Option<u64>,
                             write_timeout: Option<u64>) -> Result<Vec<Digest>> {

        let resource_type = ResourceType::WriteOp;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            ListRequest::new(session, resource_type)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = ListResponse::from_bytes(&_res)?;

        res.validate()?;

        let mut ids = Vec::new();
        
        for resource in res.resources {
            ids.push(Digest::from_bytes(&resource)?);
        }

        Ok(ids)
    }
    
    /// Samples a node write operations.
    pub fn sample_write_ops(&mut self,
                                addr: &str,
                                timeout: Option<u64>,
                                read_timeout: Option<u64>,
                                write_timeout: Option<u64>,
                                count: u32) -> Result<Vec<Digest>> {

        let resource_type = ResourceType::WriteOp;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            SampleRequest::new(session, resource_type, count)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = SampleResponse::from_bytes(&_res)?;

        res.validate()?;

        let mut ids = Vec::new();
        
        for resource in res.resources {
            ids.push(Digest::from_bytes(&resource)?);
        }

        Ok(ids)
    }

    /// Gets a node write operation.
    pub fn get_write_op(&mut self,
               addr: &str,
               timeout: Option<u64>,
               read_timeout: Option<u64>,
               write_timeout: Option<u64>,
               id: Digest) -> Result<WriteOp> {

        let resource_type = ResourceType::WriteOp;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            GetRequest::new(session, resource_type, id)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = GetResponse::from_bytes(&_res)?;

        res.validate()?;

        WriteOp::from_bytes(&res.resource)
    }

    /// Lookups a node write operation.
    pub fn lookup_write_op(&mut self,
                           addr: &str,
                           timeout: Option<u64>,
                           read_timeout: Option<u64>,
                           write_timeout: Option<u64>,
                           id: Digest) -> Result<bool> {

        let resource_type = ResourceType::WriteOp;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            LookupRequest::new(session, resource_type, id)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = LookupResponse::from_bytes(&_res)?;

        res.validate()?;

        Ok(res.found)
    }

    /// Puts a write operation to a node.
    pub fn put_write_op(&mut self,
                        addr: &str,
                        timeout: Option<u64>,
                        read_timeout: Option<u64>,
                        write_timeout: Option<u64>,
                        secret_key: SecretKey,
                        public_key: PublicKey,
                        plaintext: &[u8]) -> Result<()> {
        /* TODO

        write_op.validate()?;

        let resource_type = ResourceType::WriteOp;
        let resource = write_op.to_bytes()?;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            PutRequest::new(session, resource_type, &resource)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = PutResponse::from_bytes(&_res)?;

        res.validate()
        */
        unreachable!()
    }
}
