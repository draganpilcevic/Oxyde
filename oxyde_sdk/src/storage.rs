use crate::Address;
use serde::Serialize;
use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, AddAssign, SubAssign},
};

const STORAGE_LOCATION: &str = "TEST_BLOCKCHAIN_DB";
/// save temporary, so we can unmake all state changes on failure
const _TEMP_STORAGE_LOCATION: &str = "TEMP_TEST_BLOCKCHAIN_DB";

pub static TEST_STORAGE: StorageSystem = StorageSystem::new(); // StorageSystem::new("blockhain_storage");

//pub static MUTEX_STORAGE: Mutex<StorageSystem> = Mutex::new(StorageSystem::new());

pub struct StorageSystem {
    //pub db: Box<jfs::Store>
}

impl StorageSystem {
    pub const fn new() -> Self {
        return StorageSystem {};
    }

    const CURRENT_CONTRACT_KEY: &str = "active_contract";
    const BLOCK_ID_KEY: &str = "block_id";
    const TIMESTAMP_KEY: &str = "timestamp";


    pub fn get_common_value<T>(&self, key: &str) -> Result<T, ()> where T: Default + Serialize + Clone + for<'de> serde::Deserialize<'de> {
        let db = jfs::Store::new(format!("{}/{}", STORAGE_LOCATION, "commons")).unwrap();
        return match db.get(key) {
            Ok(val) => Ok(val),
            Err(_) => Err(()),
        };
    }

    pub fn set_common_value<T> (&self, key: &str, value: &T) -> Result<(), String> where T: Default + Serialize + Clone + for<'de> serde::Deserialize<'de> {
        let db = jfs::Store::new(format!("{}/{}", STORAGE_LOCATION, "commons")).unwrap();
        return match db.save_with_id(value, key) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        };
    }

    pub fn get_current_block(&self) -> u64 {
        let db = jfs::Store::new(format!("{}/{}", STORAGE_LOCATION, "commons")).unwrap();
        return match db.get(StorageSystem::BLOCK_ID_KEY) {
            Ok(val) => val,
            Err(_) => 0,
        };
    }

    pub fn set_current_block(&self, block_id: u64) -> Result<(), ()> {
        let db = jfs::Store::new(format!("{}/{}", STORAGE_LOCATION, "commons")).unwrap();
        return match db.save_with_id(&block_id, StorageSystem::BLOCK_ID_KEY) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        };
    }

    pub fn advance_block(&self, advance_by: u64) -> Result<(), ()> {
        let curr_block = self.get_current_block();
        return self.set_current_block(curr_block + advance_by);
    }

    pub fn set_current_contract(&self, contract_address: Address) -> Result<(), ()> {
        let db = jfs::Store::new(format!("{}/{}", STORAGE_LOCATION, "commons")).unwrap();
        return match db.save_with_id(&contract_address, StorageSystem::CURRENT_CONTRACT_KEY) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        };
    }

    pub fn clear_current_contract(&self) {
        let db = jfs::Store::new(format!("{}/{}", STORAGE_LOCATION, "commons")).unwrap();
        db.delete(StorageSystem::CURRENT_CONTRACT_KEY).unwrap();
    }

    pub fn get_current_contract(&self) -> Option<Address> {
        let db = jfs::Store::new(format!("{}/{}", STORAGE_LOCATION, "commons")).unwrap();
        return db.get(StorageSystem::CURRENT_CONTRACT_KEY).ok();

        //db.delete("active_contract");
    }

    fn get_key(&self, item_key: &str) -> String {
        let curr_contract = self.get_current_contract();
        return match curr_contract {
            Some(address) => {
                format!("{}_-_{}", address, item_key)
            }
            None => {
                format!("{}_-_{}", "default", item_key)
            }
        };
    }

    pub fn save<T>(
        &self,
        element: &impl StorageElement<T>, /*&StorageItem<T>*/
    ) -> Result<(), String>
    where
        T: Default + Serialize + Clone + for<'de> serde::Deserialize<'de>,
    {
        let db = jfs::Store::new(STORAGE_LOCATION).unwrap();

        return match db.save_with_id(&element.get_inner(), &self.get_key(&element.get_key().to_string())) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        };
    }

    pub fn load_mut<T>(&self, element: &mut impl StorageElement<T>) -> Result<(), ()>
    where
        T: Default + Serialize + Clone + for<'de> serde::Deserialize<'de>,
    {
        let db = jfs::Store::new(STORAGE_LOCATION).unwrap();

        match db.get(&self.get_key(&element.get_key())) {
            Ok(val) => {
                element.set_inner(val);
                return Ok(());
            }
            Err(_) => return Err(()),
        }
    }

    pub fn load<T>(&self, key: &str) -> Result<T, ()>
    where
        T: Default + Serialize + for<'de> serde::Deserialize<'de>,
    {
        let db = jfs::Store::new(STORAGE_LOCATION).unwrap();

        return match db.get(&self.get_key(key)) {
            Ok(val) => Ok(val),
            Err(_) => Err(()),
        };
    }
}

