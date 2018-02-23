// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `data_source` module provides the Yobicash node data source methods.

use error::ErrorKind;
use result::Result;
use crypto::Digest;
use store::Store;
use models::CoinSource;
use node::Node;

impl <S: Store> Node<S> {
    /// Checks a node `Data` write operation.
    pub fn check_data_source(&self, write_id: Digest, id: Digest, size: u32) -> Result<()> {
        // lookup write_op, if absent, stop
        if !self.lookup_write_op(write_id)? {
            return Err(ErrorKind::NotFound.into());
        }

        // get write_op, if data_id != id || data_size != size, stop
        let write_op = self.get_write_op(write_id)?;
        if write_op.data_id != id {
            return Err(ErrorKind::NotFound.into());
        }

        if write_op.data_size != size {
            return Err(ErrorKind::InvalidLength.into());
        }

        // check if the write_op is expired
        if write_op.is_expired() {
            return Err(ErrorKind::InvalidTime.into());
        }

        // check write_id fee
        self.check_output_source(CoinSource::WriteOpFee, write_id, write_op.fee.id)
    }
}
