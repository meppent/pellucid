use hex;
use primitive_types::U256;
use std::fmt;
use std::{collections::HashMap, usize};

use crate::utils::{remove_0x, u256_to_hex, usize_to_hex};

use super::opcode::Opcode;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vopcode {
    // an opcode with a value, used when it's a PUSH
    pub opcode: Opcode,
    pub value: Option<U256>,
    pub pc: usize,
    pub is_last: bool, // is it the last opcode of the bytecode ?
}

impl Vopcode {
    pub fn new(opcode: Opcode, value: Option<U256>, pc: usize, is_last: bool) -> Self {
        Vopcode::sanity_check(opcode, value, is_last);
        return Self {
            opcode,
            value,
            pc,
            is_last,
        };
    }

    pub fn get_next_pc(&self) -> Option<usize> {
        if self.is_last {
            None
        } else {
            Some(
                self.pc
                    + 1
                    + match self.opcode.as_push() {
                        Some(n_bytes) => n_bytes,
                        None => 0,
                    },
            )
        }
    }
    fn sanity_check(opcode: Opcode, value: Option<U256>, is_last: bool) {
        if let Some(v) = value {
            if let Some(n) = opcode.as_push() {
                assert!(1 <= n, "PUSH(n) must verify 1 <= n");
                assert!(n <= 32, "PUSH(n) must verify n <= 32");
                assert!(
                    v <= U256::from(256)
                        .overflowing_pow(U256::from(n))
                        .0
                        .overflowing_sub(U256::from(1))
                        .0,
                    "The value after PUSH(n) should be less than (2^8)^n"
                );
            } else {
                panic!("Vopcode with non empty value should be a push opcode.");
            }
        } else {
            assert!(
                is_last || opcode.as_push() == None,
                "Vopcode with an empty value should not be a push"
            );
        }
    }

    pub fn to_string(&self) -> String {
        let mut res: String = String::from(&usize_to_hex(self.pc));
        res.push_str(": ");
        res.push_str(&self.opcode.to_string());

        if let Some(_) = self.opcode.as_push() {
            res.push_str(" ");
            if let Some(bytes) = self.value {
                res.push_str(&u256_to_hex(bytes));
            } else {
                res.push_str("Invalid");
            }
        }

        return res;
    }
}

impl fmt::Display for Vopcode {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = String::from("Vopcode: ");
        res.push_str(&self.to_string());
        res.push_str(" is_last_line: ");
        res.push_str(&self.is_last.to_string());
        formatter.write_str(&res)?;
        Ok(())
    }
}

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