pub trait StorageElement<T> {
    fn get_inner(&self) -> T;
    fn get_key(&self) -> String;
    fn set_inner(&mut self, val: T);
    fn load_mut(&mut self);
}

#[derive(Clone)]
pub struct StorageItem<T>
where
    T: Default + Serialize + Clone,
{
    pub key: String,
    pub inner: T,
}

impl<T> StorageElement<T> for StorageItem<T>
where
    T: Default + Serialize + Clone + for<'de> serde::Deserialize<'de>,
{
    fn get_inner(&self) -> T {
        return self.inner.clone();
    }

    fn get_key(&self) -> String {
        return self.key.clone();
    }

    fn set_inner(&mut self, val: T) {
        self.inner = val;
    }

    fn load_mut(&mut self) {
        TEST_STORAGE.load_mut(self).unwrap();
    }
}

impl<T> StorageItem<T>
where
    T: Default + Serialize + Clone + for<'de> serde::Deserialize<'de>,
{
    /*
    pub fn new(value: T) -> Self {
        return StorageItem { inner: value };
    }
    */

    pub fn new(key: &str) -> Self {
        let elem = StorageItem {
            key: key.to_owned(),
            inner: T::default(),
        };

        TEST_STORAGE.save(&elem).unwrap();

        return elem;
    }

    pub fn new_with_value(key: &str, value: T) -> Self {
        let elem = StorageItem {
            key: key.to_owned(),
            inner: value,
        };

        TEST_STORAGE.save(&elem).unwrap();

        return elem;
    }

    pub fn set(&mut self, new_val: T) {
        self.inner = new_val;
        TEST_STORAGE.save(self).unwrap();
    }

    pub fn get_value(&self) -> T
    where
        T: Clone,
    {
        return self.inner.clone();
    }

    pub fn load_mut(&mut self) {
        TEST_STORAGE.load_mut(self).unwrap();
    }
}

impl<'de, T> StorageItem<T>
where
    T: Add<T> + Default + Clone + AddAssign + SubAssign + Serialize,
{
    pub fn add(&mut self, other: T) {
        self.inner += other;
    }

    pub fn sub(&mut self, other: T) {
        self.inner -= other;
    }
}

/*
impl<T> AddAssign for StorageItem<T> where T: Add<T> + Default + AddAssign + SubAssign + Serialize
{
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner;
    }
}
*/

impl<T> AddAssign<T> for StorageItem<T>
where
    T: Add<Output = T> + Clone + Default + AddAssign + SubAssign + Serialize,
{
    fn add_assign(&mut self, rhs: T) {
        self.inner += rhs;
    }
}

use std::ops;

impl<T> ops::Add<T> for StorageItem<T>
where
    T: Add<Output = T> + Clone + Default + AddAssign + SubAssign + Serialize,
{
    type Output = T;

    fn add(self, rhs: T) -> T {
        return self.inner + rhs;
    }
}

#[derive(Clone)]
pub struct StorageMap<K, V> {
    pub key: String,
    inner: HashMap<K, V>,
}

impl<K, V> StorageElement<HashMap<K, V>> for StorageMap<K, V>
where
    V: Default + Serialize + Clone + for<'de> serde::Deserialize<'de>,
    K: Clone + Serialize + Eq + Hash + for<'de> serde::Deserialize<'de>,
{
    fn get_inner(&self) -> HashMap<K, V> {
        return self.inner.clone();
    }

    fn get_key(&self) -> String {
        return self.key.clone();
    }

    fn set_inner(&mut self, val: HashMap<K, V>) {
        self.inner = val;
    }

    fn load_mut(&mut self) {
        TEST_STORAGE.load_mut(self).unwrap();
    }
}

impl<K, V> StorageMap<K, V>
where
    V: Default + Serialize + Clone + for<'de> serde::Deserialize<'de>,
    K: Clone + Serialize + Eq + Hash + for<'de> serde::Deserialize<'de>,
{
    pub fn new(key: &str) -> Self {
        let elem = StorageMap {
            key: key.to_owned(),
            inner: HashMap::default(),
        };

        TEST_STORAGE.save(&elem).unwrap();

        return elem;
    }

    pub fn set_value(&mut self, key: &K, value: V) {
        //let map: HashMap<Address, U256> = HashMap::new();
        self.inner.insert(key.to_owned(), value);
        TEST_STORAGE.save(self).unwrap();
    }

    pub fn get(&self, key: &K) -> V {
        match self.inner.get(key) {
            Some(val) => val.to_owned(),
            None => V::default(),
        } // .unwrap_or_default().to_owned();
    }
}
