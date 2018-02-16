// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `spent_output` module provides the Yobicash node spent output methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use crypto::Digest;
use store::Store;
use models::{CoinSource, Output};
use node::{Node, NodePrefix};

impl <S: Store> Node<S> {
    /// Lists the node spent `Outputs`.
    pub fn list_spent_outputs(&self) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::SpentOutput as u8;
    
        self.list::<Output>(prefix)
    }

    /// Samples the node spent `Outputs`.
    pub fn sample_spent_outputs(&self, count: u32) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::SpentOutput as u8;
    
        self.sample::<Output>(prefix, count)
    }

    /// Looks up a node spent `Output`.
    pub fn lookup_spent_output(&self, id: Digest) -> Result<bool> {
        let prefix = NodePrefix::SpentOutput as u8;

        self.lookup::<Output>(prefix, id)
    }

    /// Gets a node spent `Output`.
    pub fn get_spent_output(&self, id: Digest) -> Result<Output> {
        let prefix = NodePrefix::SpentOutput as u8;

        let output = self.get::<Output>(prefix, id)?;

        output.validate()?;

        Ok(output)
    }

    /// Adds a node spent `Output`.
    pub fn add_spent_output(&mut self, source: CoinSource, source_id: Digest, output: &Output) -> Result<()> {
        // check the network_type
        if output.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        output.validate()?;

        let id = output.id;

        self.check_spent_output_pre(source, source_id, id)?;

        self.del_unspent_output(id)?;
        
        let prefix = NodePrefix::SpentOutput as u8;
        
        self.add::<Output>(prefix, output)
    }
    
    /// Checks the preconditions of a node spent `Output`.
    pub fn check_spent_output_pre(&self, source: CoinSource, source_id: Digest, id: Digest) -> Result<()> {
        // the source should exist in the store
        // the output should exist in the source
        self.check_output_source(source, source_id, id)?;
        
        // the output should exist in the store
        if !self.lookup_spent_output(id)? {
            return Err(ErrorKind::NotFound.into());
        }
        
        if !self.lookup_spent_output(id)? {
            return Err(ErrorKind::NotFound.into());
        }

        Ok(())
    }

    /// Checks the postconditions of a node spent `Output`.
    pub fn check_spent_output_post(&self, source: CoinSource, source_id: Digest, id: Digest) -> Result<()> {
        // the source should exist in the store
        // the output should exist in the source
        self.check_output_source(source, source_id, id)?;
        
        // the output should exist in the store
        if !self.lookup_spent_output(id)? {
            return Err(ErrorKind::NotFound.into());
        }

        if self.lookup_unspent_output(id)? {
            return Err(ErrorKind::InvalidStore.into());
        }

        Ok(())
    }

    /// Checks the node spent `Output`s.
    pub fn check_spent_outputs(&self) -> Result<()> {
        let ids = self.list_spent_outputs()?;
        
        for id in ids {
            let output = self.get_spent_output(id)?;
            output.validate()?;

            if self.lookup_unspent_output(output.id)? {
                return Err(ErrorKind::InvalidStore.into());
            }
        }

        Ok(())
    }

    /// Checks a sample of the node from the spent `Output`s.
    pub fn check_spent_outputs_sample(&self, count: u32) -> Result<()> {
        let ids = self.sample_spent_outputs(count)?;
        
        for id in ids {
            let output = self.get_spent_output(id)?;
            output.validate()?;

            if self.lookup_unspent_output(output.id)? {
                return Err(ErrorKind::InvalidStore.into());
            }
        }

        Ok(())
    }
}
