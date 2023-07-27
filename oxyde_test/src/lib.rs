
use oxyde_sdk::{
    oxyde_entrypoints,
    test_env::Context,
    Address,
    Constructable,
    Deref, //DerefEntrypoints,
    Entrypoint,
    StorageItem,
    serialize, deserialize
};

#[derive(Deref)]
struct SampleContract {
    pub value: StorageItem<u64>,
}

/*
impl Deref for SampleContract {
    fn deref(&self) -> Box<dyn DerefEntrypoints> {
        return Box::new(SampleContract {
            value: self.value.clone(),
        });
    }
}
*/

#[oxyde_entrypoints]
impl SampleContract {
    pub fn constructor(val: u64) -> Self {
        return SampleContract {
            value: StorageItem::new_with_value("value", val),
        };
    }

    pub fn get_value(&self) -> u64 {
        return self.value.get_value();
    }

    pub fn set_value(&mut self, new_value: u64) {
        println!("setting new value: {}", new_value);
        self.value.set(new_value);
    }
}

#[derive(Deref)]
struct LensContract {
    pub target: StorageItem<Address>,
}

#[oxyde_entrypoints]
impl LensContract {
    pub fn constructor(target: Address) -> Self {
        return LensContract {
            target: StorageItem::new_with_value("target", target),
        };
    }

    pub fn get_value_target(&self) -> u64 {
        //println!("calling target: {:?}", self.target.get_value());
        return bincode::deserialize(
            &Context::query_contract(&self.target.get_value().to_owned(), "get_value", vec![])[..],
        )
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use oxyde_sdk::{
        serialize,
        test_env::App, Address,
    };

    use crate::{LensContract, SampleContract};

    #[test]
    fn test_sample() {
        let mut app = App::new();

        let addr_base = app.deploy::<SampleContract>(serialize(&12u64).unwrap());
        let addr_lens = app.deploy::<LensContract>(serialize(&addr_base).unwrap());

        // try base first
        let val = app.query_contract(&addr_base, "get_value", vec![]);
        let val_decoded: u64 = bincode::deserialize(&val[..]).unwrap();
        println!("val decoded direct access: {}", val_decoded);

        let val = app.query_contract(&addr_lens, "get_value_target", vec![]);
        let val_decoded: u64 = bincode::deserialize(&val[..]).unwrap();
        println!("val decoded lens: {}", val_decoded);

        println!("mutating");
        // now mutate
        app.execute_contract(
            &Address::random(),
            &addr_base,
            "set_value",
            bincode::serialize(&321u64).unwrap(),
        );

        // and check new state
        let val = app.query_contract(&addr_base, "get_value", vec![]);
        let val_decoded: u64 = bincode::deserialize(&val[..]).unwrap();
        println!("val decoded direct access: {}", val_decoded);

        let val = app.query_contract(&addr_lens, "get_value_target", vec![]);
        let val_decoded: u64 = bincode::deserialize(&val[..]).unwrap();
        println!("val decoded lens: {}", val_decoded);
    }
}
