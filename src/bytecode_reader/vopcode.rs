use primitive_types::U256;
use std::fmt;
use crate::utils::u256_to_hex;
use super::opcode::Opcode;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vopcode {
    // an opcode with a value, used when it's a PUSH
    pub opcode: Opcode,
    pub value: Option<U256>,
    pub pc: usize,
    pub is_last: bool, // is it the last opcode of the bytecode ?
}

impl Vopcode {
    pub fn new(opcode: Opcode, value: Option<U256>, pc: usize, is_last: bool) -> Self {
        Vopcode::sanity_check(opcode, value);
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
                    + if let Opcode::PUSH { item_size: n_bytes } = self.opcode {
                        n_bytes
                    } else {
                        0
                    },
            )
        }
    }
    fn sanity_check(opcode: Opcode, value: Option<U256>) {
        if let Some(v) = value {
            if let Opcode::PUSH { item_size: n_bytes } = opcode {
                assert!(1 <= n_bytes, "PUSH(n) must verify 1 <= n");
                assert!(n_bytes <= 32, "PUSH(n) must verify n <= 32");
                assert!(
                    v <= U256::from(256)
                        .overflowing_pow(U256::from(n_bytes))
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
                !opcode.is_push(),
                "Vopcode with an empty value should not be a push"
            );
        }
    }

    pub fn to_string(&self) -> String {
        let mut res: String = format!(
            "{:04x} {:02x} {}",
            self.pc,
            self.opcode.code(),
            &self.opcode.name()
        );

        if self.opcode.is_push() {
            res.push_str(" ");
            if let Some(bytes) = self.value {
                res.push_str(&u256_to_hex(bytes));
            } else {
                res.push_str("invalid");
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

#[cfg(test)] 
mod tests {
    use super::*;
    use regex::Regex;

    impl Vopcode {
        pub fn from_string(vopcode_str: &str) -> Option<Vopcode> {
            let re = Regex::new(r"([A-Fa-f0-9]+)[^\w\d]+([A-Fa-f0-9]{1, 2})[^\w\d]+([\w\d]+)[^\w\d]+(?:0x|)?([A-Fa-f0-9]+)?").unwrap();
            if let Some(caps) = re.captures(vopcode_str) {
                match (caps.get(1), caps.get(2), caps.get(3)) {
                    (Some(pc), Some(code), Some(name)) => {
                        let opcode: Opcode = Opcode::from(hex::decode(code.as_str()).unwrap()[0]);
                        
                        if name.as_str().to_lowercase() == opcode.name().to_lowercase() {
                            let mut item: Option<U256> = None;
                            match opcode {
                                Opcode::PUSH{item_size: _} => {
                                    if let Some(item_match) = caps.get(4) {
                                        let item_str: &str = item_match.as_str();
    
                                        if item_str.len() > 0 {
                                            match U256::from_str_radix(item_str, 16) {
                                                Ok(item_usize) => item = Some(item_usize),
                                                Err(_) => {},
                                            }
                                        }
                                    }
                                },
                                _ => (),
                            };
                            return Some(Vopcode::new(
                                Opcode::from(u8::from_str_radix(code.as_str(), 16).unwrap()),
                                item,
                                usize::from_str_radix(pc.as_str(), 16).unwrap(),
                                false
                            ));
                        }
                    },
                    _ => (),
                };

                
            }
            return None;
            
        }
    }
    
}
