// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `delete_op` module provides the Yobicash node delete operation methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use crypto::Digest;
use store::Store;
use models::DeleteOp;
use node::{Node, NodePrefix};

impl <S: Store> Node<S> {
    /// Lists the node `DeleteOp`s.
    pub fn list_delete_ops(&self) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::DeleteOp as u8;
    
        self.list::<DeleteOp>(prefix)
    }

    /// Samples the node `DeleteOp`s.
    pub fn sample_delete_ops(&self, count: u32) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::DeleteOp as u8;
    
        self.sample::<DeleteOp>(prefix, count)
    }

    /// Looks up a node `DeleteOp`.
    pub fn lookup_delete_op(&self, id: Digest) -> Result<bool> {
        let prefix = NodePrefix::DeleteOp as u8;

        self.lookup::<DeleteOp>(prefix, id)
    }

    /// Gets a node `DeleteOp`.
    pub fn get_delete_op(&self, id: Digest) -> Result<DeleteOp> {
        let prefix = NodePrefix::DeleteOp as u8;

        self.get::<DeleteOp>(prefix, id)
    }

    /// Adds a node `DeleteOp`.
    pub fn add_delete_op(&mut self, delete_op: &DeleteOp) -> Result<()> {
        // check the network_type
        if delete_op.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        // validate the delete_op
        delete_op.validate()?;

        // check the delete_op preconditions
        self.check_delete_op_pre(delete_op)?;

        // store the delete_op in the store
        let prefix = NodePrefix::DeleteOp as u8;
        
        self.add::<DeleteOp>(prefix, delete_op)?;

        Ok(())
    }

    /// Checks the preconditions of a node `DeleteOp`.
    pub fn check_delete_op_pre(&self, delete_op: &DeleteOp) -> Result<()> {
        // the inputs outputs sources should exist in the store
        // the inputs outputs should be unspent
        self.check_inputs_pre(&delete_op.inputs)?;
        
        // the delete_op should not exist in the store
        if self.lookup_delete_op(delete_op.id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }
        
        // the write_op should exist in the store and should verify
        let write_id = delete_op.write_id;
        
        let write_op = self.get_write_op(write_id)?;
        if !delete_op.verify(&write_op)? {
            return Err(ErrorKind::InvalidProof.into());
        }
        
        // the data should exist in the write_op
        let data_id = delete_op.data_id;

        if write_op.data_id != data_id {
            return Err(ErrorKind::NotFound.into());
        }
        
        // the data should exist
        if !self.lookup_undeleted_data(data_id)? {
            return Err(ErrorKind::NotFound.into());
        }
        
        // the fee should not exist in the store
        let fee_id = delete_op.fee.id;

        if self.lookup_unspent_output(fee_id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }

        if self.lookup_spent_output(fee_id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }

        Ok(())
    }

    /// Checks the postconditions of a node `DeleteOp`.
    pub fn check_delete_op_post(&self, delete_op: &DeleteOp) -> Result<()> {
        // the inputs outputs sources should exist in the store
        // the inputs outputs should be spent
        self.check_inputs_post(&delete_op.inputs)?;
        
        // the delete_op should exist in the store
        if !self.lookup_delete_op(delete_op.id)? {
            return Err(ErrorKind::NotFound.into());
        }
        
        // the write_op should exist in the store
        // the data should exist in the write_op
        // the data should be deleted
        let write_id = delete_op.write_id;
        let data_id = delete_op.data_id;
        let data_size = delete_op.data_size;

        self.check_deleted_data_post(write_id, data_id, data_size)?;
        
        // the fee should exist in the store
        let fee_id = delete_op.fee.id;

        if !(self.lookup_unspent_output(fee_id)? ^
             self.lookup_spent_output(fee_id)?) {
            return Err(ErrorKind::InvalidStore.into());
        }
        
        Ok(())
    }

    /// Checks the node `DeleteOp`s.
    pub fn check_delete_ops(&self) -> Result<()> {
        let ids = self.list_delete_ops()?;

        for id in ids {
            let delete_op = self.get_delete_op(id)?; 
            self.check_delete_op_post(&delete_op)?;
        }

        Ok(())
    }

    /// Checks a sample of the node `DeleteOp`s.
    pub fn check_delete_ops_sample(&self, count: u32) -> Result<()> {
        let ids = self.sample_delete_ops(count)?;

        for id in ids {
            let delete_op = self.get_delete_op(id)?; 
            self.check_delete_op_post(&delete_op)?;
        }

        Ok(())
    }
}
