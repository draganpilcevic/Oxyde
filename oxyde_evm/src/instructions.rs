#[derive(PartialEq, Eq, Debug)]
pub enum EvmInstruction {
    // 0x00 to 0x0b
    Stop,
    Add,
    Mul,
    Sub,
    Div,
    SignedDiv,
    Modulo,
    SignedModulo,
    AddModulo,
    MulModulo,
    Exp,
    SignedExtend,

    // 0x10 to to 0x1d
    LessThan,
    GreaterThan,
    SignedLessThan,
    SignedGreaterThan,
    Eq,
    IsZero,
    And,
    Or,
    Xor,
    Not,
    RetrieveByte,
    LeftShift,
    LogicalRightShift,
    ArithmeticRightShift,

    // 0x20
    Sha3,

    // 0x30 to 0x3F
    CurrentAddress,
    BalanceAccount,
    Origin,
    Caller,
    CallValue,
    CallDataLoad,
    CallDataSize,
    CallDataCopy,
    CodeSize,
    CodeCopy,
    GasPrice,
    ExtCodeSize,
    ExtCodeCopy,
    ReturnDataSize,
    ReturnDataCopy,
    ExtCodeHash,

    // 0x40 to 0x48
    BlockHash,
    Coinbase,
    Timestamp,
    Number,
    Difficulty,
    GasLimit,
    ChainId,
    SelfBalance,
    BaseFee,

    // 0x50 to 0x5b
    Pop,
    MemoryLoad,
    MemoryStore,
    MemoryStore8,
    StorageLoad,
    StorageStore,
    Jump,
    JumpIf,
    ProgramCounter,
    MemorySize,
    AvailableGas,
    JumpDest,

    // 0x5f to 0x9f
    /// Push 0 to the stack
    Push0,
    /// Push value of size N bytes on stack
    Push(usize),
    /// Duplicate value at position N on stack and push it at first position
    Dup(usize),
    /// Swap the first value on the stack with the value at the Nth position
    Swap(usize),

    // 0xa0 to 0xa4
    Log0,
    Log1,
    Log2,
    Log3,
    Log4,

    // 0xf0 to 0xf5
    Create,
    Call,
    CallCode,
    Return,
    DelegateCall,
    Create2,

    // 0xfa
    StaticCall,

    // 0xfd to 0xff
    Revert,
    Invalid,
    SelfDestruct,
}

