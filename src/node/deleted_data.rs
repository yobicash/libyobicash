// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `deleted_data` module provides the Yobicash node deleted data methods.

use error::ErrorKind;
use result::Result;
use crypto::Digest;
use store::Store;
use node::Node;

impl <S: Store> Node<S> {
    /// Checks the preconditions of a node deleted `Data`.
    pub fn check_deleted_data_pre(&self, write_id: Digest, id: Digest, size: u32) -> Result<()> {
        // the write_op should exist in the store
        let write_op = self.get_write_op(write_id)?;

        // the data should exist in the write_op
        if write_op.data_id != id {
            return Err(ErrorKind::InvalidID.into());
        }

        if write_op.data_size != size {
            return Err(ErrorKind::InvalidLength.into());
        }

        if write_op.is_expired() {
            return Err(ErrorKind::InvalidTime.into());
        }

        // the data should exist in the store
        if !self.lookup_undeleted_data(id)? {
            return Err(ErrorKind::NotFound.into());
        }

        Ok(())
    }
    
    /// Checks the postconditions of a node deleted `Data`.
    pub fn check_deleted_data_post(&self, write_id: Digest, id: Digest, size: u32) -> Result<()> {
        // the write_op should exist in the store
        let write_op = self.get_write_op(write_id)?;

        // the data should exist in the write_op
        if write_op.data_id != id {
            return Err(ErrorKind::InvalidID.into());
        }

        if write_op.data_size != size {
            return Err(ErrorKind::InvalidLength.into());
        }

        if !write_op.is_expired() {
            return Err(ErrorKind::InvalidTime.into());
        }

        // the data should not exist in the store
        if self.lookup_undeleted_data(id)? {
            return Err(ErrorKind::InvalidStore.into());
        }
        
        Ok(())
    }
}
