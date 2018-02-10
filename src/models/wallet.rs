// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://openscoinurce.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://openscoinurce.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `wallet` module provides the wallet types and methods.

use itertools::Itertools;
use serde_json as json;
use rmp_serde as messagepack;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Identify, Validate, HexSerialize, Serialize};
use utils::{NetworkType, Timestamp, Amount};
use crypto::Digest;
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use models::coin::Coin;

/// A wallet is a node on the Yobicash collection of coins used by a unique user.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Wallet {
    /// The name of the wallet.
    pub name: String,
    /// The protocol network type.
    pub network_type: NetworkType,
    /// The balance of the wallet.
    pub balance: Amount,
    /// The length of the unspent coins.
    pub ucoins_length: u32,
    /// The ids of the unspent coins.
    pub ucoins_ids: Vec<Digest>,
    /// The length of the spent coins.
    pub scoins_length: u32,
    /// The ids of the spent coins.
    pub scoins_ids: Vec<Digest>,
    /// The unix timestamp of the time the wallet has been created.
    pub created_at: Timestamp,
    /// The unix timestamp of the last time the wallet has been updated.
    pub updated_at: Timestamp,
}

impl Wallet {
    /// Creates a new `Wallet` from a name.
    pub fn new(name: &str) -> Wallet {
        let mut wallet = Wallet::default();
        wallet.name = name.to_owned();
        wallet
    }

    /// Add a new unspent coin.
    pub fn add_ucoin(&mut self, ucoin: &Coin) -> Result<()> {
        self.validate()?;

        ucoin.validate()?;
        if ucoin.output.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        let id = ucoin.output.id;

        for coin_id in self.ucoins_ids.clone() {
            if coin_id == id {
                return Err(ErrorKind::AlreadyFound.into());
            }
        }

        for coin_id in self.scoins_ids.clone() {
            if coin_id == id {
                return Err(ErrorKind::AlreadyFound.into());
            }
        }

        self.ucoins_ids.push(id);
        self.ucoins_length += 1;
        self.balance += ucoin.output.amount.clone();
        self.updated_at = Timestamp::now();
        
        Ok(())
    }

    /// Add a new spent coin.
    pub fn add_scoin(&mut self, scoin: &Coin) -> Result<()> {
        self.validate()?;
        
        scoin.validate()?;
        if scoin.output.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        let id = scoin.output.id;

        for coin_id in self.scoins_ids.clone() {
            if coin_id == id {
                return Err(ErrorKind::AlreadyFound.into());
            }
        }

        let mut found = false;

        for i in 0..self.ucoins_length as usize {
            let coin_id = self.ucoins_ids[i];
            if coin_id == id {
                found = true;
                self.scoins_ids.push(id);
                self.scoins_length += 1;
                self.balance -= scoin.output.amount.clone();
                self.updated_at = Timestamp::now();

            }
        }

        if !found {
            Err(ErrorKind::NotFound.into())
        } else {
            Ok(())
        }
    }
}

