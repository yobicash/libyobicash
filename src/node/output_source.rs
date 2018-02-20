// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `coin_source` module provides the Yobicash node coin source methods.

use error::ErrorKind;
use result::Result;
use crypto::Digest;
use store::Store;
use models::CoinSource;
use node::Node;

impl <S: Store> Node<S> {
    /// Looks up a node `Output` source.
    pub fn lookup_output_source(&self, source: CoinSource, source_id: Digest) -> Result<bool> {
        match source {
            CoinSource::TransactionFee => {
                if !self.lookup_transaction(source_id)? {
                    return Ok(false);
                }
            },
            CoinSource::TransactionOutput => {
                if !self.lookup_transaction(source_id)? {
                    return Ok(false);
                }
            },
            CoinSource::WriteOpFee => {
                if !self.lookup_write_op(source_id)? {
                    return Ok(false);
                }
            },
            CoinSource::DeleteOpFee => {
                if !self.lookup_delete_op(source_id)? {
                    return Ok(false);
                }
            },
        }

        Ok(true)
    }

    /// Checks a node `Output` source.
    pub fn check_output_source(&self, source: CoinSource, source_id: Digest, id: Digest) -> Result<()> {
        if !self.lookup_output_source(source, source_id)? {
            return Err(ErrorKind::NotFound.into());
        }
        
        match source {
            CoinSource::TransactionFee => {
                let tx = self.get_transaction(source_id)?;
                if tx.fee.id != id {
                    return Err(ErrorKind::NotFound.into());
                }
            },
            CoinSource::TransactionOutput => {
                let tx = self.get_transaction(source_id)?;
                let mut found = false;
                for _id in tx.outputs_ids {
                    if _id == id {
                        found = true;
                        break;
                    }
                }

                if !found {
                    return Err(ErrorKind::NotFound.into());
                }
            },
            CoinSource::WriteOpFee => {
                let write_op = self.get_write_op(source_id)?;
                if write_op.fee.id != id {
                    return Err(ErrorKind::NotFound.into());
                }
            },
            CoinSource::DeleteOpFee => {
                let delete_op = self.get_delete_op(source_id)?;
                if delete_op.fee.id != id {
                    return Err(ErrorKind::NotFound.into());
                }
            },
        }

        return Ok(())
    }
}