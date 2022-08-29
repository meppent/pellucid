use hex;
use primitive_types::U256;
use std::fmt;
use std::{collections::HashMap, usize};

use crate::utils::remove_0x;

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
    pub fn from(raw_bytecode: &str) -> Bytecode {
        let vec_bytecode: Vec<u8> = match hex::decode(remove_0x(&raw_bytecode)) {
            Ok(res) => res,
            Err(err) => panic!("Failed to decode bytecode: {}", err),
        };

        let mut bytecode: Bytecode = Bytecode {
            vopcodes: Vec::new(),
            pc_to_index: HashMap::new(),
        };

        let bytecode_length = vec_bytecode.len();
        let mut pc: usize = 0;
        while pc < bytecode_length {
            let origin_line = pc;
            let opcode: Opcode = Opcode::from_u8(vec_bytecode[pc]);
            pc += 1;

            let mut param: Option<U256> = None;

            if let Some(n_bytes) = opcode.as_push() {
                if pc + n_bytes < bytecode_length {
                    param = Some(U256::from_big_endian(&vec_bytecode[pc..pc + n_bytes]));
                }

                pc += n_bytes;
            }

            bytecode
                .pc_to_index
                .insert(origin_line, bytecode.vopcodes.len());
            bytecode
                .vopcodes
                .push(Vopcode::new(opcode, param, origin_line, pc >= bytecode_length));
        }

        return bytecode;
    }

    pub fn get_vopcode_at(&self, pc: usize) -> &Vopcode {
        return &self.vopcodes[self.pc_to_index[&pc]];
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
}

pub fn stringify_vopcodes(vopcodes: &[Vopcode]) -> String {
    let mut res: String = String::from("");
    for vopcode in vopcodes {
        res.push_str(&vopcode.to_string());
        res.push_str("\n");
    }
    return res;
}
