use oxyde_sdk::{
    deserialize, oxyde_entrypoints, serialize, test_env::Context, Address, Constructable, Deref,
    Entrypoint, StorageElement, StorageItem, StorageMap, U256,
};

#[derive(Deref)]
pub struct Erc20 {
    pub name: StorageItem<String>,
    pub symbol: StorageItem<String>,
    pub decimals: StorageItem<u8>,
    pub admin: StorageItem<Address>,
    pub balances: StorageMap<Address, U256>,
    pub total_supply: StorageItem<U256>,
    //pub allowances: StorageMap<Vec<u8>, U256>,
    pub allowances: StorageMap<String, U256>,
}

#[oxyde_entrypoints]
impl Erc20 {
    pub fn constructor(name: String, symbol: String, decimals: u8, admin: Address) -> Self {
        return Erc20 {
            name: StorageItem::new_with_value("name", name),
            symbol: StorageItem::new_with_value("symbol", symbol),
            decimals: StorageItem::new_with_value("decimals", decimals),
            admin: StorageItem::new_with_value("admin", admin),
            balances: StorageMap::new("balances"),
            total_supply: StorageItem::new("total_supply"),
            allowances: StorageMap::new("allowances"),
        };
    }

    pub fn mint(&mut self, beneficiary: Address, amount: U256) {
        let prev_balance = self.balances.get(&beneficiary);
        self.balances.set_value(&beneficiary, prev_balance + amount);
    }

    pub fn transfer_from(&mut self, sender: Address, beneficiary: Address, amount: U256) {
        let balance_sender = self.balances.get(&sender);
        let allowance = self.allowances.get(&format!(
            "{:?}",
            &serialize(&(sender, Context::msg_sender())).unwrap()
        ));
        if allowance < amount {
            panic!("Insufficient allowance")
        }

        if balance_sender < amount {
            panic!("Insufficient balance")
        }
        let balance_beneficiary = self.balances.get(&beneficiary);

        // decrease allowance
        self.allowances.set_value(
            //&serialize(&(sender, beneficiary)).unwrap(),
            &format!(
                "{:?}",
                &serialize(&(sender, Context::msg_sender())).unwrap()
            ),
            allowance - amount,
        );

        // transfer
        self.balances.set_value(&sender, balance_sender - amount);
        self.balances
            .set_value(&beneficiary, balance_beneficiary + amount);
    }

    pub fn transfer(&mut self, to: Address, amount: U256) {
        let balance_sender = self.balances.get(&Context::msg_sender());
        if balance_sender < amount {
            panic!("Insufficient balance")
        }

        let balance_beneficiary = self.balances.get(&to);

        self.balances
            .set_value(&Context::msg_sender(), balance_sender - amount);
        self.balances.set_value(&to, balance_beneficiary + amount);
    }

    pub fn balance_of(&self, account: Address) -> U256 {
        return self.balances.get(&account);
    }

    pub fn decimals(&self) -> u8 {
        return self.decimals.get_value();
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        //return self.allowances.get(&serialize(&(owner, spender)).unwrap());
        return self
            .allowances
            .get(&format!("{:?}", &serialize(&(owner, spender)).unwrap()));
    }

    pub fn approve(&mut self, spender: Address, amount: U256) {
        self.allowances.set_value(
            //&serialize(&(Context::msg_sender(), spender)).unwrap(),
            &format!(
                "{:?}",
                &serialize(&(Context::msg_sender(), spender)).unwrap()
            ),
            amount,
        );
    }
}

#[cfg(test)]
mod tests {
    use oxyde_sdk::test_env::App;

    use super::*;

    #[test]
    fn erc20_test_transfer() {
        let mut app = App::new();
        let admin = Address::random();

        let erc_address = app.deploy::<Erc20>(serialize(&("my token", "mtt", 8u8, admin)).unwrap());

        app.execute_contract(
            &admin,
            &erc_address,
            "mint",
            serialize(&(admin.clone(), U256::from(5200u64))).unwrap(),
        );

        let balance: U256 = deserialize(&app.query_contract(
            &erc_address,
            "balance_of",
            serialize(&admin).unwrap(),
        ))
        .unwrap();

        println!("balance admin: {:?}", balance);

        let user = Address::random();
        /*
        app.execute_contract(
            &admin,
            &erc_address,
            "transfer_from",
            serialize(&(admin.clone(), user.clone(), U256::from(1234u64))).unwrap(),
        );
        */
        app.execute_contract(
            &admin,
            &erc_address,
            "transfer",
            serialize(&(user.clone(), U256::from(1234u64))).unwrap(),
        );

        let balance: U256 = deserialize(&app.query_contract(
            &erc_address,
            "balance_of",
            serialize(&admin).unwrap(),
        ))
        .unwrap();

        println!("new balance admin: {:?}", balance);

        let balance: U256 =
            deserialize(&app.query_contract(&erc_address, "balance_of", serialize(&user).unwrap()))
                .unwrap();

        println!("balance user: {:?}", balance);

        let decimals: u8 =
            deserialize(&app.query_contract(&erc_address, "decimals", vec![])).unwrap();

        println!("decimals: {:?}", decimals);
    }

    #[test]
    fn erc20_test_transfer_from() {
        let mut app = App::new();
        let admin = Address::random();

        let erc_address = app.deploy::<Erc20>(serialize(&("my token", "mtt", 8u8, admin)).unwrap());

        let user = Address::random();
        let grantee = Address::random();
        let user_2 = Address::random();

        app.execute_contract(
            &admin,
            &erc_address,
            "mint",
            serialize(&(user, U256::from(5200u64))).unwrap(),
        );

        // give allowance to grantee
        app.execute_contract(
            &user,
            &erc_address,
            "approve",
            serialize(&(grantee, U256::from(250u64))).unwrap(),
        );

        // grantee will now transfer from user to user2
        app.execute_contract(
            &grantee,
            &erc_address,
            "transfer_from",
            serialize(&(user, user_2, U256::from(123u64))).unwrap(),
        );

        // check balance of user 2
        let res: U256 = deserialize(&app.query_contract(
            &erc_address,
            "balance_of",
            serialize(&(user_2)).unwrap(),
        ))
        .unwrap();

        println!("{}", res);
    }
}
