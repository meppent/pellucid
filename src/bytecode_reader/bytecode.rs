use hex;
use primitive_types::U256;
use std::cmp;
use std::collections::HashMap;
use std::fmt;

use crate::tools::utils::remove_0x;

use super::opcode::Opcode;
use super::vopcode::Vopcode;

#[derive(Default, Debug, PartialEq)]
pub struct Bytecode {
    vopcodes: Vec<Vopcode>,
    pc_to_index: HashMap<usize, usize>, // line => index of corresponding VOpcode in `vopcodes`
}

impl fmt::Display for Bytecode {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let res: String = stringify_vopcodes(&self.vopcodes);
        formatter.write_str(&res)?;
        Ok(())
    }
}

impl Bytecode {
    pub fn new() -> Self {
        return Bytecode {
            vopcodes: Vec::new(),
            pc_to_index: HashMap::new(),
        };
    }

    pub fn from(raw_bytecode: &str) -> Self {
        let vec_bytecode: Vec<u8> = match hex::decode(remove_0x(&raw_bytecode)) {
            Ok(res) => res,
            Err(err) => panic!("Failed to decode bytecode: {}", err),
        };

        let mut bytecode: Bytecode = Bytecode::new();

        let bytecode_length = vec_bytecode.len();
        let mut pc: usize = 0;
        while pc < bytecode_length {
            let origin_line = pc;
            let opcode: Opcode = Opcode::from(vec_bytecode[pc]);
            pc += 1;

            let mut item: Option<U256> = None;

            if let Opcode::PUSH { item_size } = opcode {
                let item_end = cmp::min(pc + item_size, bytecode_length);
                item = Some(U256::from_big_endian(&vec_bytecode[pc..item_end]));
                pc = item_end;
            }

            bytecode.insert_vopcode(Vopcode::new(opcode, item, origin_line));
        }

        return bytecode;
    }

    pub fn insert_vopcode(&mut self, vopcode: Vopcode) {
        self.pc_to_index.insert(vopcode.pc, self.vopcodes.len());
        self.vopcodes.push(vopcode);
    }

    pub fn get_vopcode_at(&self, pc: usize) -> &Vopcode {
        return &self.vopcodes[self.pc_to_index[&pc]];
    }

    pub fn get_vopcodes(&self) -> &[Vopcode] {
        return &self.vopcodes;
    }

    pub fn get_last_pc(&self) -> usize {
        return self.vopcodes[self.vopcodes.len() - 1].pc;
    }

    pub fn slice_code(&self, pc_start: usize, pc_end: usize) -> &[Vopcode] {
        return &self.vopcodes[self.pc_to_index[&pc_start]..self.pc_to_index[&pc_end] + 1];
    }

    pub fn iter<'a>(&'a self, pc_start: usize, pc_end: usize) -> std::slice::Iter<Vopcode> {
        return self.vopcodes[self.pc_to_index[&pc_start]..self.pc_to_index[&pc_end] + 1].iter();
    }

    pub fn get_previous_pc(&self, pc: usize) -> Option<usize> {
        match self.vopcodes.get(self.pc_to_index[&pc] - 1){
            None => return None,
            Some(vopcode)=> return Some(vopcode.pc)
        }
    }
}

pub fn stringify_vopcodes(vopcodes: &[Vopcode]) -> String {
    let mut res: String = String::from("");
    for vopcode in vopcodes {
        res.push_str(&vopcode.to_string());
        res.push_str("\n");
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use std::fs;

    fn read_opcodes_file(path: &str) -> Bytecode {
        let mut bytecode: Bytecode = Bytecode::new();
        let opcodes_string: String = fs::read_to_string(path).expect("Unable to read file.");
        for opcode_line in opcodes_string.split("\n") {
            if let Some(vopcode) = Vopcode::from_string(opcode_line) {
                bytecode.insert_vopcode(vopcode);
            }
        }
        return bytecode;
    }

    #[test]
    fn test_bytecode_split_simple_contract() {
        let bytecode_ref: Bytecode =
            read_opcodes_file("./assets/contracts/simple_contract/opcodes.txt");
        let bytecode_string: String =
            fs::read_to_string("./assets/contracts/simple_contract/bytecode.txt")
                .expect("Unable to read file.");
        let bytecode_test: Bytecode = Bytecode::from(&bytecode_string);
        assert_eq!(bytecode_ref, bytecode_test, "Bytecode mismatch");
    }

    #[test]
    fn test_0x_support() {
        let mut random_vec: Vec<u8> = Vec::new();
        for _ in 0..200 {
            random_vec.push(rand::random::<u8>());
        }
        let bytecode1 = hex::encode(random_vec);
        let mut bytecode2 = "0x".to_owned();
        bytecode2.push_str(&bytecode1);
        assert_eq!(Bytecode::from(&bytecode1), Bytecode::from(&bytecode2));
    }

    #[test]
    fn test_invalid_bytecode() {
        assert!(
            std::panic::catch_unwind(|| Bytecode::from("abc")).is_err(),
            "Odd size bytecode without 0x did not panic"
        );
        assert!(
            std::panic::catch_unwind(|| Bytecode::from("0xabc")).is_err(),
            "Odd size bytecode with 0x did not panic"
        );
        assert!(
            std::panic::catch_unwind(|| Bytecode::from("abcg")).is_err(),
            "Bytecode with invalid character without 0x did not panic"
        );
        assert!(
            std::panic::catch_unwind(|| Bytecode::from("0xabcg")).is_err(),
            "Bytecode with invalid character with 0x did not panic"
        );
    }
}
