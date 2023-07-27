mod storage;
pub mod utils;
pub mod test_env;
pub mod errors;


use errors::OxydeError;
pub use oxyde_macros::{oxyde_entrypoints, Deref};
use serde::{Serialize, Deserialize};
pub use storage::TEST_STORAGE;

use std::collections::HashMap;
pub type Mapping<K, V> = HashMap<K, V>;
pub use ethereum_types::{Address, U256};
pub use storage::{StorageItem, StorageMap, StorageElement};


pub fn serialize<T>(value: &T) -> Result<Vec<u8>, OxydeError> where T: Serialize {
    return match bincode::serialize(value) {
        Ok(val) => Ok(val),
        Err(e) => Err(OxydeError::SerializationError { err: e.to_string() })
    };
}

pub fn deserialize<'a, T>(value: &'a Vec<u8>) -> Result<T, OxydeError> where T: Deserialize<'a> {
    return match bincode::deserialize(value) {
        Ok(val) => Ok(val),
        Err(e) => Err(OxydeError::SerializationError { err: e.to_string() })
    };
}


pub trait Entrypoint {
    fn execute(&mut self, method: &str, data: Vec<u8>) -> Vec<u8>;
    fn query(&self, method: &str, data: Vec<u8>) -> Vec<u8>;
}

pub trait Constructable {
    fn _constructor(data: Vec<u8>) -> Box<dyn Deref>;
}


pub trait Deref {
    fn deref(&self) -> Box<dyn Entrypoint>;
    fn load_from_store(&mut self);
}

