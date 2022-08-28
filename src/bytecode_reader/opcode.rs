use const_decoder::Decoder;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(Default, Copy, Clone, Debug, EnumIter, PartialEq, Eq, Hash, IntoStaticStr)]
pub enum Opcode {
    STOP = Trait::from(b"00", 0, true).encode(),
    ADD = Trait::from(b"01", 2, true).encode(),
    MUL = Trait::from(b"02", 2, true).encode(),
    SUB = Trait::from(b"03", 2, true).encode(),
    DIV = Trait::from(b"04", 2, true).encode(),
    SDIV = Trait::from(b"05", 2, true).encode(),
    MOD = Trait::from(b"06", 2, true).encode(),
    SMOD = Trait::from(b"07", 2, true).encode(),
    ADDMOD = Trait::from(b"08", 3, true).encode(),
    MULMOD = Trait::from(b"09", 3, true).encode(),
    EXP = Trait::from(b"0A", 2, true).encode(),
    SIGNEXTEND = Trait::from(b"0B", 2, true).encode(),

    LT = Trait::from(b"10", 2, true).encode(),
    GT = Trait::from(b"11", 2, true).encode(),
    SLT = Trait::from(b"12", 2, true).encode(),
    SGT = Trait::from(b"13", 2, true).encode(),
    EQ = Trait::from(b"14", 2, true).encode(),
    ISZERO = Trait::from(b"15", 1, true).encode(),
    AND = Trait::from(b"16", 2, true).encode(),
    OR = Trait::from(b"17", 2, true).encode(),
    XOR = Trait::from(b"18", 2, true).encode(),
    NOT = Trait::from(b"19", 1, true).encode(),
    BYTE = Trait::from(b"1a", 2, true).encode(),
    SHL = Trait::from(b"1b", 2, true).encode(),
    SHR = Trait::from(b"1c", 2, true).encode(),
    SAR = Trait::from(b"1d", 2, true).encode(),

    SHA3 = Trait::from(b"20", 2, true).encode(),

    ADDRESS = Trait::from(b"30", 0, true).encode(),
    BALANCE = Trait::from(b"31", 1, true).encode(),
    ORIGIN = Trait::from(b"32", 0, true).encode(),
    CALLER = Trait::from(b"33", 0, true).encode(),
    CALLVALUE = Trait::from(b"34", 0, true).encode(),
    CALLDATALOAD = Trait::from(b"35", 1, true).encode(),
    CALLDATASIZE = Trait::from(b"36", 0, true).encode(),
    CALLDATACOPY = Trait::from(b"37", 3, false).encode(),
    CODESIZE = Trait::from(b"38", 0, true).encode(),
    CODECOPY = Trait::from(b"39", 3, false).encode(),
    GASPRICE = Trait::from(b"3a", 0, true).encode(),
    EXTCODESIZE = Trait::from(b"3b", 1, true).encode(),
    EXTCODECOPY = Trait::from(b"3c", 4, false).encode(),
    RETURNDATASIZE = Trait::from(b"3d", 0, true).encode(),
    RETURNDATACOPY = Trait::from(b"3e", 3, false).encode(),
    EXTCODEHASH = Trait::from(b"3f", 1, true).encode(),
    BLOCKHASH = Trait::from(b"40", 1, true).encode(),
    COINBASE = Trait::from(b"41", 0, true).encode(),
    TIMESTAMP = Trait::from(b"42", 0, true).encode(),
    NUMBER = Trait::from(b"43", 0, true).encode(),
    DIFFICULTY = Trait::from(b"44", 0, true).encode(),
    GASLIMIT = Trait::from(b"45", 0, true).encode(),
    CHAINID = Trait::from(b"46", 0, true).encode(),
    SELFBALANCE = Trait::from(b"47", 0, true).encode(),
    BASEFEE = Trait::from(b"48", 0, true).encode(),

    POP = Trait::from(b"50", 1, false).encode(),
    MLOAD = Trait::from(b"51", 1, true).encode(),
    MSTORE = Trait::from(b"52", 2, false).encode(),
    MSTORE8 = Trait::from(b"53", 2, false).encode(),
    SLOAD = Trait::from(b"54", 1, true).encode(),
    SSTORE = Trait::from(b"55", 2, false).encode(),
    JUMP = Trait::from(b"56", 1, false).encode(),
    JUMPI = Trait::from(b"57", 2, false).encode(),
    PC = Trait::from(b"58", 0, true).encode(),
    MSIZE = Trait::from(b"59", 0, true).encode(),
    GAS = Trait::from(b"5a", 0, true).encode(),
    JUMPDEST = Trait::from(b"5b", 0, false).encode(),

