// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `unspent_output` module provides the Yobicash node unspent output methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use crypto::Digest;
use store::Store;
use models::{CoinSource, Output};
use node::{Node, NodePrefix};

impl <S: Store> Node<S> {
    /// Lists the node unspent `Outputs`.
    pub fn list_unspent_outputs(&self) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::UnspentOutput as u8;
    
        self.list::<Output>(prefix)
    }

    /// Samples the node unspent `Outputs`.
    pub fn sample_unspent_outputs(&self, count: u32) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::UnspentOutput as u8;
    
        self.sample::<Output>(prefix, count)
    }

    /// Looks up a node unspent `Output`.
    pub fn lookup_unspent_output(&self, id: Digest) -> Result<bool> {
        let prefix = NodePrefix::UnspentOutput as u8;

        self.lookup::<Output>(prefix, id)
    }

    /// Gets a node unspent `Output`.
    pub fn get_unspent_output(&self, id: Digest) -> Result<Output> {
        let prefix = NodePrefix::UnspentOutput as u8;

        let output = self.get::<Output>(prefix, id)?;

        output.validate()?;

        Ok(output)
    }

    /// Adds a node unspent `Output`.
    pub fn add_unspent_output(&mut self, source: CoinSource, source_id: Digest, output: &Output) -> Result<()> {
        // check the network_type
        if output.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        output.validate()?;

        let id = output.id;

        self.check_unspent_output_pre(source, source_id, id)?;
        
        let prefix = NodePrefix::UnspentOutput as u8;
        
        self.add::<Output>(prefix, output)
    }

    /// Deletes a node unspent `Output`.
    pub fn del_unspent_output(&mut self, id: Digest) -> Result<()> {
        let prefix = NodePrefix::UnspentOutput as u8;

        self.del::<Output>(prefix, id)
    }

    /// Checks the preconditions of a node unspent `Output`.
    pub fn check_unspent_output_pre(&self, source: CoinSource, source_id: Digest, id: Digest) -> Result<()> {
        // the source should exist in the store
        // the output should exist in the source
        self.check_output_source(source, source_id, id)?;

        // the output should not exist in the store
        if self.lookup_unspent_output(id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }

        if self.lookup_spent_output(id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }

        Ok(())
    }

    /// Checks the postconditions of a node unspent `Output`.
    pub fn check_unspent_output_post(&self, source: CoinSource, source_id: Digest, id: Digest) -> Result<()> {
        // the source should exist in the store
        // the output should exist in the source
        self.check_output_source(source, source_id, id)?;
        
        // the output should exist in the store
        if !self.lookup_unspent_output(id)? {
            return Err(ErrorKind::NotFound.into());
        }

        if self.lookup_spent_output(id)? {
            return Err(ErrorKind::InvalidStore.into());
        }
    
        Ok(())
    }

    /// Checks the node from the unspent `Output`s.
    pub fn check_unspent_outputs(&self) -> Result<()> {
        let ids = self.list_unspent_outputs()?;
        
        for id in ids {
            let output = self.get_unspent_output(id)?;
            output.validate()?;

            if self.lookup_spent_output(output.id)? {
                return Err(ErrorKind::InvalidStore.into());
            }
        }

        Ok(())
    }

    /// Checks a sample of the node from the unspent `Output`s.
    pub fn check_unspent_outputs_sample(&self, count: u32) -> Result<()> {
        let ids = self.sample_unspent_outputs(count)?;
        
        for id in ids {
            let output = self.get_unspent_output(id)?;
            output.validate()?;

            if self.lookup_spent_output(output.id)? {
                return Err(ErrorKind::InvalidStore.into());
            }
        }

        Ok(())
    }
}
