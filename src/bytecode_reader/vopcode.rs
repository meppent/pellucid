use primitive_types::U256;
use std::fmt;

use crate::utils::{u256_to_hex, usize_to_hex};

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
