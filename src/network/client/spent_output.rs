// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `spent_output` module provides the Yobicash network client `SpentOutput` methods.

use result::Result;
use traits::{Validate, Serialize};
use crypto::Digest;
use crypto::BinarySerialize as CryptoBinarySerialize;
use models::Output;
use store::Store;
use node::Node;
use network::traits::Connection;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::*;
use network::client::Client;

use std::io::{Read, Write};

impl<S: Store, T: Connection + Read + Write> Client<S, T> {
    /// Lists a node spent outputs.
    pub fn list_spent_outputs(&mut self,
                             addr: &str,
                             timeout: Option<u64>,
                             read_timeout: Option<u64>,
                             write_timeout: Option<u64>) -> Result<Vec<Digest>> {

        let resource_type = ResourceType::SpentOutput;

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
    
    /// Samples a node spent outputs.
    pub fn sample_spent_outputs(&mut self,
                                addr: &str,
                                timeout: Option<u64>,
                                read_timeout: Option<u64>,
                                write_timeout: Option<u64>,
                                count: u32) -> Result<Vec<Digest>> {

        let resource_type = ResourceType::SpentOutput;

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

    /// Gets a node spent output.
    pub fn get_spent_output(&mut self,
               addr: &str,
               timeout: Option<u64>,
               read_timeout: Option<u64>,
               write_timeout: Option<u64>,
               id: Digest) -> Result<Output> {

        let resource_type = ResourceType::SpentOutput;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            GetRequest::new(session, resource_type, id)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = GetResponse::from_bytes(&_res)?;

        res.validate()?;

        Output::from_bytes(&res.resource)
    }

    /// Lookups a node spent output.
    pub fn lookup_spent_output(&mut self,
                               addr: &str,
                               timeout: Option<u64>,
                               read_timeout: Option<u64>,
                               write_timeout: Option<u64>,
                               id: Digest) -> Result<bool> {

        let resource_type = ResourceType::SpentOutput;

        let mut builder = |_node: &mut Node<S>, session: &Session| {
            LookupRequest::new(session, resource_type, id)?.to_bytes()
        };

        let _res = self.raw_send(addr, timeout, read_timeout, write_timeout, &mut builder)?;

        let res = LookupResponse::from_bytes(&_res)?;

        res.validate()?;

        Ok(res.found)
    }
}