    PUSH1 = Trait::from(b"60", 0, true).encode(),
    PUSH2 = Trait::from(b"61", 0, true).encode(),
    PUSH3 = Trait::from(b"62", 0, true).encode(),
    PUSH4 = Trait::from(b"63", 0, true).encode(),
    PUSH5 = Trait::from(b"64", 0, true).encode(),
    PUSH6 = Trait::from(b"65", 0, true).encode(),
    PUSH7 = Trait::from(b"66", 0, true).encode(),
    PUSH8 = Trait::from(b"67", 0, true).encode(),
    PUSH9 = Trait::from(b"68", 0, true).encode(),
    PUSH10 = Trait::from(b"69", 0, true).encode(),
    PUSH11 = Trait::from(b"6a", 0, true).encode(),
    PUSH12 = Trait::from(b"6b", 0, true).encode(),
    PUSH13 = Trait::from(b"6c", 0, true).encode(),
    PUSH14 = Trait::from(b"6d", 0, true).encode(),
    PUSH15 = Trait::from(b"6e", 0, true).encode(),
    PUSH16 = Trait::from(b"6f", 0, true).encode(),
    PUSH17 = Trait::from(b"70", 0, true).encode(),
    PUSH18 = Trait::from(b"71", 0, true).encode(),
    PUSH19 = Trait::from(b"72", 0, true).encode(),
    PUSH20 = Trait::from(b"73", 0, true).encode(),
    PUSH21 = Trait::from(b"74", 0, true).encode(),
    PUSH22 = Trait::from(b"75", 0, true).encode(),
    PUSH23 = Trait::from(b"76", 0, true).encode(),
    PUSH24 = Trait::from(b"77", 0, true).encode(),
    PUSH25 = Trait::from(b"78", 0, true).encode(),
    PUSH26 = Trait::from(b"79", 0, true).encode(),
    PUSH27 = Trait::from(b"7a", 0, true).encode(),
    PUSH28 = Trait::from(b"7b", 0, true).encode(),
    PUSH29 = Trait::from(b"7c", 0, true).encode(),
    PUSH30 = Trait::from(b"7d", 0, true).encode(),
    PUSH31 = Trait::from(b"7e", 0, true).encode(),
    PUSH32 = Trait::from(b"7f", 0, true).encode(),

    DUP1 = Trait::from(b"80", 1, false).encode(),
    DUP2 = Trait::from(b"81", 2, false).encode(),
    DUP3 = Trait::from(b"82", 3, false).encode(),
    DUP4 = Trait::from(b"83", 4, false).encode(),
    DUP5 = Trait::from(b"84", 5, false).encode(),
    DUP6 = Trait::from(b"85", 6, false).encode(),
    DUP7 = Trait::from(b"86", 7, false).encode(),
    DUP8 = Trait::from(b"87", 8, false).encode(),
    DUP9 = Trait::from(b"88", 9, false).encode(),
    DUP10 = Trait::from(b"89", 10, false).encode(),
    DUP11 = Trait::from(b"8a", 11, false).encode(),
    DUP12 = Trait::from(b"8b", 12, false).encode(),
    DUP13 = Trait::from(b"8c", 13, false).encode(),
    DUP14 = Trait::from(b"8d", 14, false).encode(),
    DUP15 = Trait::from(b"8e", 15, false).encode(),
    DUP18 = Trait::from(b"8f", 16, false).encode(),

    SWAP1 = Trait::from(b"90", 2, false).encode(),
    SWAP2 = Trait::from(b"91", 3, false).encode(),
    SWAP3 = Trait::from(b"92", 4, false).encode(),
    SWAP4 = Trait::from(b"93", 5, false).encode(),
    SWAP5 = Trait::from(b"94", 6, false).encode(),
    SWAP6 = Trait::from(b"95", 7, false).encode(),
    SWAP7 = Trait::from(b"96", 8, false).encode(),
    SWAP8 = Trait::from(b"97", 9, false).encode(),
    SWAP9 = Trait::from(b"98", 10, false).encode(),
    SWAP10 = Trait::from(b"99", 11, false).encode(),
    SWAP11 = Trait::from(b"9a", 12, false).encode(),
    SWAP12 = Trait::from(b"9b", 13, false).encode(),
    SWAP13 = Trait::from(b"9c", 14, false).encode(),
    SWAP14 = Trait::from(b"9d", 15, false).encode(),
    SWAP15 = Trait::from(b"9e", 16, false).encode(),
    SWAP19 = Trait::from(b"9f", 17, false).encode(),

    LOG0 = Trait::from(b"a0", 2, false).encode(),
    LOG1 = Trait::from(b"a1", 3, false).encode(),
    LOG2 = Trait::from(b"a2", 4, false).encode(),
    LOG3 = Trait::from(b"a3", 5, false).encode(),
    LOG4 = Trait::from(b"a4", 6, false).encode(),

