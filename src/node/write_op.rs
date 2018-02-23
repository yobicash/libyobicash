// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `write_op` module provides the Yobicash node write operation methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use crypto::Digest;
use store::Store;
use models::WriteOp;
use node::{Node, NodePrefix};

impl <S: Store> Node<S> {
    /// Lists the node `WriteOp`s.
    pub fn list_write_ops(&self) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::WriteOp as u8;
    
        self.list::<WriteOp>(prefix)
    }

    /// Samples the node `WriteOp`s.
    pub fn sample_write_ops(&self, count: u32) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::WriteOp as u8;
    
        self.sample::<WriteOp>(prefix, count)
    }

    /// Looks up a node `WriteOp`.
    pub fn lookup_write_op(&self, id: Digest) -> Result<bool> {
        let prefix = NodePrefix::WriteOp as u8;

        self.lookup::<WriteOp>(prefix, id)
    }

    /// Gets a node `WriteOp`.
    pub fn get_write_op(&self, id: Digest) -> Result<WriteOp> {
        let prefix = NodePrefix::WriteOp as u8;

        self.get::<WriteOp>(prefix, id)
    }

    /// Adds a node `WriteOp`.
    pub fn add_write_op(&mut self, write_op: &WriteOp) -> Result<()> {
        // check the network_type
        if write_op.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        // validate the write_op
        write_op.validate()?;

        if write_op.is_expired() {
            return Err(ErrorKind::InvalidTime.into());
        }

        // check the write_op preconditions
        self.check_write_op_pre(write_op)?;

        // store the write_op in the store
        let prefix = NodePrefix::WriteOp as u8;

        self.add::<WriteOp>(prefix, write_op)?;

        Ok(())
    }

    /// Checks the preconditions of a node `WriteOp`.
    pub fn check_write_op_pre(&self, write_op: &WriteOp) -> Result<()> {
        // the inputs outputs sources should exist in the store
        // the inputs outputs should be unspent
        self.check_inputs_pre(&write_op.inputs)?;
        
        // the write_op should not exist in the store
        // the data should not exist in the store
        let id = write_op.id;
        let data_id = write_op.data_id;
        let data_size = write_op.data_size;

        self.check_undeleted_data_pre(id, data_id, data_size)?;

        // the fee should not exist in the store
        let fee_id = write_op.fee.id;

        if self.lookup_unspent_output(fee_id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }

        if self.lookup_spent_output(fee_id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }

        Ok(())
    }

    /// Checks the postconditions of a node `WriteOp`.
    pub fn check_write_op_post(&self, write_op: &WriteOp) -> Result<()> {
        // the inputs outputs sources should exist in the store
        // the inputs outputs should be spent
        self.check_inputs_post(&write_op.inputs)?;
        
        // the write_op should exist in the store
        // the data should exist in the write_op
        let write_id = write_op.id;
        let data_id = write_op.data_id;
        let data_size = write_op.data_size;

        self.check_undeleted_data_post(write_id, data_id, data_size)?;
        
        // the fee should exist in the store
        let fee_id = write_op.fee.id;

        if !(self.lookup_unspent_output(fee_id)? ^
             self.lookup_spent_output(fee_id)?) {
            return Err(ErrorKind::InvalidStore.into());
        }

        Ok(())
    }

    /// Checks the node `WriteOp`s.
    pub fn check_write_ops(&self) -> Result<()> {
        let ids = self.list_write_ops()?;

        for id in ids {
            let write_op = self.get_write_op(id)?; 
            self.check_write_op_post(&write_op)?;
        }

        Ok(())
    }

    /// Checks a sample of the node `WriteOp`s.
    pub fn check_write_ops_sample(&self, count: u32) -> Result<()> {
        let ids = self.sample_write_ops(count)?;

        for id in ids {
            let write_op = self.get_write_op(id)?; 
            self.check_write_op_post(&write_op)?;
        }

        Ok(())
    }
}