impl Default for Wallet {
    fn default() -> Wallet {
        let now = Timestamp::now();

        Wallet {
            name: String::new(),
            network_type: NetworkType::default(),
            balance: Amount::new(),
            ucoins_length: 0,
            ucoins_ids: Vec::new(),
            scoins_length: 0,
            scoins_ids: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl<'a> Identify<'a> for Wallet {
    type ID = String;

    fn id(&self) -> Result<Self::ID> {
        Ok(self.name.clone())
    }

    fn id_to_bytes(id: Self::ID) -> Result<Vec<u8>> {
        Ok(id.into_bytes())
    }

    fn id_from_bytes(b: &[u8]) -> Result<Self::ID> {
        let mut utf8 = Vec::new();
        utf8.extend_from_slice(b);

        Ok(String::from_utf8(utf8)?)
    }

    fn binary_id(&self) -> Result<Vec<u8>> {
        let id = self.id()?;

        Self::id_to_bytes(id)
    }

    fn id_from_string(s: &str) -> Result<Self::ID> {
        Ok(String::from(s))
    }

    fn id_to_string(id: Self::ID) -> Result<String> {
        Ok(id)
    }

    fn string_id(&self) -> Result<String> {
        Ok(self.name.clone())
    }
}

impl Validate for Wallet {
    fn validate(&self) -> Result<()> {
        if self.ucoins_length as usize != self.ucoins_ids.len() {
            return Err(ErrorKind::InvalidLength.into());
        }

        if self.scoins_length as usize != self.scoins_ids.len() {
            return Err(ErrorKind::InvalidLength.into());
        }

        let mut ucoins_binary_ids = Vec::new();
        for coin_id in self.ucoins_ids.clone() {
            ucoins_binary_ids.push(coin_id.to_bytes()?);
        }

        if ucoins_binary_ids.iter().unique().count() != self.ucoins_length as usize {
            return Err(ErrorKind::DuplicatesFound.into());
        }
        
        let mut scoins_binary_ids = Vec::new();
        for coin_id in self.scoins_ids.clone() {
            scoins_binary_ids.push(coin_id.to_bytes()?);
        }

        if scoins_binary_ids.iter().unique().count() != self.scoins_length as usize {
            return Err(ErrorKind::DuplicatesFound.into());
        }

        self.created_at.validate()?;
        self.updated_at.validate()?;
        

        if self.created_at > self.updated_at {
            return Err(ErrorKind::InvalidTimestamp.into());
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for Wallet {
    fn to_json(&self) -> Result<String> {
        let mut ucoins_ids_json = Vec::new();
        for coin_id in self.ucoins_ids.clone() {
            ucoins_ids_json.push(coin_id.to_hex()?);
        }
        
        let mut scoins_ids_json = Vec::new();
        for coin_id in self.scoins_ids.clone() {
            scoins_ids_json.push(coin_id.to_hex()?);
        }

        let obj = json!({
            "name": self.name,
            "network_type": self.network_type.to_hex()?,
            "balance": self.balance.to_string(),
            "ucoins_length": self.ucoins_length,
            "ucoins_ids": ucoins_ids_json,
            "scoins_length": self.scoins_length,
            "scoins_ids": scoins_ids_json,
            "created_at": self.created_at.to_string(),
            "updated_at": self.updated_at.to_string(),
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let name_value = obj["name"].clone();
        let name: String = json::from_value(name_value)?;

        let balance_value = obj["balance"].clone();
        let balance_str: String = json::from_value(balance_value)?;
        let balance = Amount::from_string(&balance_str)?;

        let network_type_value = obj["network_type"].clone();
        let network_type_hex: String = json::from_value(network_type_value)?;
        let network_type = NetworkType::from_hex(&network_type_hex)?;

        let ucoins_length_value = obj["ucoins_length"].clone();
        let ucoins_length: u32 = json::from_value(ucoins_length_value)?;

        let ucoins_ids_values = obj["ucoins_ids"].clone();
        let mut ucoins_ids = Vec::new();
        for i in 0..ucoins_length as usize {
            let coin_id_value = ucoins_ids_values[i].clone();
            let coin_id_hex: String = json::from_value(coin_id_value)?;
            ucoins_ids.push(Digest::from_hex(&coin_id_hex)?);
        }

        let scoins_length_value = obj["scoins_length"].clone();
        let scoins_length: u32 = json::from_value(scoins_length_value)?;

        let scoins_ids_values = obj["scoins_ids"].clone();
        let mut scoins_ids = Vec::new();
        for i in 0..scoins_length as usize {
            let coin_id_value = scoins_ids_values[i].clone();
            let coin_id_hex: String = json::from_value(coin_id_value)?;
            scoins_ids.push(Digest::from_hex(&coin_id_hex)?);
        }
        
        let created_at_value = obj["created_at"].clone();
        let created_at_str: String = json::from_value(created_at_value)?;
        let created_at = Timestamp::from_string(&created_at_str)?;
        
        let updated_at_value = obj["updated_at"].clone();
        let updated_at_str: String = json::from_value(updated_at_value)?;
        let updated_at = Timestamp::from_string(&updated_at_str)?;

        let wallet = Wallet {
            name: name,
            network_type: network_type,
            balance: balance,
            ucoins_length: ucoins_length,
            ucoins_ids: ucoins_ids,
            scoins_length: scoins_length,
            scoins_ids: scoins_ids,
            created_at: created_at,
            updated_at: updated_at,
        };

        Ok(wallet)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let wallet = messagepack::from_slice(b)?;

        Ok(wallet)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
