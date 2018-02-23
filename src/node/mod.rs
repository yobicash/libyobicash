// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `node` module provides the Yobicash node types and methods.

use result::Result;
use crypto::{Scalar, ZKPWitness, Key};
use utils::NetworkType;
use models::{Coin, CoinSource, Transaction, Peer};
use store::Store;

use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

mod utils;
mod peer;
mod coin_source;
mod unspent_coin;
mod spent_coin;
mod output_source;
mod unspent_output;
mod spent_output;
mod data_source;
mod undeleted_data;
mod deleted_data;
mod input;
mod transaction;
mod write_op;

/// The store mode of the node.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum NodeMode {
    /// Memory only mode.
    Memory=0,
    /// Persistent mode.
    Persistent=1,
}

/// The node store prefixes.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum NodePrefix {
    /// Node peer store prefix.
    Peer=0,
    /// Node transaction store prefix.
    Transaction=1,
    /// Node write operation store prefix.
    WriteOp=2,
    /// Node unspent coin.
    UnspentCoin=3,
    /// Node spent coin.
    SpentCoin=4,
    /// Node unspent output.
    UnspentOutput=5,
    /// Node spent output.
    SpentOutput=6,
    /// Node undeleted data.
    UndeletedData=7,
}

/// The node check mode.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum NodeCheckMode {
    /// Check only the node frontier.
    Frontier,
    /// Check a sample of the node.
    Sample(u32),
    /// Check both the frontier and a sample.
    FrontierAndSample(u32),
    /// Check all the node.
    All,
}

/// The type used to represent the Yobicash node.
#[derive(Clone, Debug)]
pub struct Node<S: Store> {
    /// The protocol network type.
    network_type: NetworkType,
    /// The mode of the node.
    mode: NodeMode,
    /// The node encryption key.
    key: Key,
    /// The daghchain store.
    store: Arc<Mutex<S>>,
    /// The store max size.
    max_size: u32,
}

impl<S: Store> Node<S> {
    /// Init the node. If it is a regtest node,
    /// init returns the genesis coin.
    pub fn init(&mut self, check_mode: NodeCheckMode, seed: Vec<SocketAddr>) -> Result<()> {
        let network_type = self.network_type;

        // if 0: empty
        // then 0: create and add the genesis:
        // then 0: if 1: regtest, then: create the associated coin
        // else 0: on
        if self.is_empty()? {
            match network_type {
                NetworkType::RegTest => {
                    let instance = Scalar::random();
                    let witness = ZKPWitness::new(instance)?;
                    let transaction = Transaction::new_regtest_genesis(witness)?;
                    let output = transaction.fee.clone();
                    self.add_transaction(&transaction)?;
                    
                    let source = CoinSource::TransactionFee;
                    let source_id = transaction.id;
                    let coin = Coin::new(source, source_id, &output, instance)?;
                    self.add_unspent_coin(&coin)?;
                },
                NetworkType::TestNet => {
                    let transaction = Transaction::new_testnet_genesis()?;
                    self.add_transaction(&transaction)?;
                },
                NetworkType::MainNet => {
                    let transaction = Transaction::new_mainnet_genesis()?;
                    self.add_transaction(&transaction)?;
                },
            }
        }

        for address in seed.clone() {
            let peer = Peer::new(&format!("{}", address))?;
            self.add_peer(&peer)?;
        }
            
        match check_mode {
            NodeCheckMode::Frontier => {
                self.check_frontier()?;
            },
            NodeCheckMode::Sample(count) => {
                self.check_sample(count)?;
            },
            NodeCheckMode::FrontierAndSample(count) => {
                self.check_frontier_and_sample(count)?;
            },
            NodeCheckMode::All => {
                self.check_all()?;
            }
        }
       
        Ok(())
    }
    
    /// Checks the frontier of the node.
    pub fn check_frontier(&self) -> Result<()> {
        self.check_unspent_coins()?;
        self.check_unspent_outputs()?;
        self.check_undeleted_data()?;

        Ok(())
    }

    /// Checks a sample of the frontier of the node.
    pub fn check_frontier_sample(&self, count: u32) -> Result<()> {
        self.check_unspent_coins_sample(count)?;
        self.check_unspent_outputs_sample(count)?;
        self.check_undeleted_data_sample(count)?;

        Ok(())
    }

    /// Checks a sample of the node.
    pub fn check_sample(&self, count: u32) -> Result<()> {
        self.check_frontier_sample(count)?;
        self.check_spent_coins_sample(count)?;
        self.check_spent_outputs_sample(count)?;
        self.check_transactions_sample(count)?;
        self.check_write_ops_sample(count)?;
        self.check_peers_sample(count)?;

        Ok(())
    }

    /// Checks the frontier and a sample of the node.
    pub fn check_frontier_and_sample(&self, count: u32) -> Result<()> {
        self.check_frontier()?;
        self.check_spent_coins_sample(count)?;
        self.check_spent_outputs_sample(count)?;
        self.check_transactions_sample(count)?;
        self.check_write_ops_sample(count)?;
        self.check_peers_sample(count)?;

        Ok(())
    }

    /// Checks all the the node.
    pub fn check_all(&self) -> Result<()> {
        self.check_frontier()?;
        self.check_spent_coins()?;
        self.check_spent_outputs()?;
        self.check_transactions()?;
        self.check_write_ops()?;
        self.check_peers()?;

        Ok(())
    }
}
