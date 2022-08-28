use primitive_types::U256;
use std::fmt;
use std::{collections::HashMap, usize};

use crate::utils::{assert_hex, hex_to_u256, u256_to_hex, usize_to_hex};

use super::opcode::Opcode;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vopcode {
    // an opcode with a value, used when it's a PUSH
    pub opcode: Opcode,
    pub value: Option<U256>,
    pub pc: usize,
}

impl Vopcode {
    pub fn new(opcode: Opcode, value: Option<U256>, pc: usize) -> Self {
        Vopcode::sanity_check(opcode, value);
        return Self { opcode, value, pc };
    }

    pub fn get_next_pc(&self) -> usize {
        return self.pc
            + 1
            + match self.opcode.as_push() {
                Some(n_bytes) => n_bytes,
                None => 0,
            };
    }

    fn sanity_check(opcode: Opcode, value: Option<U256>) {
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
                panic!("Vopcode with non empty value should be a push opcode.")
            }
        } else {
            assert!(
                opcode.as_push() == None,
                "Vopcode with an empty value should not be a push"
            );
        }
    }

    pub fn to_string(&self) -> String {
        let mut res: String = String::from(&usize_to_hex(self.pc));
        res.push_str(": ");
        res.push_str(&self.opcode.to_string());

        if let Some(bytes) = self.value {
            res.push_str(" ");
            res.push_str(&u256_to_hex(bytes));
        }
        return res;
    }
}

pub struct Bytecode {
    vopcodes: Vec<Vopcode>,
    pc_to_index: HashMap<usize, usize>, // line => index of corresponding VOpcode in `vopcodes`
}

impl fmt::Display for Bytecode {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let res: String = self.stringify_range(0, self.get_last_pc());
        formatter.write_str(&res)?;
        Ok(())
    }
}

impl Bytecode {
    pub fn from(raw_bytecode: &str) -> Bytecode {
        let string_bytecode: &str = &Self::format_string_bytecode(raw_bytecode);
        assert!(string_bytecode.len() % 2 == 0);
        let length: usize = string_bytecode.len() / 2;

        let mut loader: Bytecode = Bytecode {
            vopcodes: Vec::new(),
            pc_to_index: HashMap::new(),
        };

        let mut line: usize = 0;
        while line < length {
            let origin_line = line;
            let hex_opcode: &str = &string_bytecode[2 * line..2 * (line + 1)];
            let opcode: Opcode = Opcode::from(hex_opcode);
            let mut param: Option<U256> = None;

            if let Some(n_bytes) = opcode.as_push() {
                if line + 1 + n_bytes >= length {
                    // we are at the end, it's probably the hash of the solidity
                    break;
                }
                let hex: &str = &string_bytecode[2 * (line + 1)..2 * (line + 1 + n_bytes)];

                param = Some(hex_to_u256(hex));
                line += n_bytes;
            }

            loader
                .vopcodes
                .push(Vopcode::new(opcode, param, origin_line));
            loader
                .pc_to_index
                .insert(origin_line, loader.vopcodes.len() - 1);
            line += 1;
        }
        return loader;
    }

    pub fn pc_exists(&self, pc: usize) -> bool {
        return self.pc_to_index.contains_key(&pc);
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

    fn format_string_bytecode(bytecode: &str) -> String {
        assert!(bytecode.len() % 2 == 0);
        assert!(bytecode.len() > 0);

        let mut formatted_bytecode: String = if &bytecode[0..2] == "0x" {
            bytecode[2..].to_string()
        } else {
            bytecode.to_string()
        };
        formatted_bytecode = formatted_bytecode.to_uppercase();
        assert_hex(&formatted_bytecode);
        return formatted_bytecode.to_string();
    }

    pub fn iter<'a>(&'a self, pc_start: usize, pc_end: usize) -> std::slice::Iter<Vopcode> {
        return self.vopcodes[self.pc_to_index[&pc_start]..self.pc_to_index[&pc_end] + 1].iter();
    }

    pub fn stringify_range(&self, pc_start: usize, pc_end: usize) -> String {
        // both range pc_start and pc_end are included

        let mut res: String = String::from("");

        for vopcode in self.iter(pc_start, pc_end) {
            res.push_str(&vopcode.to_string());
            res.push_str("\n");
        }
        return res;
    }
}
