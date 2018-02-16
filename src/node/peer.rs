// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `peer` module provides the Yobicash node undeleted data methods.

use result::Result;
use traits::Validate;
use store::Store;
use models::Peer;
use node::{Node, NodePrefix};

impl <S: Store> Node<S> {
    /// Lists the node `Peer`s.
    pub fn list_peers(&self) -> Result<Vec<String>> {
        let prefix = NodePrefix::Peer as u8;
    
        self.list::<Peer>(prefix)
    }

    /// Samples the node `Peer`s.
    pub fn sample_peers(&self, count: u32) -> Result<Vec<String>> {
        let prefix = NodePrefix::Peer as u8;
    
        self.sample::<Peer>(prefix, count)
    }

    /// Looks up a node `Peer`.
    pub fn lookup_peer(&self, address: &str) -> Result<bool> {
        let prefix = NodePrefix::Peer as u8;

        self.lookup::<Peer>(prefix, String::from(address))
    }

    /// Gets a node `Peer`.
    pub fn get_peer(&self, address: &str) -> Result<Peer> {
        let prefix = NodePrefix::Peer as u8;

        self.get::<Peer>(prefix, String::from(address))
    }

    /// Adds a node `Peer`.
    pub fn add_peer(&mut self, peer: &Peer) -> Result<()> {
        // validate the peer
        peer.validate()?;
        
        // store the peer in the store
        let prefix = NodePrefix::Peer as u8;
        
        self.add::<Peer>(prefix, peer)
    }

    /// Deletes a node `Peer`.
    pub fn del_peer(&self, address: &str) -> Result<bool> {
        let prefix = NodePrefix::Peer as u8;

        self.lookup::<Peer>(prefix, String::from(address))
    }

    /// Checks the node `Peer`s.
    pub fn check_peers(&self) -> Result<()> {
        let addresses = self.list_peers()?;

        for address in addresses.clone() {
            let peer = self.get_peer(&address)?; 
            peer.validate()?;
        }

        Ok(())
    }

    /// Checks a sample of the node `Peer`s.
    pub fn check_peers_sample(&self, count: u32) -> Result<()> {
        let addresses = self.sample_peers(count)?;

        for address in addresses.clone() {
            let peer = self.get_peer(&address)?; 
            peer.validate()?;
        }

        Ok(())
    }
}
