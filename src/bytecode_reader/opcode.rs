use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    PUSH { n_bytes: usize },

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
    stack_input: usize,
    stack_output: usize,
}

impl Opcode {
    fn opcode_info(&self) -> OpcodeInfo {
        match self {
            // 0x0 range - arithmetic ops.
            Opcode::STOP => OpcodeInfo {
                code: 0x00,
                stack_input: 0,
                stack_output: 0,
            },
            Opcode::ADD => OpcodeInfo {
                code: 0x01,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::MUL => OpcodeInfo {
                code: 0x02,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::SUB => OpcodeInfo {
                code: 0x03,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::DIV => OpcodeInfo {
                code: 0x04,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::SDIV => OpcodeInfo {
                code: 0x05,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::MOD => OpcodeInfo {
                code: 0x06,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::SMOD => OpcodeInfo {
                code: 0x07,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::ADDMOD => OpcodeInfo {
                code: 0x08,
                stack_input: 3,
                stack_output: 1,
            },
            Opcode::MULMOD => OpcodeInfo {
                code: 0x09,
                stack_input: 3,
                stack_output: 1,
            },
            Opcode::EXP => OpcodeInfo {
                code: 0x0a,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::SIGNEXTEND => OpcodeInfo {
                code: 0x0b,
                stack_input: 2,
                stack_output: 1,
            },

            // 0x10 range - comparison ops.
            Opcode::LT => OpcodeInfo {
                code: 0x10,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::GT => OpcodeInfo {
                code: 0x11,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::SLT => OpcodeInfo {
                code: 0x12,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::SGT => OpcodeInfo {
                code: 0x13,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::EQ => OpcodeInfo {
                code: 0x14,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::ISZERO => OpcodeInfo {
                code: 0x15,
                stack_input: 1,
                stack_output: 1,
            },
            Opcode::AND => OpcodeInfo {
                code: 0x16,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::OR => OpcodeInfo {
                code: 0x17,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::XOR => OpcodeInfo {
                code: 0x17,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::NOT => OpcodeInfo {
                code: 0x19,
                stack_input: 1,
                stack_output: 1,
            },
            Opcode::BYTE => OpcodeInfo {
                code: 0x1a,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::SHL => OpcodeInfo {
                code: 0x1b,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::SHR => OpcodeInfo {
                code: 0x1c,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::SAR => OpcodeInfo {
                code: 0x1d,
                stack_input: 2,
                stack_output: 1,
            },

            // 0x20 range - crypto.
            Opcode::SHA3 => OpcodeInfo {
                code: 0x20,
                stack_input: 2,
                stack_output: 1,
            },

            // 0x30 range - closure state.
            Opcode::ADDRESS => OpcodeInfo {
                code: 0x20,
                stack_input: 2,
                stack_output: 1,
            },
            Opcode::BALANCE => OpcodeInfo {
                code: 0x31,
                stack_input: 1,
                stack_output: 1,
            },
            Opcode::ORIGIN => OpcodeInfo {
                code: 0x32,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::CALLER => OpcodeInfo {
                code: 0x33,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::CALLVALUE => OpcodeInfo {
                code: 0x34,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::CALLDATALOAD => OpcodeInfo {
                code: 0x35,
                stack_input: 1,
                stack_output: 1,
            },
            Opcode::CALLDATASIZE => OpcodeInfo {
                code: 0x36,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::CALLDATACOPY => OpcodeInfo {
                code: 0x37,
                stack_input: 3,
                stack_output: 0,
            },
            Opcode::CODESIZE => OpcodeInfo {
                code: 0x38,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::CODECOPY => OpcodeInfo {
                code: 0x39,
                stack_input: 3,
                stack_output: 0,
            },
            Opcode::GASPRICE => OpcodeInfo {
                code: 0x3a,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::EXTCODESIZE => OpcodeInfo {
                code: 0x3b,
                stack_input: 1,
                stack_output: 1,
            },
            Opcode::EXTCODECOPY => OpcodeInfo {
                code: 0x3c,
                stack_input: 4,
                stack_output: 0,
            },
            Opcode::RETURNDATASIZE => OpcodeInfo {
                code: 0x3d,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::RETURNDATACOPY => OpcodeInfo {
                code: 0x3e,
                stack_input: 3,
                stack_output: 0,
            },
            Opcode::EXTCODEHASH => OpcodeInfo {
                code: 0x3f,
                stack_input: 1,
                stack_output: 1,
            },

            // 0x40 range - block operations.
            Opcode::BLOCKHASH => OpcodeInfo {
                code: 0x40,
                stack_input: 1,
                stack_output: 1,
            },
            Opcode::COINBASE => OpcodeInfo {
                code: 0x41,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::TIMESTAMP => OpcodeInfo {
                code: 0x42,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::NUMBER => OpcodeInfo {
                code: 0x43,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::DIFFICULTY => OpcodeInfo {
                code: 0x44,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::GASLIMIT => OpcodeInfo {
                code: 0x45,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::CHAINID => OpcodeInfo {
                code: 0x46,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::SELFBALANCE => OpcodeInfo {
                code: 0x47,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::BASEFEE => OpcodeInfo {
                code: 0x48,
                stack_input: 0,
                stack_output: 1,
            },

            // 0x50 range - 'storage' and execution.
            Opcode::POP => OpcodeInfo {
                code: 0x50,
                stack_input: 1,
                stack_output: 0,
            },
            Opcode::MLOAD => OpcodeInfo {
                code: 0x51,
                stack_input: 1,
                stack_output: 1,
            },
            Opcode::MSTORE => OpcodeInfo {
                code: 0x52,
                stack_input: 2,
                stack_output: 0,
            },
            Opcode::MSTORE8 => OpcodeInfo {
                code: 0x53,
                stack_input: 2,
                stack_output: 0,
            },
            Opcode::SLOAD => OpcodeInfo {
                code: 0x54,
                stack_input: 1,
                stack_output: 1,
            },
            Opcode::SSTORE => OpcodeInfo {
                code: 0x55,
                stack_input: 2,
                stack_output: 0,
            },
            Opcode::JUMP => OpcodeInfo {
                code: 0x56,
                stack_input: 1,
                stack_output: 0,
            },
            Opcode::JUMPI => OpcodeInfo {
                code: 0x57,
                stack_input: 2,
                stack_output: 0,
            },
            Opcode::PC => OpcodeInfo {
                code: 0x58,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::MSIZE => OpcodeInfo {
                code: 0x59,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::GAS => OpcodeInfo {
                code: 0x5a,
                stack_input: 0,
                stack_output: 1,
            },
            Opcode::JUMPDEST => OpcodeInfo {
                code: 0x5b,
                stack_input: 0,
                stack_output: 0,
            },

            // 0x60 range - pushes.
            Opcode::PUSH { n_bytes } => OpcodeInfo {
                code: 0x5f + *n_bytes as u8,
                stack_input: 0,
                stack_output: 1,
            },

            // 0x80 range - dups.
            Opcode::DUP { depth } => OpcodeInfo {
                code: 0x7f + *depth as u8,
                stack_input: *depth,
                stack_output: 0,
            },

            // 0x90 range - swaps.
            Opcode::SWAP { depth } => OpcodeInfo {
                code: 0x8f + *depth as u8,
                stack_input: *depth,
                stack_output: 0,
            },

            // 0xa0 range - logging ops.
            Opcode::LOG { topic_count } => OpcodeInfo {
                code: 0xa0 + *topic_count as u8,
                stack_input: topic_count + 2,
                stack_output: 0,
            },

            // 0xf0 range - closures.
            Opcode::CREATE => OpcodeInfo {
                code: 0xf0,
                stack_input: 3,
                stack_output: 1,
            },

            Opcode::CALL => OpcodeInfo {
                code: 0xf1,
                stack_input: 7,
                stack_output: 1,
            },
            Opcode::CALLCODE => OpcodeInfo {
                code: 0xf2,
                stack_input: 7,
                stack_output: 1,
            },
            Opcode::RETURN => OpcodeInfo {
                code: 0xf3,
                stack_input: 2,
                stack_output: 0,
            },
            Opcode::DELEGATECALL => OpcodeInfo {
                code: 0xf4,
                stack_input: 6,
                stack_output: 0,
            },
            Opcode::CREATE2 => OpcodeInfo {
                code: 0xf5,
                stack_input: 4,
                stack_output: 1,
            },
            Opcode::STATICCALL => OpcodeInfo {
                code: 0xfa,
                stack_input: 6,
                stack_output: 1,
            },
            Opcode::REVERT => OpcodeInfo {
                code: 0xfd,
                stack_input: 2,
                stack_output: 0,
            },
            Opcode::SELFDESTRUCT => OpcodeInfo {
                code: 0xff,
                stack_input: 1,
                stack_output: 0,
            },
            Opcode::INVALID { code } => OpcodeInfo {
                code: *code,
                stack_input: 0,
                stack_output: 0,
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

    pub fn is_push(&self) -> bool {
        if let Self::PUSH { n_bytes } = self {
            true
        } else {
            false
        }
    }

    pub fn is_swap(&self) -> bool {
        if let Self::SWAP { depth } = self {
            true
        } else {
            false
        }
    }
    pub fn is_dup(&self) -> bool {
        if let Self::DUP { depth } = self {
            true
        } else {
            false
        }
    }

    pub fn is_log(&self) -> bool {
        if let Self::LOG { topic_count } = self {
            true
        } else {
            false
        }
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
            0x60 => Opcode::PUSH { n_bytes: 1 },
            0x61 => Opcode::PUSH { n_bytes: 2 },
            0x62 => Opcode::PUSH { n_bytes: 3 },
            0x63 => Opcode::PUSH { n_bytes: 4 },
            0x64 => Opcode::PUSH { n_bytes: 5 },
            0x65 => Opcode::PUSH { n_bytes: 6 },
            0x66 => Opcode::PUSH { n_bytes: 7 },
            0x67 => Opcode::PUSH { n_bytes: 8 },
            0x68 => Opcode::PUSH { n_bytes: 9 },
            0x69 => Opcode::PUSH { n_bytes: 10 },
            0x6a => Opcode::PUSH { n_bytes: 11 },
            0x6b => Opcode::PUSH { n_bytes: 12 },
            0x6c => Opcode::PUSH { n_bytes: 13 },
            0x6d => Opcode::PUSH { n_bytes: 14 },
            0x6e => Opcode::PUSH { n_bytes: 15 },
            0x6f => Opcode::PUSH { n_bytes: 16 },
            0x70 => Opcode::PUSH { n_bytes: 17 },
            0x71 => Opcode::PUSH { n_bytes: 18 },
            0x72 => Opcode::PUSH { n_bytes: 19 },
            0x73 => Opcode::PUSH { n_bytes: 20 },
            0x74 => Opcode::PUSH { n_bytes: 21 },
            0x75 => Opcode::PUSH { n_bytes: 22 },
            0x76 => Opcode::PUSH { n_bytes: 23 },
            0x77 => Opcode::PUSH { n_bytes: 24 },
            0x78 => Opcode::PUSH { n_bytes: 25 },
            0x79 => Opcode::PUSH { n_bytes: 26 },
            0x7A => Opcode::PUSH { n_bytes: 27 },
            0x7B => Opcode::PUSH { n_bytes: 28 },
            0x7C => Opcode::PUSH { n_bytes: 29 },
            0x7D => Opcode::PUSH { n_bytes: 30 },
            0x7E => Opcode::PUSH { n_bytes: 31 },
            0x7F => Opcode::PUSH { n_bytes: 32 },

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