impl EvmInstruction {
    pub fn from_opcode(opcode: usize) -> Option<Self> {
        return match opcode {
            0x00 => Some(EvmInstruction::Stop),
            0x01 => Some(EvmInstruction::Add),
            0x02 => Some(EvmInstruction::Mul),
            0x03 => Some(EvmInstruction::Sub),
            0x04 => Some(EvmInstruction::Div),
            0x05 => Some(EvmInstruction::SignedDiv),
            0x06 => Some(EvmInstruction::Modulo),
            0x07 => Some(EvmInstruction::SignedModulo),
            0x08 => Some(EvmInstruction::AddModulo),
            0x09 => Some(EvmInstruction::MulModulo),
            0x0a => Some(EvmInstruction::Exp),
            0x0b => Some(EvmInstruction::SignedExtend),
            0x0c..=0x0f => None,
            0x10 => Some(EvmInstruction::LessThan),
            0x11 => Some(EvmInstruction::GreaterThan),
            0x12 => Some(EvmInstruction::SignedLessThan),
            0x13 => Some(EvmInstruction::SignedGreaterThan),
            0x14 => Some(EvmInstruction::Eq),
            0x15 => Some(EvmInstruction::IsZero),
            0x16 => Some(EvmInstruction::And),
            0x17 => Some(EvmInstruction::Or),
            0x18 => Some(EvmInstruction::Xor),
            0x19 => Some(EvmInstruction::Not),
            0x1a => Some(EvmInstruction::RetrieveByte),
            0x1b => Some(EvmInstruction::LeftShift),
            0x1c => Some(EvmInstruction::LogicalRightShift),
            0x1d => Some(EvmInstruction::ArithmeticRightShift),

            0x1e..=0x1f => None,
            0x20 => Some(EvmInstruction::Sha3),
            0x21..=0x2f => None,

            0x30 => Some(EvmInstruction::CurrentAddress),
            0x31 => Some(EvmInstruction::BalanceAccount),
            0x32 => Some(EvmInstruction::Origin),
            0x33 => Some(EvmInstruction::Caller),
            0x34 => Some(EvmInstruction::CallValue),
            0x35 => Some(EvmInstruction::CallDataLoad),
            0x36 => Some(EvmInstruction::CallDataSize),
            0x37 => Some(EvmInstruction::CallDataCopy),
            0x38 => Some(EvmInstruction::CodeSize),
            0x39 => Some(EvmInstruction::CodeCopy),
            0x3a => Some(EvmInstruction::GasPrice),
            0x3b => Some(EvmInstruction::ExtCodeSize),
            0x3c => Some(EvmInstruction::ExtCodeCopy),
            0x3d => Some(EvmInstruction::ReturnDataSize),
            0x3e => Some(EvmInstruction::ReturnDataCopy),
            0x3f => Some(EvmInstruction::ExtCodeHash),

            0x40 => Some(EvmInstruction::BlockHash),
            0x41 => Some(EvmInstruction::Coinbase),
            0x42 => Some(EvmInstruction::Timestamp),
            0x43 => Some(EvmInstruction::Number),
            0x44 => Some(EvmInstruction::Difficulty),
            0x45 => Some(EvmInstruction::GasLimit),
            0x46 => Some(EvmInstruction::ChainId),
            0x47 => Some(EvmInstruction::SelfBalance),
            0x48 => Some(EvmInstruction::BaseFee),
            0x49..=0x4f => None,

            0x50 => Some(EvmInstruction::Pop),
            0x51 => Some(EvmInstruction::MemoryLoad),
            0x52 => Some(EvmInstruction::MemoryStore),
            0x53 => Some(EvmInstruction::MemoryStore8),
            0x54 => Some(EvmInstruction::StorageLoad),
            0x55 => Some(EvmInstruction::StorageStore),
            0x56 => Some(EvmInstruction::Jump),
            0x57 => Some(EvmInstruction::JumpIf),
            0x58 => Some(EvmInstruction::ProgramCounter),
            0x59 => Some(EvmInstruction::MemorySize),
            0x5a => Some(EvmInstruction::AvailableGas),
            0x5b => Some(EvmInstruction::JumpDest),
            0x5c..=0x5e => None,
            0x5f => Some(EvmInstruction::Push0),
            0x60..=0x7f => Some(EvmInstruction::Push(opcode - 0x60 + 1)),
            0x80..=0x8f => Some(EvmInstruction::Dup(opcode - 0x80)),
            0x90..=0x9f => Some(EvmInstruction::Swap(opcode - 0x90 + 1)),

            0xa0 => Some(EvmInstruction::Log0),
            0xa1 => Some(EvmInstruction::Log1),
            0xa2 => Some(EvmInstruction::Log2),
            0xa3 => Some(EvmInstruction::Log3),
            0xa4 => Some(EvmInstruction::Log4),

            // there seems to be some opcode here somewhere but unused?
            0xa5..=0xef => None,

            0xf0 => Some(EvmInstruction::Create),
            0xf1 => Some(EvmInstruction::Call),
            0xf2 => Some(EvmInstruction::CallCode),
            0xf3 => Some(EvmInstruction::Return),
            0xf4 => Some(EvmInstruction::DelegateCall),
            0xf5 => Some(EvmInstruction::Create2),

            0xf6..=0xf9 => None,

            0xfa => Some(EvmInstruction::StaticCall),

            0xfb..=0xfc => None,

            0xfd => Some(EvmInstruction::Revert),
            0xfe => Some(EvmInstruction::Invalid),
            0xff => Some(EvmInstruction::SelfDestruct),

            _ => None,
        };
    }

