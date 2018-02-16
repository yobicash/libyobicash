// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `traits` module provides the store traits.

use result::Result;
use store::key::StoreKey;
use store::value::StoreValue;
use store::item::StoreItem;
use store::mode::StoreMode;

/// A trait defining the storage operations required by a store.
pub trait Store {
    /// Returns the store data size.
    fn size(&self, mode: StoreMode) -> Result<u32>;

    /// Returns the prefix data size.
    fn prefix_size(&self, mode: StoreMode, prefix: u8) -> Result<u32>;

    /// Returns if the store is empty.
    fn is_empty(&self, mode: StoreMode) -> Result<bool>;

    /// Returns if the store prefix is empty.
    fn is_prefix_empty(&self, mode: StoreMode, prefix: u8) -> Result<bool>;

    /// List the items in the store.
    fn list(&self, mode: StoreMode, prefix: u8) -> Result<Vec<StoreItem>>;

    /// List the keys in the store.
    fn list_keys(&self, mode: StoreMode, prefix: u8) -> Result<Vec<StoreKey>>;

    /// List the values in the store.
    fn list_values(&self, mode: StoreMode, prefix: u8) -> Result<Vec<StoreValue>>;

    /// Sample the items from the store.
    fn sample(&self, mode: StoreMode, prefix: u8, count: u32) -> Result<Vec<StoreItem>>;

    /// Sample the keys from the store.
    fn sample_keys(&self, mode: StoreMode, prefix: u8, count: u32) -> Result<Vec<StoreKey>>;

    /// Sample the values from the store.
    fn sample_values(&self, mode: StoreMode, prefix: u8, count: u32) -> Result<Vec<StoreValue>>;

    /// Lookup the data of key `key` in the store.
    fn lookup(&self, mode: StoreMode, prefix: u8, key: &StoreKey) -> Result<bool>;

    /// Get the data of key `key` from the store.
    fn get(&self, mode: StoreMode, prefix: u8, key: &StoreKey) -> Result<StoreValue>;

    /// Put the `item` in the store. (upsert)
    fn put(&mut self, mode: StoreMode, prefix: u8, item: &StoreItem) -> Result<()>;

    /// Delete the item of key `key` from the store.
    fn del(&mut self, mode: StoreMode, prefix: u8, key: &StoreKey) -> Result<()>;
}
