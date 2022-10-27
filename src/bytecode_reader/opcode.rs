use super::opcode_functions::{
    eval_add, eval_addmod, eval_and, eval_div, eval_eq, eval_exp, eval_gt, eval_iszero, eval_lt,
    eval_mod, eval_mul, eval_mulmod, eval_not, eval_or, eval_sar, eval_sdiv, eval_sgt, eval_shl,
    eval_shr, eval_signextend, eval_slt, eval_smod, eval_sub, eval_xor,
};
use primitive_types::U256;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize, Serialize)]
pub enum Opcode {
    // 0x0 range - arithmetic ops.
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,

    // 0x10 range - comparison ops.
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    SAR,

    // 0x20 range - crypto.
    SHA3,

    // 0x30 range - closure state.
    ADDRESS,
    BALANCE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    EXTCODEHASH,

    // 0x40 range - block operations.
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    DIFFICULTY,
    GASLIMIT,
    CHAINID,
    SELFBALANCE,
    BASEFEE,

    // 0x50 range - 'storage' and execution.
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    MSIZE,
    GAS,
    JUMPDEST,

    // 0x60 range - pushes.
    PUSH { item_size: usize },

    // 0x80 range - dups.
    DUP { depth: usize },

    // 0x90 range - swaps.
    SWAP { depth: usize },

    // 0xa0 range - logging ops.
    LOG { topic_count: usize },

    // 0xf0 range - closures.
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
    CREATE2,
    STATICCALL,
    REVERT,
    SELFDESTRUCT,

    INVALID { code: u8 },
}

struct OpcodeInfo {
    code: u8,
    name: String,
    stack_input: usize,
    stack_output: usize,
    external_effect: bool, // memory, storage, call, sha3 ... whenever the moment of the use of the opcode matters
    function: Option<Box<dyn Fn(Vec<U256>) -> U256>>,
}

