// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `unspent_coin` module provides the Yobicash network client `UnspentCoin` methods.

use result::Result;
use traits::{Validate, Serialize};
use crypto::Digest;
use crypto::BinarySerialize as CryptoBinarySerialize;
use models::{CoinSource, Coin};
use store::Store;
use node::Node;
use network::traits::Connection;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::*;
use network::client::Client;

use std::io::{Read, Write};

impl<S: Store, T: Connection + Read + Write> Client<S, T> {
    /// Lists a node unspent coins.
    pub fn list_unspent_coins(&mut self,
                             addr: &str,
                             timeout: Option<u64>,
                             read_timeout: Option<u64>,
                             write_timeout: Option<u64>) -> Result<Vec<Digest>> {

        let resource_type = ResourceType::UnspentCoin;

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
    
    /// Samples a node unspent coins.
    pub fn sample_unspent_coins(&mut self,
                                addr: &str,
                                timeout: Option<u64>,
                                read_timeout: Option<u64>,
                                write_timeout: Option<u64>,
                                count: u32) -> Result<Vec<Digest>> {

        let resource_type = ResourceType::UnspentCoin;

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

    /// Gets a node unspent coin.
    pub fn get_unspent_coin(&mut self,
               addr: &str,
               timeout: Option<u64>,
               read_timeout: Option<u64>,
               write_timeout: Option<u64>,
               id: Digest) -> Result<Coin> {

        let resource_type = ResourceType::UnspentCoin;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            GetRequest::new(session, resource_type, id)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = GetResponse::from_bytes(&_res)?;

        res.validate()?;

        Coin::from_bytes(&res.resource)
    }

    /// Lookups a node unspent coin.
    pub fn lookup_unspent_coin(&mut self,
                               addr: &str,
                               timeout: Option<u64>,
                               read_timeout: Option<u64>,
                               write_timeout: Option<u64>,
                               id: Digest) -> Result<bool> {

        let resource_type = ResourceType::UnspentCoin;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            LookupRequest::new(session, resource_type, id)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = LookupResponse::from_bytes(&_res)?;

        res.validate()?;

        Ok(res.found)
    }

    /// Put an unspent coin to the node.
    pub fn put_unspent_coin(&mut self,
                            addr: &str,
                            timeout: Option<u64>,
                            read_timeout: Option<u64>,
                            write_timeout: Option<u64>,
                            source: CoinSource,
                            source_id: Digest,
                            coin: &Coin) -> Result<()> {
        coin.validate()?;

        let resource_type = ResourceType::UnspentCoin;
        let resource = coin.to_bytes()?;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            PutRequest::new(session,
                            resource_type,
                            Some(source),
                            Some(source_id),
                            None,
                            &resource)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = PutResponse::from_bytes(&_res)?;

        res.validate()
    }
}
