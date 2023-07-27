use instructions::EvmInstruction;

pub mod instructions;

pub struct Evm {
    pub program_counter: usize,
    pub stack: Vec<EvmTypes>,
}


#[derive(Debug, PartialEq, Eq)]
pub enum EvmTypes {
    U8(u8),

}


impl Evm {
    pub fn new() -> Self {
        return Evm {
            program_counter: 0,
            stack: vec![],
        };
    }

    pub fn tick(&mut self, bytecode: &Vec<usize>) {
        // get current opcode
        let opcode =
            EvmInstruction::from_opcode(*bytecode.get(self.program_counter).unwrap()).unwrap();

        match opcode {
            EvmInstruction::Push(nb_bytes) => {
                self.program_counter += 1;
                let data = bytecode[self.program_counter..(self.program_counter + nb_bytes)].to_vec();
                
                match nb_bytes {
                    1 => self.stack.push(EvmTypes::U8(data[0].try_into().unwrap())),
                    _ => panic!("not implemented"),
                }

                // advance program counter
                self.program_counter += nb_bytes;
            }
            _ => panic!("not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instructions::EvmInstruction, Evm};

    #[test]
    fn load_bytecode() {
        let filename = "sample_bytecode.bin";
        let data = std::fs::read_to_string(filename).unwrap();

        let bytecode: Vec<usize> = data
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|elems| usize::from_str_radix(&format!("{}{}", elems[0], elems[1]), 16).unwrap())
            .collect();

        let opcode = EvmInstruction::from_opcode(bytecode[0]).unwrap();
        assert_eq!(opcode, EvmInstruction::Push(1));
    }

    #[test]
    fn test_evm() {
        let filename = "sample_bytecode.bin";
        let data = std::fs::read_to_string(filename).unwrap();

        let bytecode: Vec<usize> = data
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|elems| usize::from_str_radix(&format!("{}{}", elems[0], elems[1]), 16).unwrap())
            .collect();

        let mut evm = Evm::new();
        evm.tick(&bytecode);

        println!("evm: {:?}", evm.stack);
    }
}
