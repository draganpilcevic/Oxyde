pub mod typing;
pub mod utils;
pub mod parsers;

pub struct SampleContract {
    pub field_a: u64,
    field_b: u64,
}

impl SampleContract {
    pub fn constructor(val_a: u64, val_b: u64) -> Self {
        return SampleContract {
            field_a: val_a,
            field_b: val_b,
        };
    }

    pub fn set_a(&mut self, val: u64) {
        self.field_a = val;
    }
}
