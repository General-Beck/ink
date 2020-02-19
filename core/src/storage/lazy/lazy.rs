// Copyright 2019-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::super::{
    KeyPtr,
    LazyCell,
    PullForward,
    PushForward,
    StorageSize,
};
use ink_primitives::Key;

/// A lazy storage entity.
///
/// This loads its value from storage upon first use.
///
/// # Note
///
/// Use this if the storage field doesn't need to be loaded in some or most cases.
#[derive(Debug)]
pub struct Lazy<T> {
    cell: LazyCell<T>,
}

impl<T> StorageSize for Lazy<T>
where
    T: StorageSize,
{
    const SIZE: u64 = <LazyCell<T> as StorageSize>::SIZE;
}

impl<T> PullForward for Lazy<T>
where
    T: StorageSize,
{
    fn pull_forward(ptr: &mut KeyPtr) -> Self {
        Self {
            cell: <LazyCell<T> as PullForward>::pull_forward(ptr)
        }
    }
}

impl<T> PushForward for Lazy<T>
where
    T: PushForward,
{
    fn push_forward(&self, ptr: &mut KeyPtr) {
        <LazyCell<T> as PushForward>::push_forward(&self.cell, ptr)
    }
}

impl<T> Lazy<T> {
    /// Creates an eagerly populated lazy storage value.
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            cell: LazyCell::new(Some(value)),
        }
    }

    /// Creates a true lazy storage value for the given key.
    #[must_use]
    pub fn lazy(key: Key) -> Self {
        Self {
            cell: LazyCell::lazy(key),
        }
    }
}

impl<T> Lazy<T>
where
    T: scale::Decode,
{
    /// Returns a shared reference to the lazily loaded value.
    ///
    /// # Note
    ///
    /// This loads the value from the contract storage if this did not happed before.
    ///
    /// # Panics
    ///
    /// If loading from contract storage failed.
    #[must_use]
    pub fn get(&self) -> &T {
        self.cell.get().expect("expected Some value")
    }

    /// Returns an exclusive reference to the lazily loaded value.
    ///
    /// # Note
    ///
    /// This loads the value from the contract storage if this did not happed before.
    ///
    /// # Panics
    ///
    /// If loading from contract storage failed.
    #[must_use]
    pub fn get_mut(&mut self) -> &mut T {
        self.cell.get_mut().expect("expected Some value")
    }
}

impl<T> From<T> for Lazy<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Default for Lazy<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> core::cmp::PartialEq for Lazy<T>
where
    T: PartialEq + scale::Decode,
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.get(), other.get())
    }
}

impl<T> core::cmp::Eq for Lazy<T> where T: Eq + scale::Decode {}

impl<T> core::cmp::PartialOrd for Lazy<T>
where
    T: PartialOrd + scale::Decode,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        PartialOrd::partial_cmp(self.get(), other.get())
    }
    fn lt(&self, other: &Self) -> bool {
        PartialOrd::lt(self.get(), other.get())
    }
    fn le(&self, other: &Self) -> bool {
        PartialOrd::le(self.get(), other.get())
    }
    fn ge(&self, other: &Self) -> bool {
        PartialOrd::ge(self.get(), other.get())
    }
    fn gt(&self, other: &Self) -> bool {
        PartialOrd::gt(self.get(), other.get())
    }
}

impl<T> core::cmp::Ord for Lazy<T>
where
    T: core::cmp::Ord + scale::Decode,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Ord::cmp(self.get(), other.get())
    }
}

impl<T> core::fmt::Display for Lazy<T>
where
    T: scale::Decode + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(self.get(), f)
    }
}

impl<T> core::hash::Hash for Lazy<T>
where
    T: core::hash::Hash + scale::Decode,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

impl<T> core::convert::AsRef<T> for Lazy<T>
where
    T: scale::Decode,
{
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> core::convert::AsMut<T> for Lazy<T>
where
    T: scale::Decode,
{
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> ink_prelude::borrow::Borrow<T> for Lazy<T>
where
    T: scale::Decode,
{
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> ink_prelude::borrow::BorrowMut<T> for Lazy<T>
where
    T: scale::Decode,
{
    fn borrow_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> core::ops::Deref for Lazy<T>
where
    T: scale::Decode,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> core::ops::DerefMut for Lazy<T>
where
    T: scale::Decode,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}