    pub fn to_opcode(&self) -> usize {
        return match self {
            EvmInstruction::Stop => 0x00,
            EvmInstruction::Add => 0x01,
            EvmInstruction::Mul => 0x02,
            EvmInstruction::Sub => 0x03,
            EvmInstruction::Div => 0x04,
            EvmInstruction::SignedDiv => 0x05,
            EvmInstruction::Modulo => 0x06,
            EvmInstruction::SignedModulo => 0x07,
            EvmInstruction::AddModulo => 0x08,
            EvmInstruction::MulModulo => 0x09,
            EvmInstruction::Exp => 0x0a,
            EvmInstruction::SignedExtend => 0x0b,
            EvmInstruction::LessThan => 0x10,
            EvmInstruction::GreaterThan => 0x11,
            EvmInstruction::SignedLessThan => 0x12,
            EvmInstruction::SignedGreaterThan => 0x13,
            EvmInstruction::Eq => 0x14,
            EvmInstruction::IsZero => 0x15,
            EvmInstruction::And => 0x16,
            EvmInstruction::Or => 0x17,
            EvmInstruction::Xor => 0x18,
            EvmInstruction::Not => 0x19,
            EvmInstruction::RetrieveByte => 0x1a,
            EvmInstruction::LeftShift => 0x1b,
            EvmInstruction::LogicalRightShift => 0x1c,
            EvmInstruction::ArithmeticRightShift => 0x1d,
            EvmInstruction::Sha3 => 0x20,
            EvmInstruction::CurrentAddress => 0x30,
            EvmInstruction::BalanceAccount => 0x31,
            EvmInstruction::Origin => 0x32,
            EvmInstruction::Caller => 0x33,
            EvmInstruction::CallValue => 0x34,
            EvmInstruction::CallDataLoad => 0x35,
            EvmInstruction::CallDataSize => 0x36,
            EvmInstruction::CallDataCopy => 0x37,
            EvmInstruction::CodeSize => 0x38,
            EvmInstruction::CodeCopy => 0x39,
            EvmInstruction::GasPrice => 0x3a,
            EvmInstruction::ExtCodeSize => 0x3b,
            EvmInstruction::ExtCodeCopy => 0x3c,
            EvmInstruction::ReturnDataSize => 0x3d,
            EvmInstruction::ReturnDataCopy => 0x3e,
            EvmInstruction::ExtCodeHash => 0x3f,
            EvmInstruction::BlockHash => 0x40,
            EvmInstruction::Coinbase => 0x41,
            EvmInstruction::Timestamp => 0x42,
            EvmInstruction::Number => 0x43,
            EvmInstruction::Difficulty => 0x44,
            EvmInstruction::GasLimit => 0x45,
            EvmInstruction::ChainId => 0x46,
            EvmInstruction::SelfBalance => 0x47,
            EvmInstruction::BaseFee => 0x48,
            EvmInstruction::Pop => 0x50,
            EvmInstruction::MemoryLoad => 0x51,
            EvmInstruction::MemoryStore => 0x52,
            EvmInstruction::MemoryStore8 => 0x53,
            EvmInstruction::StorageLoad => 0x54,
            EvmInstruction::StorageStore => 0x55,
            EvmInstruction::Jump => 0x56,
            EvmInstruction::JumpIf => 0x57,
            EvmInstruction::ProgramCounter => 0x58,
            EvmInstruction::MemorySize => 0x59,
            EvmInstruction::AvailableGas => 0x5a,
            EvmInstruction::JumpDest => 0x5b,
            EvmInstruction::Push0 => 0x5f,
            EvmInstruction::Push(n) => 0x60 + n - 1,
            EvmInstruction::Dup(n) => 0x80 + n,
            EvmInstruction::Swap(n) => 0x90 + n - 1,
            EvmInstruction::Log0 => 0xa0,
            EvmInstruction::Log1 => 0xa1,
            EvmInstruction::Log2 => 0xa2,
            EvmInstruction::Log3 => 0xa3,
            EvmInstruction::Log4 => 0xa4,
            EvmInstruction::Create => 0xf0,
            EvmInstruction::Call => 0xf1,
            EvmInstruction::CallCode => 0xf2,
            EvmInstruction::Return => 0xf3,
            EvmInstruction::DelegateCall => 0xf4,
            EvmInstruction::Create2 => 0xf5,
            EvmInstruction::StaticCall => 0xfa,
            EvmInstruction::Revert => 0xfd,
            EvmInstruction::Invalid => 0xfe,
            EvmInstruction::SelfDestruct => 0xff,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::EvmInstruction;

    #[test]
    fn get_dedup() {
        let opcode = 0x80;
        let instruction = EvmInstruction::from_opcode(opcode).unwrap();
        assert_eq!(instruction, EvmInstruction::Dup(0));

        let opcode = 0x8f;
        let instruction = EvmInstruction::from_opcode(opcode).unwrap();
        assert_eq!(instruction, EvmInstruction::Dup(15));
    }

    #[test]
    fn get_push() {
        let opcode = 0x60;
        let instruction = EvmInstruction::from_opcode(opcode).unwrap();
        assert_eq!(instruction, EvmInstruction::Push(1));

        let opcode = 0x7f;
        let instruction = EvmInstruction::from_opcode(opcode).unwrap();
        assert_eq!(instruction, EvmInstruction::Push(32));
    }

    #[test]
    fn get_swap() {
        let opcode = 0x90;
        let instruction = EvmInstruction::from_opcode(opcode).unwrap();
        assert_eq!(instruction, EvmInstruction::Swap(1));

        let opcode = 0x9f;
        let instruction = EvmInstruction::from_opcode(opcode).unwrap();
        assert_eq!(instruction, EvmInstruction::Swap(16));
    }

    #[test]
    fn roundtrip_instructions() {
        for value in 0x00..=0xff {
            match EvmInstruction::from_opcode(value) {
                None => (),
                Some(instruction) => {
                    assert_eq!(instruction.to_opcode(), value);
                }
            }
        }
    }
}
