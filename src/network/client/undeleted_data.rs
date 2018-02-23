// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `undeleted_data` module provides the Yobicash network client `UndeletedData` methods.

use result::Result;
use traits::{Validate, Serialize};
use crypto::Digest;
use crypto::BinarySerialize as CryptoBinarySerialize;
use models::Data;
use store::Store;
use node::Node;
use network::traits::Connection;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::*;
use network::client::Client;

use std::io::{Read, Write};

impl<S: Store, T: Connection + Read + Write> Client<S, T> {
    /// Lists a node undeleted data.
    pub fn list_undeleted_data(&mut self,
                             addr: &str,
                             timeout: Option<u64>,
                             read_timeout: Option<u64>,
                             write_timeout: Option<u64>) -> Result<Vec<Digest>> {

        let resource_type = ResourceType::UndeletedData;

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
    
    /// Samples a node undeleted data.
    pub fn sample_undeleted_data(&mut self,
                                addr: &str,
                                timeout: Option<u64>,
                                read_timeout: Option<u64>,
                                write_timeout: Option<u64>,
                                count: u32) -> Result<Vec<Digest>> {

        let resource_type = ResourceType::UndeletedData;

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

    /// Gets a node undeleted data.
    pub fn get_undeleted_data(&mut self,
               addr: &str,
               timeout: Option<u64>,
               read_timeout: Option<u64>,
               write_timeout: Option<u64>,
               id: Digest) -> Result<Data> {

        let resource_type = ResourceType::UndeletedData;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            GetRequest::new(session, resource_type, id)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = GetResponse::from_bytes(&_res)?;

        res.validate()?;

        Data::from_bytes(&res.resource)
    }

    /// Lookups a node undeleted data.
    pub fn lookup_undeleted_data(&mut self,
                                 addr: &str,
                                 timeout: Option<u64>,
                                 read_timeout: Option<u64>,
                                 write_timeout: Option<u64>,
                                 id: Digest) -> Result<bool> {

        let resource_type = ResourceType::UndeletedData;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            LookupRequest::new(session, resource_type, id)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = LookupResponse::from_bytes(&_res)?;

        res.validate()?;

        Ok(res.found)
    }

    /// Put an undeleted data to the node.
    pub fn put_undeleted_data(&mut self,
                              addr: &str,
                              timeout: Option<u64>,
                              read_timeout: Option<u64>,
                              write_timeout: Option<u64>,
                              write_id: Digest,
                              data: &Data) -> Result<()> {
        data.validate()?;

        let resource_type = ResourceType::UndeletedData;
        let resource = data.to_bytes()?;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            PutRequest::new(session,
                            resource_type,
                            None,
                            None,
                            Some(write_id),
                            &resource)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = PutResponse::from_bytes(&_res)?;

        res.validate()
    }
}
