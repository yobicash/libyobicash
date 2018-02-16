// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `undeleted_data` module provides the Yobicash node undeleted data methods.

use error::ErrorKind;
use result::Result;
use traits::{Validate, Serialize};
use crypto::Digest;
use store::Store;
use models::Data;
use node::{Node, NodePrefix};

impl<S: Store> Node<S> {
    /// Lists the node undeleted `Datas`.
    pub fn list_undeleted_data(&self) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::UndeletedData as u8;
    
        self.list::<Data>(prefix)
    }

    /// Samples the node undeleted `Datas`.
    pub fn sample_undeleted_data(&self, count: u32) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::UndeletedData as u8;
    
        self.sample::<Data>(prefix, count)
    }

    /// Looks up a node undeleted `Data`.
    pub fn lookup_undeleted_data(&self, id: Digest) -> Result<bool> {
        let prefix = NodePrefix::UndeletedData as u8;

        self.lookup::<Data>(prefix, id)
    }

    /// Gets a node undeleted `Data`.
    pub fn get_undeleted_data(&self, id: Digest) -> Result<Data> {
        let prefix = NodePrefix::UndeletedData as u8;

        let data = self.get::<Data>(prefix, id)?;

        data.validate()?;

        Ok(data)
    }

    /// Adds a node undeleted `Data`.
    pub fn add_undeleted_data(&mut self, write_id: Digest, data: &Data) -> Result<()> {
        // check the network_type
        if data.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        data.validate()?;

        let id = data.id;
        let size = data.to_bytes()?.len() as u32;

        self.check_undeleted_data_pre(write_id, id, size)?;
        
        let prefix = NodePrefix::UndeletedData as u8;
        
        self.add::<Data>(prefix, data)
    }

    /// Deletes a node undeleted `Data`.
    pub fn del_undeleted_data(&mut self, id: Digest) -> Result<()> {
        let prefix = NodePrefix::UndeletedData as u8;

        self.del::<Data>(prefix, id)
    }

    /// Checks the preconditions of a node undeleted `Data`.
    pub fn check_undeleted_data_pre(&self, write_id: Digest, id: Digest, size: u32) -> Result<()> {
        // the write_op should exist in the store
        // the data should exist in the write_op
        self.check_data_source(write_id, id, size)?;

        // the data should not exist in the store
        if self.lookup_undeleted_data(id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }

        Ok(())
    }

    /// Checks the postconditions of a node undeleted `Data`.
    pub fn check_undeleted_data_post(&self, write_id: Digest, id: Digest, size: u32) -> Result<()> {
        // the write_op should exist in the store
        // the data should exist in the write_op
        self.check_data_source(write_id, id, size)?;

        // the data should exist in the store
        if !self.lookup_undeleted_data(id)? {
            return Err(ErrorKind::NotFound.into());
        }

        Ok(())
    }

    /// Checks the node from the undeleted `Data`s.
    pub fn check_undeleted_data(&self) -> Result<()> {
        let ids = self.list_undeleted_data()?;
        
        for id in ids {
            let data = self.get_undeleted_data(id)?;
            data.validate()?;
        }

        Ok(())
    }

    /// Checks a sample of the node from the undeleted `Data`s.
    pub fn check_undeleted_data_sample(&self, count: u32) -> Result<()> {
        let ids = self.sample_undeleted_data(count)?;
        
        for id in ids {
            let data = self.get_undeleted_data(id)?;
            data.validate()?;
        }

        Ok(())
    }
}
