

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    use oxyde_macros::oxyde_entrypoints;
    use oxyde_sdk::{Constructable, Entrypoint, Deref, serialize, deserialize};

    #[derive(Deref)]
    struct SampleContract {}


    #[oxyde_entrypoints]
    impl SampleContract {
        pub fn constructor(data: String) -> Self {
            println!("In constructor for SampleContract: {}", data);
            return SampleContract {};
        }

        pub fn mut_fn_no_args(&mut self) {
            return ();
        }

        pub fn mut_fn_single_arg(&mut self, data_1: u64) { //-> Result<(), String> {
            return ();
        }

        pub fn mut_fn_two_args(&mut self, data_1: u64, data_2: String) { //-> Result<(), String> {
            return ();
        }

        pub fn non_mut_fn_no_args(&self) { //-> Result<(), String> {
            return ();
        }

        pub fn non_mut_fn_single_arg(&self, data_1: u64) { //-> Result<(), String> {
            return ();
        }

        pub fn non_mut_fn_two_args(&self, data_1: u64, data_2: String) { //-> Result<(), String> {
            return ();
        }

        pub fn pure_fn_no_args() { //-> Result<(), String> {
            return ();
        }

        pub fn pure_fn_single_arg(data_1: u64) { //-> Result<(), String> {
            return ();
        }

        pub fn pure_fn_two_args(data_1: u64, data_2: String) { //-> Result<(), String> {
            return ();
        }
    }

    #[test]
    fn call_method_success() {
        let mut sample_contract = SampleContract {};

        // mutating functions
        sample_contract.execute("mut_fn_no_args", bincode::serialize(&()).unwrap());
        sample_contract.execute("mut_fn_single_arg", bincode::serialize(&(123u64)).unwrap());
        sample_contract.execute(
            "mut_fn_two_args",
            bincode::serialize(&(123u64, String::from("hey there"))).unwrap(),
        );

        sample_contract.query("non_mut_fn_no_args", bincode::serialize(&()).unwrap());
        sample_contract.query(
            "non_mut_fn_single_arg",
            bincode::serialize(&(123u64)).unwrap(),
        );
        sample_contract.query(
            "non_mut_fn_two_args",
            bincode::serialize(&(123u64, String::from("hey there"))).unwrap(),
        );

        sample_contract.query("pure_fn_no_args", bincode::serialize(&()).unwrap());
        sample_contract.query("pure_fn_single_arg", bincode::serialize(&(123u64)).unwrap());
        sample_contract.query(
            "pure_fn_two_args",
            bincode::serialize(&(123u64, String::from("hey there"))).unwrap(),
        );

        let boxed = SampleContract::_constructor(bincode::serialize("no way?").unwrap());
    }
}
