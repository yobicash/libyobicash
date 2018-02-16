// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `utils` module provides the Yobicash node utils methods.

use error::ErrorKind;
use result::Result;
use traits::{Identify, Serialize};
use store::{Store, StoreMode, StoreKey, StoreItem};
use node::{Node, NodeMode};

impl <S: Store> Node<S> {
    /// Returns the used store mode.
    pub fn mode(&self) -> StoreMode {
        if self.mode == NodeMode::Memory {
            StoreMode::Memory
        } else {
            StoreMode::Persistent
        }
    }
    
    /// Returns if the store is empty
    pub fn is_empty(&self) -> Result<bool> {
        let mode = self.mode();

        if self.store.lock()?.is_empty(mode)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns if the store is empty
    pub fn is_prefix_empty(&self, prefix: u8) -> Result<bool> {
        let mode = self.mode();

        if self.store.lock()?.is_prefix_empty(mode, prefix)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Returns the store size.
    pub fn size(&self) -> Result<u32> {
        let mode = self.mode();

        self.store.lock()?.size(mode)
    }

    /// Returns the store prefix size.
    pub fn prefix_size(&self, prefix: u8) -> Result<u32> {
        let mode = self.mode();

        self.store.lock()?.prefix_size(mode, prefix)
    }

    /// Lists a node resourse ids.
    pub fn list<'a, T: Identify<'a>>(&self, prefix: u8) -> Result<Vec<T::ID>> {
        let mode = self.mode();
        let store = self.store.lock()?;
       
        let keys = store.list_keys(mode, prefix)?;

        let mut ids = Vec::new();

        for key in keys {
            ids.push(key.to_id::<T>()?)
        }

        Ok(ids)
    }

    /// Samples a node resourse ids.
    pub fn sample<'a, T: Identify<'a>>(&self, prefix: u8, count: u32) -> Result<Vec<T::ID>> {
        let mode = self.mode();
        let store = self.store.lock()?;
       
        let keys = store.sample_keys(mode, prefix, count)?;

        let mut ids = Vec::new();

        for key in keys {
            ids.push(key.to_id::<T>()?)
        }

        Ok(ids)
    }

    /// Looks up a node resource id in the store.
    pub fn lookup<'a, T: Identify<'a>>(&self, prefix: u8, id: T::ID) -> Result<bool> {
        let mode = self.mode();
        let key = StoreKey::from_id::<T>(id)?;
        let store = self.store.lock()?;

        store.lookup(mode, prefix, &key)
    }

    /// Gets a node resource from the store.
    pub fn get<'a, T: Identify<'a> + Serialize<'a>>(&self, prefix: u8, id:T::ID) -> Result<T> {
        let mode = self.mode();
        let key = StoreKey::from_id::<T>(id)?;
        let store = self.store.lock()?;
       
        let item = store.get(mode, prefix, &key)?;

        let key = self.key;
        item.to_object::<T>(key)
    }

    /// Adds a node resource in the store.
    pub fn add<'a, T: Identify<'a> + Serialize<'a>>(&mut self, prefix: u8, obj: &T) -> Result<()> {
        let mode = self.mode();
        let key = self.key;
        let item = StoreItem::from_object::<T>(obj, key)?;
        
        let item_size = item.to_bytes()?.len() as u32;
        let size = self.size()?;

        if item_size + size > self.max_size {
            return Err(ErrorKind::NotEnoughSpace.into());
        }
        
        let mut store = self.store.lock()?;

        store.put(mode, prefix, &item)
    }

    /// Deletes a node resource in the store.
    pub fn del<'a, T: Identify<'a> + Serialize<'a>>(&mut self, prefix: u8, id: T::ID) -> Result<()> {
        let mode = self.mode();
        let key = StoreKey::from_id::<T>(id)?;
        let mut store = self.store.lock()?;

        store.del(mode, prefix, &key)
    }
}