    CREATE = Trait::from(b"f0", 3, true).encode(),
    CALL = Trait::from(b"f1", 7, true).encode(),
    CALLCODE = Trait::from(b"f2", 7, true).encode(),
    RETURN = Trait::from(b"f3", 2, false).encode(),
    DELEGATECALL = Trait::from(b"f4", 6, true).encode(),
    CREATE2 = Trait::from(b"f5", 4, true).encode(),

    STATICCALL = Trait::from(b"fa", 6, true).encode(),

    REVERT = Trait::from(b"fd", 2, false).encode(),
    SELFDESTRUCT = Trait::from(b"ff", 1, false).encode(),

    #[default]
    INVALID = Trait::from(b"fe", 0, false).encode(),
}

impl Opcode {
    pub fn from(hex_symbol: &str) -> Self {
        match HEX_TO_OPCODE
            .lock()
            .unwrap()
            .get(&u8::from_str_radix(hex_symbol, 16).unwrap())
        {
            Some(opcode) => *opcode,
            None => Opcode::INVALID,
        }
    }

    pub fn from_u8(from: u8) -> Self {
        match HEX_TO_OPCODE.lock().unwrap().get(&from) {
            Some(opcode) => *opcode,
            None => Opcode::INVALID,
        }
    }

    pub fn n_stack_input(&self) -> usize {
        return OPCODE_INFO.lock().unwrap()[self].n_stack_input as usize;
    }

    pub fn has_stack_output(&self) -> bool {
        return OPCODE_INFO.lock().unwrap()[self].stack_output;
    }

    pub fn hex_code(&self) -> String {
        let res: String = format!("{:x}", OPCODE_INFO.lock().unwrap()[self].hex_symbol);
        if res.len() == 2 {
            res
        } else {
            "0".to_owned() + &res
        }
    }

    pub fn to_string(&self) -> String {
        let res: &'static str = self.into();
        return res.to_string();
    }

    fn as_repetitive_opcode(&self, base_name: &str) -> Option<usize> {
        let name: String = self.to_string();
        if name.len() >= base_name.len() + 1 && &name[0..base_name.len()] == base_name {
            return Some(name[base_name.len()..].parse().unwrap());
        } else {
            return None;
        }
    }

    pub fn as_push(&self) -> Option<usize> {
        return self.as_repetitive_opcode("PUSH");
    }
    pub fn as_swap(&self) -> Option<usize> {
        return self.as_repetitive_opcode("SWAP");
    }
    pub fn as_dup(&self) -> Option<usize> {
        return self.as_repetitive_opcode("DUP");
    }
    pub fn as_log(&self) -> Option<usize> {
        return self.as_repetitive_opcode("LOG");
    }
}

lazy_static! {
    static ref HEX_TO_OPCODE: Mutex<HashMap<u8, Opcode>> = Mutex::new(HashMap::new());
    static ref OPCODE_INFO: Mutex<HashMap<Opcode, Trait>> = Mutex::new(HashMap::new());
}
struct Trait {
    hex_symbol: u8,     // the hex code of the opcode
    n_stack_input: u8,  // number of elements consumed on the stack
    stack_output: bool, // does it produce an element on the stack
}

impl Trait {
    const fn from(hex_symbol: &[u8], n_stack_input: u8, stack_output: bool) -> Self {
        return Trait {
            hex_symbol: hex_to_u8(hex_symbol),
            n_stack_input,
            stack_output,
        };
    }
    const fn encode(&self) -> isize {
        return (self.hex_symbol as isize) * 2isize.pow(9)
            + (self.n_stack_input as isize) * 2isize.pow(1)
            + (self.stack_output as isize);
    }
    const fn decode(encoded: isize) -> Self {
        return Trait {
            hex_symbol: (encoded / 2isize.pow(9)) as u8,
            n_stack_input: ((encoded / 2) % 2isize.pow(8)) as u8,
            stack_output: (encoded % 2) != 0,
        };
    }
}

pub fn init_opcodes() {
    for opcode in Opcode::iter() {
        let encoded: isize = opcode as isize;
        let traits: Trait = Trait::decode(encoded);
        HEX_TO_OPCODE
            .lock()
            .unwrap()
            .insert(traits.hex_symbol, opcode);
        OPCODE_INFO.lock().unwrap().insert(opcode, traits);
    }
}

const fn hex_to_u8(hex_str: &[u8]) -> u8 {
    // hex_str: 2 characters in {0, 1 ... 9, a, b ... f}
    return (Decoder::Hex.decode(hex_str) as [u8; 1])[0];
}
