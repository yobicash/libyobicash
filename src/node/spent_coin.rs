// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `spent_coin` module provides the Yobicash node spent coin methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use crypto::Digest;
use store::Store;
use models::{CoinSource, Coin};
use node::{Node, NodePrefix};

impl <S: Store> Node<S> {
    /// Lists the node spent `Coins`.
    pub fn list_spent_coins(&self) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::SpentCoin as u8;
    
        self.list::<Coin>(prefix)
    }

    /// Samples the node spent `Coins`.
    pub fn sample_spent_coins(&self, count: u32) -> Result<Vec<Digest>> {
        let prefix = NodePrefix::SpentCoin as u8;
    
        self.sample::<Coin>(prefix, count)
    }

    /// Looks up a node spent `Coin`.
    pub fn lookup_spent_coin(&self, id: Digest) -> Result<bool> {
        let prefix = NodePrefix::SpentCoin as u8;

        self.lookup::<Coin>(prefix, id)
    }

    /// Gets a node spent `Coin`.
    pub fn get_spent_coin(&self, id: Digest) -> Result<Coin> {
        let prefix = NodePrefix::SpentCoin as u8;

        let coin = self.get::<Coin>(prefix, id)?;

        coin.validate()?;

        Ok(coin)
    }

    /// Adds a node spent `Coin`.
    pub fn add_spent_coin(&mut self, coin: &Coin) -> Result<()> {
        // check the network_type
        if coin.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        coin.validate()?;

        let id = coin.id;
        let source = coin.source;
        let source_id = coin.source_id;

        self.check_spent_coin_pre(source, source_id, id)?;

        self.del_unspent_coin(id)?;
        
        let prefix = NodePrefix::SpentCoin as u8;
        
        self.add::<Coin>(prefix, coin)
    }

    /// Deletes a node spent `Coin`.
    pub fn del_spent_coin(&mut self, id: Digest) -> Result<()> {
        let prefix = NodePrefix::SpentCoin as u8;

        self.del::<Coin>(prefix, id)
    }

    /// Checks the preconditions of a node spent `Coin`.
    pub fn check_spent_coin_pre(&self, source: CoinSource, source_id: Digest, id: Digest) -> Result<()> {
        // the source should exist in the store
        // the coin should exist in the source
        self.check_coin_source(source, source_id, id)?;
        
        // the coin should exist in the store
        if self.lookup_unspent_coin(id)? {
            return Err(ErrorKind::NotFound.into());
        }
        
        if !self.lookup_spent_coin(id)? {
            return Err(ErrorKind::NotFound.into());
        }

        Ok(())
    }

    /// Checks the postconditions of a node spent `Coin`.
    pub fn check_spent_coin_post(&self, source: CoinSource, source_id: Digest, id: Digest) -> Result<()> {
        // the source should exist in the store
        // the coin should exist in the source
        self.check_coin_source(source, source_id, id)?;
        
        // the coin should exist in the store
        if !self.lookup_spent_coin(id)? {
            return Err(ErrorKind::NotFound.into());
        }

        if self.lookup_unspent_coin(id)? {
            return Err(ErrorKind::InvalidStore.into());
        }

        Ok(())
    }

    /// Checks the node spent `Coin`s.
    pub fn check_spent_coins(&self) -> Result<()> {
        let ids = self.list_spent_coins()?;
        
        for id in ids {
            let coin = self.get_spent_coin(id)?;
            coin.validate()?;

            let id = coin.id;
            let source = coin.source;
            let source_id = coin.source_id;

            self.check_spent_coin_post(source, source_id, id)?;
        }

        Ok(())
    }

    /// Checks a sample of the node from the spent `Coin`s.
    pub fn check_spent_coins_sample(&self, count: u32) -> Result<()> {
        let ids = self.sample_spent_coins(count)?;
        
        for id in ids {
            let coin = self.get_spent_coin(id)?;
            coin.validate()?;

            let id = coin.id;
            let source = coin.source;
            let source_id = coin.source_id;

            self.check_spent_coin_post(source, source_id, id)?;
        }

        Ok(())
    }
}
