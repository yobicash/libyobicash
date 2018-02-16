// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `transaction` module provides the Yobicash node transaction methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use crypto::Digest;
use store::Store;
use models::Transaction;
use node::{Node, NodePrefix};

impl <S: Store> Node<S> {
    /// Lists the node `Transaction`s.
    pub fn list_transactions(&self) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::Transaction as u8;
    
        self.list::<Transaction>(prefix)
    }

    /// Samples the node `Transaction`s.
    pub fn sample_transactions(&self, count: u32) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::Transaction as u8;
    
        self.sample::<Transaction>(prefix, count)
    }

    /// Looks up a node `Transaction`.
    pub fn lookup_transaction(&self, id: Digest) -> Result<bool> {
        let prefix = NodePrefix::Transaction as u8;

        self.lookup::<Transaction>(prefix, id)
    }

    /// Gets a node `Transaction`.
    pub fn get_transaction(&self, id: Digest) -> Result<Transaction> {
        let prefix = NodePrefix::Transaction as u8;

        let transaction = self.get::<Transaction>(prefix, id)?;

        transaction.validate()?;

        Ok(transaction)
    }

    /// Adds a node `Transaction`.
    pub fn add_transaction(&mut self, transaction: &Transaction) -> Result<()> {
        // check the network_type
        if transaction.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        // validate the transaction
        transaction.validate()?;

        // check the transaction preconditions
        self.check_transaction_pre(&transaction)?;

        // store the transaction in the store
        let prefix = NodePrefix::Transaction as u8;

        self.add::<Transaction>(prefix, transaction)?;

        Ok(())
    }

    /// Checks the preconditions of a node `Transaction`.
    pub fn check_transaction_pre(&self, transaction: &Transaction) -> Result<()> {
        // the inputs outputs sources should exist in the store
        // the inputs outputs should be unspent
        self.check_inputs_pre(&transaction.inputs)?;

        // the transaction should not exist in the store 
        if self.lookup_transaction(transaction.id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }

        // the outputs should not exist in the store
        for id in transaction.outputs_ids.clone() {
            if self.lookup_unspent_output(id)? {
                return Err(ErrorKind::AlreadyFound.into());
            }
            
            if self.lookup_spent_output(id)? {
                return Err(ErrorKind::AlreadyFound.into());
            }
        }

        // the fee should not exist in the store
        let fee_id = transaction.fee.id;

        if self.lookup_unspent_output(fee_id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }

        if self.lookup_spent_output(fee_id)? {
            return Err(ErrorKind::AlreadyFound.into());
        }

        Ok(())
    }

    /// Checks the postconditions of a node `Transaction`.
    pub fn check_transaction_post(&self, transaction: &Transaction) -> Result<()> {
        // the inputs outputs sources should exist in the store
        // the inputs outputs should be spent
        self.check_inputs_post(&transaction.inputs)?;

        // the transaction should exist in the store
        if !self.lookup_transaction(transaction.id)? {
            return Err(ErrorKind::NotFound.into());
        }

        // the fee should exist in the store
        let fee_id = transaction.fee.id;

        if !(self.lookup_unspent_output(fee_id)? ^
             self.lookup_spent_output(fee_id)?) {
            return Err(ErrorKind::InvalidStore.into());
        }
        
        Ok(())
    }

    /// Checks the node `Transaction`s.
    pub fn check_transactions(&self) -> Result<()> {
        let ids = self.list_transactions()?;

        for id in ids {
            let transaction = self.get_transaction(id)?; 
            self.check_transaction_post(&transaction)?;
        }

        Ok(())
    }

    /// Checks a sample of the node `Transaction`s.
    pub fn check_transactions_sample(&self, count: u32) -> Result<()> {
        let ids = self.sample_transactions(count)?;

        for id in ids {
            let transaction = self.get_transaction(id)?; 
            self.check_transaction_post(&transaction)?;
        }

        Ok(())
    }
}
