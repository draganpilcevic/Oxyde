
#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    use ethereum_types::Address;
    use oxyde_sdk::{Constructable, Entrypoint, StorageItem, test_env::App, Deref, oxyde_entrypoints, serialize, deserialize};

    #[derive(Deref)]
    struct SampleContract {
        pub val: StorageItem<u64>,
    }

    #[oxyde_entrypoints]
    impl SampleContract {
        pub fn constructor(data: u64) -> Self {
            return SampleContract {
                val: StorageItem::new_with_value("val", data),
            };
        }

        pub fn mut_val(&mut self, data: u64) { // -> Result<(), String> {
            self.val.set(data);
            //return Ok(());
        }

        pub fn get_val(&self) -> u64 {
            return self.val.inner;
        }
    }

    #[test]
    fn test_deploy() {
        let mut app = App::new();
        let contract_address = app.deploy::<SampleContract>(bincode::serialize(&11u64).unwrap());

        let res = app.query_contract(&contract_address, "get_val", vec![]);

        let val: u64 = bincode::deserialize(&res).unwrap();
        assert_eq!(val, 11u64);
        
        let admin = Address::random();

        app.execute_contract(&admin,&contract_address, "mut_val", bincode::serialize(&412u64).unwrap());
        let res = app.query_contract(&contract_address, "get_val", vec![]);
        let val: u64 = bincode::deserialize(&res).unwrap();
        assert_eq!(val, 412u64);
        

        //let boxed = SampleContract::_constructor(bincode::serialize("no way?").unwrap());
    }
}
