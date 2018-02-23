// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `transaction` module provides the Yobicash network client `Transaction` methods.

use result::Result;
use traits::{Validate, Serialize};
use crypto::{Digest, ZKPWitness};
use crypto::BinarySerialize as CryptoBinarySerialize;
use utils::Amount;
use models::Transaction;
use store::Store;
use node::Node;
use network::traits::Connection;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::*;
use network::client::Client;

use std::io::{Read, Write};

impl<S: Store, T: Connection + Read + Write> Client<S, T> {
    /// Lists a node transactions.
    pub fn list_transaction(&mut self,
                             addr: &str,
                             timeout: Option<u64>,
                             read_timeout: Option<u64>,
                             write_timeout: Option<u64>) -> Result<Vec<Digest>> {

        let resource_type = ResourceType::Transaction;

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
    
    /// Samples a node transactions.
    pub fn sample_transactions(&mut self,
                                addr: &str,
                                timeout: Option<u64>,
                                read_timeout: Option<u64>,
                                write_timeout: Option<u64>,
                                count: u32) -> Result<Vec<Digest>> {

        let resource_type = ResourceType::Transaction;

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

    /// Gets a node transaction.
    pub fn get_transaction(&mut self,
               addr: &str,
               timeout: Option<u64>,
               read_timeout: Option<u64>,
               write_timeout: Option<u64>,
               id: Digest) -> Result<Transaction> {

        let resource_type = ResourceType::Transaction;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            GetRequest::new(session, resource_type, id)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = GetResponse::from_bytes(&_res)?;

        res.validate()?;

        Transaction::from_bytes(&res.resource)
    }

    /// Lookups a node transaction.
    pub fn lookup_transaction(&mut self,
                              addr: &str,
                              timeout: Option<u64>,
                              read_timeout: Option<u64>,
                              write_timeout: Option<u64>,
                              id: Digest) -> Result<bool> {

        let resource_type = ResourceType::Transaction;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            LookupRequest::new(session, resource_type, id)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = LookupResponse::from_bytes(&_res)?;

        res.validate()?;

        Ok(res.found)
    }

    /// Puts a transaction to a node.
    pub fn put_transaction(&mut self,
                           addr: &str,
                           timeout: Option<u64>,
                           read_timeout: Option<u64>,
                           write_timeout: Option<u64>,
                           outputs: &[(&Amount, ZKPWitness)]) -> Result<()> {
        /* TODO

        transaction.validate()?;

        let resource_type = ResourceType::Transaction;
        let resource = transaction.to_bytes()?;

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
