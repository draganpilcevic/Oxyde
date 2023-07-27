use std::{sync::Mutex, time::{SystemTime, UNIX_EPOCH}};

use crate::{Address, Constructable, Entrypoint, Mapping, U256, Deref, TEST_STORAGE}; //, DerefEntrypoints};
use lazy_static::{lazy_static};
use serde::Serialize;

lazy_static! {
    static ref BALANCES: Mapping<Address, U256> = Mapping::new();
    //static ref CONTRACTS: Mapping<Address, Box<dyn Entrypoint>> = Mapping::new();
}

pub static mut CONTRACTS: Mutex<Vec<(Address, Box<dyn Deref>)>> = Mutex::new(vec![]);
//pub static mut BALANCES: Mutex<Mapping<Address, U256>> = Mutex::new(Mapping::new());


pub struct Context {}

impl Context {
    pub fn execute_contract(
        contract_address: &Address,
        method: &str,
        data: Vec<u8>,
    ) -> Vec<u8> {
        let prev_contract = TEST_STORAGE.get_current_contract().unwrap();

        let mut contract = unsafe {
            let mut lock = CONTRACTS.lock().unwrap();
            let contract = lock.iter_mut().find(|(addr, _)| addr.eq(&contract_address));


            let copy = contract.unwrap().1.deref();
            //contract.unwrap().1.execute(method, data)
            copy
        };

        let data = contract.execute(method, data);
        TEST_STORAGE.set_current_contract(prev_contract.clone()).unwrap();

        return data;
    }

    pub fn query_contract(
        contract_address: &Address,
        method: &str,
        data: Vec<u8>,
    ) -> Vec<u8> {
        let prev_contract = TEST_STORAGE.get_current_contract().unwrap();

        let contract = unsafe {
            let mut lock = CONTRACTS.lock().unwrap();
            // all addresses
            //let addresses: Vec<Address> = lock.iter().map(|elem| elem.0.clone()).collect();


            let contract = lock.iter_mut().find(|(addr, _)| addr.eq(&contract_address));
            let contract = &mut contract.unwrap().1;
            TEST_STORAGE.set_current_contract(contract_address.clone()).unwrap();

            contract.load_from_store();
            contract.deref() //  execute(method, data)
            //contract.unwrap().1.deref() //.1.query(method, data)
        };

        let data = contract.query(method, data);
        TEST_STORAGE.set_current_contract(prev_contract.clone()).unwrap();

        return data;
    }

    pub fn msg_sender() -> Address {
        return TEST_STORAGE.get_common_value("msg_sender").unwrap();
    }

    pub fn timestamp() -> U256 {
        return TEST_STORAGE.get_common_value("timestamp").unwrap();
    }

    pub fn emit<T>(event: T) where T: Serialize {
        bincode::serialize(&event).unwrap();
    }
}



pub struct App {}

impl App {
    pub fn deploy<T>(&mut self, data: Vec<u8>) -> Address
    where
        T: Deref + Entrypoint + Constructable,
    {
        let addr = Address::random();
        TEST_STORAGE.set_current_contract(addr.clone()).unwrap();

        unsafe {
            let mut lock = CONTRACTS.lock().unwrap();
            lock.push((addr, T::_constructor(data)));
        }

        //self.contracts.insert(addr.clone(), T::_constructor(data));
        return addr;
    }

    pub fn new() -> Self {
        // setting some global data that must be explicitly modified?  
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        TEST_STORAGE.set_common_value("timestamp", &U256::from(time)).unwrap();

        return App {};
    }

    pub fn execute_contract(
        &mut self,
        caller: &Address,
        contract_address: &Address,
        method: &str,
        data: Vec<u8>,
    ) -> Vec<u8> {
        TEST_STORAGE.set_current_contract(contract_address.clone()).unwrap();
        TEST_STORAGE.set_common_value("msg_sender", caller).unwrap();

        let mut contract = unsafe {
            let mut lock = CONTRACTS.lock().unwrap();
            
            let contract = lock.iter_mut().find(|(addr, _)| addr.eq(&contract_address));
            let contract = &mut contract.unwrap().1;
            // set active
            
            contract.load_from_store();
            contract.deref() //  execute(method, data)
        };

        let data = contract.execute(method, data);

        return data;
    }

    pub fn query_contract(
        &self,
        contract_address: &Address,
        method: &str,
        data: Vec<u8>,
    ) -> Vec<u8> {
        TEST_STORAGE.set_current_contract(contract_address.clone()).unwrap();

        let contract = unsafe {
            let mut lock = CONTRACTS.lock().unwrap();
            let contract = lock.iter_mut().find(|(addr, _)| addr.eq(&contract_address));

            //yup.unwrap().1.deref() //  execute(method, data)
            let contract = &mut contract.unwrap().1;

            contract.load_from_store();
            contract.deref() //  execute(method, data)
            //yup.unwrap().1.query(method, data)
        };

        let data = contract.query(method, data);

        return data;
    }
}


