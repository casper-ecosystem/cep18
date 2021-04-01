// --------------------------- DSL ------------------------------------

use std::{
    convert::TryInto,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub use contract::contract_api::runtime;
pub use contract::contract_api::storage;
pub use contract::unwrap_or_revert::UnwrapOrRevert;
pub use types;
pub use contract_macro::*;
use types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
};

pub trait Context {
    fn func() {}
}

pub trait Save {
    fn save(&self);
}


pub struct Map<K, V> {
    prefix: String,
    // storage: HashMap<String, V>,
    key_type: PhantomData<K>,
    value_type: PhantomData<V>,
}

impl<K, V> Map<K, V>
where
    K: Clone + Default + GetKey + Eq + Hash,
    V: Clone + Default + FromBytes + ToBytes + CLTyped,
{
    pub fn new(prefix: String) -> Self {
        Map {
            prefix: prefix,
            key_type: PhantomData,
            value_type: PhantomData,
        }
    }

    pub fn get(&self, key: &K) -> V {
        // self.storage.get(&key.get_key(&self.prefix)).unwrap()
        get_key(&key.get_key(&self.prefix))
    }

    pub fn set(&mut self, key: &K, value: V) {
        // self.storage.insert(key.get_key(&self.prefix), value);
        set_key(&key.get_key(&self.prefix), value)
    }
}

pub struct Variable<V> {
    prefix: String,
    value_type: V,
    has_change: bool,
}

impl<V> Deref for Variable<V>
where
    V: Clone + Default + FromBytes + ToBytes + CLTyped,
{
    type Target = V;
    fn deref(&self) -> &Self::Target {
        &self.value_type
    }
}

impl<V> DerefMut for Variable<V>
where
    V: Clone + Default + FromBytes + ToBytes + CLTyped,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.has_change = true;
        &mut self.value_type
    }
}

impl<V> Variable<V>
where
    V: Clone + Default + FromBytes + ToBytes + CLTyped,
{
    pub fn new(prefix: String, value: V) -> Self {
        Variable {
            prefix: prefix,
            value_type: value,
            has_change: true,
        }
    }

    pub fn get(&self) -> V {
        // self.storage.get(&key.get_key(&self.prefix)).unwrap()
        get_key(&self.prefix)
    }

    pub fn set(&self) {
        // self.storage.insert(key.get_key(&self.prefix), value);
        set_key(&self.prefix, self.value_type.clone())
    }

    pub fn has_change(&self) -> bool {
        self.has_change
    }
}

pub trait GetKey {
    fn get_key(&self, prefix: &String) -> String;
}

pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}
