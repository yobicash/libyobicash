// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `input` module provides the Yobicash node input methods.

use error::ErrorKind;
use result::Result;
use store::Store;
use models::Input;
use node::Node;

impl <S: Store> Node<S> {
    /// Checks the preconditions of node `Input`s.
    pub fn check_inputs_pre(&self, inputs: &[Input]) -> Result<()> {
        // the inputs outputs sources should exist in the store
        // the inputs outputs should be unspent
        // the inputs outputs should verify the inputs
        for input in inputs.clone() {
            let source = input.source;
            let source_id = input.source_id;
            let id = input.id;

            let output = self.get_unspent_output(id)?;
            if output.verify(input)? {
                return Err(ErrorKind::InvalidProof.into());
            }

            self.check_spent_output_pre(source, source_id, id)?;
        }

        Ok(())
    }

    /// Checks the postconditions of node type `Input`s.
    pub fn check_inputs_post(&self, inputs: &[Input]) -> Result<()> {
        // the inputs outputs sources should exist in the store
        // the inputs outputs should be spent
        // the inputs outputs should verify the inputs
        for input in inputs.clone() {
            let source = input.source;
            let source_id = input.source_id;
            let id = input.id;

            let output = self.get_spent_output(id)?;
            if output.verify(input)? {
                return Err(ErrorKind::InvalidProof.into());
            }

            self.check_spent_output_post(source, source_id, id)?;
        }

        Ok(())
    }
}