impl Opcode {
    fn opcode_info(&self) -> OpcodeInfo {
        match self {
            // 0x0 range - arithmetic ops.
            Opcode::STOP => OpcodeInfo {
                code: 0x00,
                name: "STOP".to_owned(),
                stack_input: 0,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::ADD => OpcodeInfo {
                code: 0x01,
                name: "ADD".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_add)),
            },
            Opcode::MUL => OpcodeInfo {
                code: 0x02,
                name: "MUL".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_mul)),
            },
            Opcode::SUB => OpcodeInfo {
                code: 0x03,
                name: "SUB".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_sub)),
            },
            Opcode::DIV => OpcodeInfo {
                code: 0x04,
                name: "DIV".to_owned(),

                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_div)),
            },
            Opcode::SDIV => OpcodeInfo {
                code: 0x05,
                name: "SDIV".to_owned(),

                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_sdiv)),
            },
            Opcode::MOD => OpcodeInfo {
                code: 0x06,
                name: "MOD".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_mod)),
            },
            Opcode::SMOD => OpcodeInfo {
                code: 0x07,
                name: "SMOD".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_smod)),
            },
            Opcode::ADDMOD => OpcodeInfo {
                code: 0x08,
                name: "ADDMOD".to_owned(),
                stack_input: 3,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_addmod)),
            },
            Opcode::MULMOD => OpcodeInfo {
                code: 0x09,
                name: "MULMOD".to_owned(),
                stack_input: 3,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_mulmod)),
            },
            Opcode::EXP => OpcodeInfo {
                code: 0x0a,
                name: "EXP".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_exp)),
            },
            Opcode::SIGNEXTEND => OpcodeInfo {
                code: 0x0b,
                name: "SIGNEXTEND".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_signextend)),
            },

            // 0x10 range - comparison ops.
            Opcode::LT => OpcodeInfo {
                code: 0x10,
                name: "LT".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_lt)),
            },
            Opcode::GT => OpcodeInfo {
                code: 0x11,
                name: "GT".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_gt)),
            },
            Opcode::SLT => OpcodeInfo {
                code: 0x12,
                name: "SLT".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_slt)),
            },
            Opcode::SGT => OpcodeInfo {
                code: 0x13,
                name: "SGT".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_sgt)),
            },
            Opcode::EQ => OpcodeInfo {
                code: 0x14,
                name: "EQ".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_eq)),
            },
            Opcode::ISZERO => OpcodeInfo {
                code: 0x15,
                name: "ISZERO".to_owned(),
                stack_input: 1,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_iszero)),
            },
            Opcode::AND => OpcodeInfo {
                code: 0x16,
                name: "AND".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_and)),
            },
            Opcode::OR => OpcodeInfo {
                code: 0x17,
                name: "OR".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_or)),
            },
            Opcode::XOR => OpcodeInfo {
                code: 0x17,
                name: "XOR".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_xor)),
            },
            Opcode::NOT => OpcodeInfo {
                code: 0x19,
                name: "NOT".to_owned(),
                stack_input: 1,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_not)),
            },
            Opcode::BYTE => OpcodeInfo {
                code: 0x1a,
                name: "BYTE".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::SHL => OpcodeInfo {
                code: 0x1b,
                name: "SHL".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_shl)),
            },
            Opcode::SHR => OpcodeInfo {
                code: 0x1c,
                name: "SHR".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_shr)),
            },
            Opcode::SAR => OpcodeInfo {
                code: 0x1d,
                name: "SAR".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: false,
                function: Some(Box::new(eval_sar)),
            },

            // 0x20 range - crypto.
            Opcode::SHA3 => OpcodeInfo {
                code: 0x20,
                name: "SHA3".to_owned(),
                stack_input: 2,
                stack_output: 1,
                external_effect: true,
                function: None,
            },

            // 0x30 range - closure state.
            Opcode::ADDRESS => OpcodeInfo {
                code: 0x30,
                name: "ADDRESS".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::BALANCE => OpcodeInfo {
                code: 0x31,
                name: "BALANCE".to_owned(),
                stack_input: 1,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::ORIGIN => OpcodeInfo {
                code: 0x32,
                name: "ORIGIN".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::CALLER => OpcodeInfo {
                code: 0x33,
                name: "CALLER".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::CALLVALUE => OpcodeInfo {
                code: 0x34,
                name: "CALLVALUE".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::CALLDATALOAD => OpcodeInfo {
                code: 0x35,
                name: "CALLDATALOAD".to_owned(),
                stack_input: 1,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::CALLDATASIZE => OpcodeInfo {
                code: 0x36,
                name: "CALLDATASIZE".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::CALLDATACOPY => OpcodeInfo {
                code: 0x37,
                name: "CALLDATACOPY".to_owned(),
                stack_input: 3,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::CODESIZE => OpcodeInfo {
                code: 0x38,
                name: "CODESIZE".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::CODECOPY => OpcodeInfo {
                code: 0x39,
                name: "CODECOPY".to_owned(),
                stack_input: 3,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::GASPRICE => OpcodeInfo {
                code: 0x3a,
                name: "GASPRICE".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::EXTCODESIZE => OpcodeInfo {
                code: 0x3b,
                name: "EXTCODESIZE".to_owned(),
                stack_input: 1,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::EXTCODECOPY => OpcodeInfo {
                code: 0x3c,
                name: "EXTCODECOPY".to_owned(),
                stack_input: 4,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::RETURNDATASIZE => OpcodeInfo {
                code: 0x3d,
                name: "RETURNDATASIZE".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::RETURNDATACOPY => OpcodeInfo {
                code: 0x3e,
                name: "RETURNDATACOPY".to_owned(),
                stack_input: 3,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::EXTCODEHASH => OpcodeInfo {
                code: 0x3f,
                name: "EXTCODEHASH".to_owned(),
                stack_input: 1,
                stack_output: 1,
                external_effect: true,
                function: None,
            },

            // 0x40 range - block operations.
            Opcode::BLOCKHASH => OpcodeInfo {
                code: 0x40,
                name: "BLOCKHASH".to_owned(),
                stack_input: 1,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::COINBASE => OpcodeInfo {
                code: 0x41,
                name: "COINBASE".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::TIMESTAMP => OpcodeInfo {
                code: 0x42,
                name: "TIMESTAMP".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::NUMBER => OpcodeInfo {
                code: 0x43,
                name: "NUMBER".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::DIFFICULTY => OpcodeInfo {
                code: 0x44,
                name: "DIFFICULTY".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::GASLIMIT => OpcodeInfo {
                code: 0x45,
                name: "GASLIMIT".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::CHAINID => OpcodeInfo {
                code: 0x46,
                name: "CHAINID".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::SELFBALANCE => OpcodeInfo {
                code: 0x47,
                name: "SELFBALANCE".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::BASEFEE => OpcodeInfo {
                code: 0x48,
                name: "BASEFEE".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },

            // 0x50 range - 'storage' and execution.
            Opcode::POP => OpcodeInfo {
                code: 0x50,
                name: "POP".to_owned(),
                stack_input: 1,
                stack_output: 0,
                external_effect: false,
                function: None,
            },
            Opcode::MLOAD => OpcodeInfo {
                code: 0x51,
                name: "MLOAD".to_owned(),
                stack_input: 1,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::MSTORE => OpcodeInfo {
                code: 0x52,
                name: "MSTORE".to_owned(),
                stack_input: 2,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::MSTORE8 => OpcodeInfo {
                code: 0x53,
                name: "MSTORE8".to_owned(),
                stack_input: 2,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::SLOAD => OpcodeInfo {
                code: 0x54,
                name: "SLOAD".to_owned(),
                stack_input: 1,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::SSTORE => OpcodeInfo {
                code: 0x55,
                name: "SSTORE".to_owned(),
                stack_input: 2,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::JUMP => OpcodeInfo {
                code: 0x56,
                name: "JUMP".to_owned(),
                stack_input: 1,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::JUMPI => OpcodeInfo {
                code: 0x57,
                name: "JUMPI".to_owned(),
                stack_input: 2,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::PC => OpcodeInfo {
                code: 0x58,
                name: "PC".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: false,
                function: None,
            },
            Opcode::MSIZE => OpcodeInfo {
                code: 0x59,
                name: "MSIZE".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::GAS => OpcodeInfo {
                code: 0x5a,
                name: "GAS".to_owned(),
                stack_input: 0,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::JUMPDEST => OpcodeInfo {
                code: 0x5b,
                name: "JUMPDEST".to_owned(),
                stack_input: 0,
                stack_output: 0,
                external_effect: false,
                function: None,
            },

            // 0x60 range - pushes.
            Opcode::PUSH { item_size } => {
                let mut name = String::from("PUSH");
                name.push_str(&item_size.to_string());
                OpcodeInfo {
                    code: 0x5f + *item_size as u8,
                    name,
                    stack_input: 0,
                    stack_output: 1,
                    external_effect: false,
                    function: None,
                }
            }

            // 0x80 range - dups.
            Opcode::DUP { depth } => {
                let mut name = String::from("DUP");
                name.push_str(&depth.to_string());
                OpcodeInfo {
                    code: 0x7f + *depth as u8,
                    name,
                    stack_input: *depth,
                    stack_output: depth + 1,
                    external_effect: false,
                    function: None,
                }
            }

            // 0x90 range - swaps.
            Opcode::SWAP { depth } => {
                let mut name = String::from("SWAP");
                name.push_str(&depth.to_string());
                OpcodeInfo {
                    code: 0x8f + *depth as u8,
                    name,
                    stack_input: *depth + 1,
                    stack_output: *depth + 1,
                    external_effect: false,
                    function: None,
                }
            }

            // 0xa0 range - logging ops.
            Opcode::LOG { topic_count } => {
                let mut name = String::from("LOG");
                name.push_str(&topic_count.to_string());
                OpcodeInfo {
                    code: 0xa0 + *topic_count as u8,
                    name,
                    stack_input: topic_count + 2,
                    stack_output: 0,
                    external_effect: true,
                    function: None,
                }
            }

            // 0xf0 range - closures.
            Opcode::CREATE => OpcodeInfo {
                code: 0xf0,
                name: "CREATE".to_owned(),
                stack_input: 3,
                stack_output: 1,
                external_effect: true,
                function: None,
            },

            Opcode::CALL => OpcodeInfo {
                code: 0xf1,
                name: "CALL".to_owned(),
                stack_input: 7,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::CALLCODE => OpcodeInfo {
                code: 0xf2,
                name: "CALLCODE".to_owned(),
                stack_input: 7,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::RETURN => OpcodeInfo {
                code: 0xf3,
                name: "RETURN".to_owned(),
                stack_input: 2,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::DELEGATECALL => OpcodeInfo {
                code: 0xf4,
                name: "DELEGATECALL".to_owned(),
                stack_input: 6,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::CREATE2 => OpcodeInfo {
                code: 0xf5,
                name: "CREATE2".to_owned(),
                stack_input: 4,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::STATICCALL => OpcodeInfo {
                code: 0xfa,
                name: "STATICCALL".to_owned(),
                stack_input: 6,
                stack_output: 1,
                external_effect: true,
                function: None,
            },
            Opcode::REVERT => OpcodeInfo {
                code: 0xfd,
                name: "REVERT".to_owned(),
                stack_input: 2,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::SELFDESTRUCT => OpcodeInfo {
                code: 0xff,
                name: "SELFDESTRUCT".to_owned(),
                stack_input: 1,
                stack_output: 0,
                external_effect: true,
                function: None,
            },
            Opcode::INVALID { code } => OpcodeInfo {
                code: *code,
                name: "INVALID".to_owned(),
                stack_input: 0,
                stack_output: 0,
                external_effect: false,
                function: None,
            },
        }
    }

    pub fn code(&self) -> u8 {
        return self.opcode_info().code;
    }

    pub fn stack_input(&self) -> usize {
        return self.opcode_info().stack_input;
    }

    pub fn stack_output(&self) -> usize {
        return self.opcode_info().stack_output;
    }

    pub fn name(&self) -> String {
        return self.opcode_info().name;
    }

    pub fn has_effect(&self) -> bool {
        return self.opcode_info().external_effect;
    }

    pub fn get_function(&self) -> Option<Box<dyn Fn(Vec<U256>) -> U256>> {
        return self.opcode_info().function;
    }

    pub fn to_hex(&self) -> String {
        let mut res = String::from("0x");
        res.push_str(&hex::encode([self.code()]));
        return res;
    }

    pub fn is_push(&self) -> bool {
        match self {
            Self::PUSH { item_size: _ } => true,
            _ => false,
        }
    }

    pub fn is_swap(&self) -> bool {
        match self {
            Self::SWAP { depth: _ } => true,
            _ => false,
        }
    }

    pub fn is_dup(&self) -> bool {
        match self {
            Self::DUP { depth: _ } => true,
            _ => false,
        }
    }

    pub fn is_log(&self) -> bool {
        match self {
            Self::LOG { topic_count: _ } => true,
            _ => false,
        }
    }

    pub fn is_invalid(&self) -> bool {
        match self {
            Self::INVALID { code: _ } => true,
            _ => false,
        }
    }

    pub fn is_exiting(&self) -> bool {
        match self {
            Self::STOP | Self::RETURN | Self::REVERT | Self::SELFDESTRUCT => true,
            _ => false,
        }
    }

    pub fn is_jump(&self) -> bool {
        match self {
            Self::JUMP | Self::JUMPI => true,
            _ => false,
        }
    }

    pub fn delta(&self) -> isize {
        return self.stack_output() as isize - self.stack_input() as isize;
    }

    pub fn from(code: u8) -> Opcode {
        return match code {
            // 0x0 range - arithmetic ops.
            0x00 => Opcode::STOP,
            0x01 => Opcode::ADD,
            0x02 => Opcode::MUL,
            0x03 => Opcode::SUB,
            0x04 => Opcode::DIV,
            0x05 => Opcode::SDIV,
            0x06 => Opcode::MOD,
            0x07 => Opcode::SMOD,
            0x08 => Opcode::ADDMOD,
            0x09 => Opcode::MULMOD,
            0x0A => Opcode::EXP,
            0x0B => Opcode::SIGNEXTEND,

            // 0x10 range - comparison ops.
            0x10 => Opcode::LT,
            0x11 => Opcode::GT,
            0x12 => Opcode::SLT,
            0x13 => Opcode::SGT,
            0x14 => Opcode::EQ,
            0x15 => Opcode::ISZERO,
            0x16 => Opcode::AND,
            0x17 => Opcode::OR,
            0x18 => Opcode::XOR,
            0x19 => Opcode::NOT,
            0x1A => Opcode::BYTE,
            0x1B => Opcode::SHL,
            0x1C => Opcode::SHR,
            0x1d => Opcode::SAR,

            // 0x20 range - crypto.
            0x20 => Opcode::SHA3,

            // 0x40 range - block operations.
            0x30 => Opcode::ADDRESS,
            0x31 => Opcode::BALANCE,
            0x32 => Opcode::ORIGIN,
            0x33 => Opcode::CALLER,
            0x34 => Opcode::CALLVALUE,
            0x35 => Opcode::CALLDATALOAD,
            0x36 => Opcode::CALLDATASIZE,
            0x37 => Opcode::CALLDATACOPY,
            0x38 => Opcode::CODESIZE,
            0x39 => Opcode::CODECOPY,
            0x3a => Opcode::GASPRICE,
            0x3b => Opcode::EXTCODESIZE,
            0x3c => Opcode::EXTCODECOPY,
            0x3d => Opcode::RETURNDATASIZE,
            0x3e => Opcode::RETURNDATACOPY,
            0x3f => Opcode::EXTCODEHASH,

            // 0x50 range - 'storage' and execution.
            0x40 => Opcode::BLOCKHASH,
            0x41 => Opcode::COINBASE,
            0x42 => Opcode::TIMESTAMP,
            0x43 => Opcode::NUMBER,
            0x44 => Opcode::DIFFICULTY,
            0x45 => Opcode::GASLIMIT,
            0x46 => Opcode::CHAINID,
            0x47 => Opcode::SELFBALANCE,
            0x48 => Opcode::BASEFEE,

            // 0x50 range - 'storage' and execution.
            0x50 => Opcode::POP,
            0x51 => Opcode::MLOAD,
            0x52 => Opcode::MSTORE,
            0x53 => Opcode::MSTORE8,
            0x54 => Opcode::SLOAD,
            0x55 => Opcode::SSTORE,
            0x56 => Opcode::JUMP,
            0x57 => Opcode::JUMPI,
            0x58 => Opcode::PC,
            0x59 => Opcode::MSIZE,
            0x5A => Opcode::GAS,
            0x5B => Opcode::JUMPDEST,

            // 0x60 range - pushes.
            0x60 => Opcode::PUSH { item_size: 1 },
            0x61 => Opcode::PUSH { item_size: 2 },
            0x62 => Opcode::PUSH { item_size: 3 },
            0x63 => Opcode::PUSH { item_size: 4 },
            0x64 => Opcode::PUSH { item_size: 5 },
            0x65 => Opcode::PUSH { item_size: 6 },
            0x66 => Opcode::PUSH { item_size: 7 },
            0x67 => Opcode::PUSH { item_size: 8 },
            0x68 => Opcode::PUSH { item_size: 9 },
            0x69 => Opcode::PUSH { item_size: 10 },
            0x6a => Opcode::PUSH { item_size: 11 },
            0x6b => Opcode::PUSH { item_size: 12 },
            0x6c => Opcode::PUSH { item_size: 13 },
            0x6d => Opcode::PUSH { item_size: 14 },
            0x6e => Opcode::PUSH { item_size: 15 },
            0x6f => Opcode::PUSH { item_size: 16 },
            0x70 => Opcode::PUSH { item_size: 17 },
            0x71 => Opcode::PUSH { item_size: 18 },
            0x72 => Opcode::PUSH { item_size: 19 },
            0x73 => Opcode::PUSH { item_size: 20 },
            0x74 => Opcode::PUSH { item_size: 21 },
            0x75 => Opcode::PUSH { item_size: 22 },
            0x76 => Opcode::PUSH { item_size: 23 },
            0x77 => Opcode::PUSH { item_size: 24 },
            0x78 => Opcode::PUSH { item_size: 25 },
            0x79 => Opcode::PUSH { item_size: 26 },
            0x7A => Opcode::PUSH { item_size: 27 },
            0x7B => Opcode::PUSH { item_size: 28 },
            0x7C => Opcode::PUSH { item_size: 29 },
            0x7D => Opcode::PUSH { item_size: 30 },
            0x7E => Opcode::PUSH { item_size: 31 },
            0x7F => Opcode::PUSH { item_size: 32 },

            // 0x80 range - dups.
            0x80 => Opcode::DUP { depth: 1 },
            0x81 => Opcode::DUP { depth: 2 },
            0x82 => Opcode::DUP { depth: 3 },
            0x83 => Opcode::DUP { depth: 4 },
            0x84 => Opcode::DUP { depth: 5 },
            0x85 => Opcode::DUP { depth: 6 },
            0x86 => Opcode::DUP { depth: 7 },
            0x87 => Opcode::DUP { depth: 8 },
            0x88 => Opcode::DUP { depth: 9 },
            0x89 => Opcode::DUP { depth: 10 },
            0x8a => Opcode::DUP { depth: 11 },
            0x8b => Opcode::DUP { depth: 12 },
            0x8c => Opcode::DUP { depth: 13 },
            0x8d => Opcode::DUP { depth: 14 },
            0x8e => Opcode::DUP { depth: 15 },
            0x8f => Opcode::DUP { depth: 16 },

            // 0x90 range - swaps.
            0x90 => Opcode::SWAP { depth: 1 },
            0x91 => Opcode::SWAP { depth: 2 },
            0x92 => Opcode::SWAP { depth: 3 },
            0x93 => Opcode::SWAP { depth: 4 },
            0x94 => Opcode::SWAP { depth: 5 },
            0x95 => Opcode::SWAP { depth: 6 },
            0x96 => Opcode::SWAP { depth: 7 },
            0x97 => Opcode::SWAP { depth: 8 },
            0x98 => Opcode::SWAP { depth: 9 },
            0x99 => Opcode::SWAP { depth: 10 },
            0x9a => Opcode::SWAP { depth: 11 },
            0x9b => Opcode::SWAP { depth: 12 },
            0x9c => Opcode::SWAP { depth: 13 },
            0x9d => Opcode::SWAP { depth: 14 },
            0x9e => Opcode::SWAP { depth: 15 },
            0x9f => Opcode::SWAP { depth: 16 },

            // 0xa0 range - logging ops.
            0xa0 => Opcode::LOG { topic_count: 0 },
            0xa1 => Opcode::LOG { topic_count: 1 },
            0xa2 => Opcode::LOG { topic_count: 2 },
            0xa3 => Opcode::LOG { topic_count: 3 },
            0xa4 => Opcode::LOG { topic_count: 4 },

            // 0xf0 range - closures.
            0xf0 => Opcode::CREATE,
            0xf1 => Opcode::CALL,
            0xf2 => Opcode::CALLCODE,
            0xf3 => Opcode::RETURN,
            0xf4 => Opcode::DELEGATECALL,
            0xf5 => Opcode::CREATE2,

            0xfA => Opcode::STATICCALL,

            0xfD => Opcode::REVERT,
            0xfF => Opcode::SELFDESTRUCT,
            other => Opcode::INVALID { code: other },
        };
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

const OFFSET_SIZE_SEP: &str = "::";

pub fn calculation_to_str(opcode: Opcode, get_args: impl Fn(usize, bool) -> String) -> String {
    // get_args(index, is_nested)
    // is_nested == true means that the string argument will be surrounded by parantheses if it's a non trivial expression
    match opcode {
        Opcode::STOP => format!("stop"),
        Opcode::ADD => format!("{} + {}", get_args(0, true), get_args(1, true)),
        Opcode::MUL => format!("{} * {}", get_args(0, true), get_args(1, true)),
        Opcode::SUB => format!("{} - {}", get_args(0, true), get_args(1, true)),
        Opcode::DIV => format!("{} / {}", get_args(0, true), get_args(1, true)),
        Opcode::SDIV => format!("{} / {} (signed)", get_args(0, true), get_args(1, true)),
        Opcode::MOD => format!("{} % {}", get_args(0, true), get_args(1, true)),
        Opcode::SMOD => format!("{} % {} (signed)", get_args(0, true), get_args(1, true)),
        Opcode::ADDMOD => format!("{} % {} mod {}", get_args(0, true), get_args(1, true), get_args(2, true)),
        Opcode::MULMOD => format!("{} * {} mod {}", get_args(0, true), get_args(1, true), get_args(2, true)),
        Opcode::EXP => format!("{} ** {}", get_args(0, true), get_args(1, true)),
        Opcode::SIGNEXTEND => format!("signextend({}, {})", get_args(0, true), get_args(1, true)),
        Opcode::LT => format!("{} < {}", get_args(0, true), get_args(1, true)),
        Opcode::GT => format!("{} > {}", get_args(0, true), get_args(1, true)),
        Opcode::SLT => format!("{} < {} (signed)", get_args(0, true), get_args(1, true)),
        Opcode::SGT => format!("{} > {} (signed)", get_args(0, true), get_args(1, true)),
        Opcode::EQ => format!("{} == {}", get_args(0, true), get_args(1, true)),
        Opcode::ISZERO => format!("!{}", get_args(0, true)),
        Opcode::AND => format!("{} & {}", get_args(0, true), get_args(1, true)),
        Opcode::OR => format!("{} | {}", get_args(0, true), get_args(1, true)),
        Opcode::XOR => format!("{} ^ {}", get_args(0, true), get_args(1, true)),
        Opcode::NOT => format!("~{}", get_args(0, true)),
        Opcode::BYTE => format!("{}-th byte of {}", get_args(0, true), get_args(1, true)),
        Opcode::SHL => format!("{} << {}", get_args(1, true), get_args(0, true)),
        Opcode::SHR => format!("{} >> {}", get_args(1, true), get_args(0, true)),
        Opcode::SAR => format!("{} >> {} (signed)", get_args(1, true), get_args(0, true)),
        Opcode::SHA3 => format!("sha3[{}{OFFSET_SIZE_SEP}{}]", get_args(0, true), get_args(1, true)),
        Opcode::ADDRESS => format!("address(this)"),
        Opcode::BALANCE => format!("address({}).balance", get_args(0, false)),
        Opcode::ORIGIN => format!("tx.origin"),
        Opcode::CALLER => format!("msg.sender"),
        Opcode::CALLVALUE => format!("msg.value"),
        Opcode::CALLDATALOAD => format!("calldata[{}]", get_args(0, false)),
        Opcode::CALLDATASIZE => format!("calldatasize"),
        Opcode::CALLDATACOPY => format!(
            "memory[{}..] = calldata[{}{OFFSET_SIZE_SEP}{}]",
            get_args(0, false), get_args(1, true), get_args(2, true)
        ),
        Opcode::CODESIZE => format!("codesize"),
        Opcode::CODECOPY => format!(
            "memory[{}..] = code(this)[{}{OFFSET_SIZE_SEP}{}]",
            get_args(0, false), get_args(1, true), get_args(2, true)
        ),
        Opcode::GASPRICE => format!("gasprice"),
        Opcode::EXTCODESIZE => format!("extcodesize({})", get_args(0, true)),
        Opcode::EXTCODECOPY => format!(
            "memory[{}..] = code({})[{}{OFFSET_SIZE_SEP}{}]",
            get_args(1, false), get_args(0, false), get_args(2, true), get_args(3, true)
        ),
        Opcode::RETURNDATASIZE => format!("returndatasize"),
        Opcode::RETURNDATACOPY => format!(
            "memory[{}..] = returndata[{}{OFFSET_SIZE_SEP}{}]",
            get_args(0, true), get_args(1, true), get_args(2, true)
        ),
        Opcode::EXTCODEHASH => format!("codehash({})", get_args(0, true)),
        Opcode::BLOCKHASH => format!("blockhash({})", get_args(0, true)),
        Opcode::COINBASE => format!("coinbase"),
        Opcode::TIMESTAMP => format!("block.timestamp"),
        Opcode::NUMBER => format!("block.number"),
        Opcode::DIFFICULTY => format!("block.difficulty"),
        Opcode::GASLIMIT => format!("gasLeft()"),
        Opcode::CHAINID => format!("block.chainid"),
        Opcode::SELFBALANCE => format!("address(this).balance"),
        Opcode::BASEFEE => format!("block.basefee"),
        Opcode::POP => panic!("POP should not be displayed at a high level."),
        Opcode::MLOAD => format!("memory[{}]", get_args(0, false)),
        Opcode::MSTORE => format!("memory[{}] = {}", get_args(0, false), get_args(1, false)), // general case: we do do show that the size is 32 bytes
        Opcode::MSTORE8 => format!("memory[{}] = {} (1 byte)", get_args(0, false), get_args(1, false)), // particular case
        Opcode::SLOAD => format!("storage[{}]", get_args(0, false)),
        Opcode::SSTORE => format!("storage[{}] = {}", get_args(0, false), get_args(1, false)),
        Opcode::JUMP => panic!("JUMP should not be displayed at a high level."),
        Opcode::JUMPI => panic!("JUMPI should not be displayed at a high level."),
        Opcode::PC => format!("PC"), // TODO fill it with the integer value when building the graph
        Opcode::MSIZE => format!("msize"),
        Opcode::GAS => format!("gasleft()"),
        Opcode::JUMPDEST => panic!("JUMPDEST should not be displayed at a high level."),
        Opcode::PUSH { item_size: _ } => panic!("PUSH should not be displayed at a high level."),
        Opcode::DUP { depth: _ } => panic!("DUP should not be displayed at a high level."),
        Opcode::SWAP { depth: _ } => panic!("SWAP should not be displayed at a high level."),
        Opcode::LOG { topic_count } => {
            let mut res = format!("log[{}{OFFSET_SIZE_SEP}{}]", get_args(0, true), get_args(1, true));
            for topic_index in 0..topic_count {
                res += &format!(", {}", get_args(2 + topic_index, true));
            }
            res += ")";
            return res;
        }
        Opcode::CREATE => format!(
            "create(value: {}, code: [{}{OFFSET_SIZE_SEP}{}])",
            get_args(0, false), get_args(1, true), get_args(2, true)
        ),
        Opcode::CALL => format!(
            "call(gas: {}, address: {}, value: {}, args: [{}{OFFSET_SIZE_SEP}{}]), res: [{}{OFFSET_SIZE_SEP}{}])",
            get_args(0, false), get_args(1, false), get_args(2, false), get_args(3, true), get_args(4, true),get_args(5, true), get_args(6, true)
        ),
        Opcode::CALLCODE => format!(
            "callcode(gas: {}, address: {}, value: {}, args: [{}{OFFSET_SIZE_SEP}{}]), res: [{}{OFFSET_SIZE_SEP}{}]",
            get_args(0, false), get_args(1, false), get_args(2, false), get_args(3, true), get_args(4, true), get_args(5, true), get_args(6, true)
        ),
        Opcode::RETURN => format!("return[{}{OFFSET_SIZE_SEP}{}]", get_args(0, true), get_args(1, true)),
        Opcode::DELEGATECALL => format!(
            "delegatecall(gas: {}, address: {}, args: [{}{OFFSET_SIZE_SEP}{}], res: [{}{OFFSET_SIZE_SEP}{}] )",
            get_args(0, false), get_args(1, false), get_args(2, true), get_args(3, true), get_args(4, true), get_args(5, true),
        ),
        Opcode::CREATE2 => format!(
            "create(value: {}, code: [{}{OFFSET_SIZE_SEP}{}], salt: {})",
            get_args(0, false), get_args(1, true), get_args(2, true), get_args(3, false)
        ),
        Opcode::STATICCALL => format!(
            "staticcall(gas: {}, address: {}, args: [{}{OFFSET_SIZE_SEP}{}], res: [{}{OFFSET_SIZE_SEP}{}] )",
            get_args(0, false), get_args(1, false), get_args(2, true), get_args(3, true), get_args(4, true), get_args(5, true),
        ),
        Opcode::REVERT => format!("revert[{}{OFFSET_SIZE_SEP}{}]", get_args(0, true), get_args(1, true)),
        Opcode::SELFDESTRUCT => format!("seldestruct({})", get_args(0, false)),
        Opcode::INVALID { code: _ } => panic!("INVALID should not be displayed at a high level."),
    }
}
