
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Opcode {
    pub code: u8,
    pub stack_input: usize,
    pub stack_output: usize,
    pub n: usize // value of N in case of a PUSHN, DUPN, SWAPN or LOGN
}

// 0x0 range - arithmetic ops.
pub const STOP : Opcode = Opcode {code: 0x00, stack_input: 0, stack_output: 0, n: 0};
pub const ADD : Opcode = Opcode {code: 0x01, stack_input: 2, stack_output: 1, n: 0};
pub const MUL : Opcode = Opcode {code: 0x02, stack_input: 2, stack_output: 1, n: 0};
pub const SUB : Opcode = Opcode {code: 0x03, stack_input: 2, stack_output: 1, n: 0};
pub const DIV : Opcode = Opcode {code: 0x04, stack_input: 2, stack_output: 1, n: 0};
pub const SDIV : Opcode = Opcode {code: 0x05, stack_input: 2, stack_output: 1, n: 0};
pub const MOD : Opcode = Opcode {code: 0x06, stack_input: 2, stack_output: 1, n: 0};
pub const SMOD : Opcode = Opcode {code: 0x07, stack_input: 2, stack_output: 1, n: 0};
pub const ADDMOD : Opcode = Opcode {code: 0x08, stack_input: 3, stack_output: 1, n: 0};
pub const MULMOD : Opcode = Opcode {code: 0x09, stack_input: 3, stack_output: 1, n: 0};
pub const EXP : Opcode = Opcode {code: 0x0a, stack_input: 2, stack_output: 1, n: 0};
pub const SIGNEXTEND : Opcode = Opcode {code: 0x0b, stack_input: 2, stack_output: 1, n: 0};

// 0x10 range - comparison ops.
pub const LT : Opcode = Opcode {code: 0x10, stack_input: 2, stack_output: 1, n: 0};
pub const GT : Opcode = Opcode {code: 0x11, stack_input: 2, stack_output: 1, n: 0};
pub const SLT : Opcode = Opcode {code: 0x12, stack_input: 2, stack_output: 1, n: 0};
pub const SGT : Opcode = Opcode {code: 0x13, stack_input: 2, stack_output: 1, n: 0};
pub const EQ : Opcode = Opcode {code: 0x14, stack_input: 2, stack_output: 1, n: 0};
pub const ISZERO : Opcode = Opcode {code: 0x15, stack_input: 1, stack_output: 1, n: 0};
pub const AND : Opcode = Opcode {code: 0x16, stack_input: 2, stack_output: 1, n: 0};
pub const OR : Opcode = Opcode {code: 0x17, stack_input: 2, stack_output: 1, n: 0};
pub const XOR : Opcode = Opcode {code: 0x18, stack_input: 2, stack_output: 1, n: 0};
pub const NOT : Opcode = Opcode {code: 0x19, stack_input: 1, stack_output: 1, n: 0};
pub const BYTE : Opcode = Opcode {code: 0x1a, stack_input: 2, stack_output: 1, n: 0};
pub const SHL : Opcode = Opcode {code: 0x1b, stack_input: 2, stack_output: 1, n: 0};
pub const SHR : Opcode = Opcode {code: 0x1c, stack_input: 2, stack_output: 1, n: 0};
pub const SAR : Opcode = Opcode {code: 0x1d, stack_input: 2, stack_output: 1, n: 0};

// 0x20 range - crypto.
pub const SHA3 : Opcode = Opcode {code: 0x20, stack_input: 2, stack_output: 1, n: 0};

// 0x30 range - closure state.
pub const ADDRESS : Opcode = Opcode {code: 0x30, stack_input: 0, stack_output: 1, n: 0};
pub const BALANCE : Opcode = Opcode {code: 0x31, stack_input: 1, stack_output: 1, n: 0};
pub const ORIGIN : Opcode = Opcode {code: 0x32, stack_input: 0, stack_output: 1, n: 0};
pub const CALLER : Opcode = Opcode {code: 0x33, stack_input: 0, stack_output: 1, n: 0};
pub const CALLVALUE : Opcode = Opcode {code: 0x34, stack_input: 0, stack_output: 1, n: 0};
pub const CALLDATALOAD : Opcode = Opcode {code: 0x35, stack_input: 1, stack_output: 1, n: 0};
pub const CALLDATASIZE : Opcode = Opcode {code: 0x36, stack_input: 0, stack_output: 1, n: 0};
pub const CALLDATACOPY : Opcode = Opcode {code: 0x37, stack_input: 3, stack_output: 0, n: 0};
pub const CODESIZE : Opcode = Opcode {code: 0x38, stack_input: 0, stack_output: 1, n: 0};
pub const CODECOPY : Opcode = Opcode {code: 0x39, stack_input: 3, stack_output: 0, n: 0};
pub const GASPRICE : Opcode = Opcode {code: 0x3a, stack_input: 0, stack_output: 1, n: 0};
pub const EXTCODESIZE : Opcode = Opcode {code: 0x3b, stack_input: 1, stack_output: 1, n: 0};
pub const EXTCODECOPY : Opcode = Opcode {code: 0x3c, stack_input: 4, stack_output: 0, n: 0};
pub const RETURNDATASIZE : Opcode = Opcode {code: 0x3d, stack_input: 0, stack_output: 1, n: 0};
pub const RETURNDATACOPY : Opcode = Opcode {code: 0x3e, stack_input: 3, stack_output: 0, n: 0};
pub const EXTCODEHASH : Opcode = Opcode {code: 0x3f, stack_input: 1, stack_output: 1, n: 0};

// 0x40 range - block operations.
pub const BLOCKHASH : Opcode = Opcode {code: 0x40, stack_input: 1, stack_output: 1, n: 0};
pub const COINBASE : Opcode = Opcode {code: 0x41, stack_input: 0, stack_output: 1, n: 0};
pub const TIMESTAMP : Opcode = Opcode {code: 0x42, stack_input: 0, stack_output: 1, n: 0};
pub const NUMBER : Opcode = Opcode {code: 0x43, stack_input: 0, stack_output: 1, n: 0};
pub const DIFFICULTY : Opcode = Opcode {code: 0x44, stack_input: 0, stack_output: 1, n: 0};
pub const GASLIMIT : Opcode = Opcode {code: 0x45, stack_input: 0, stack_output: 1, n: 0};
pub const CHAINID : Opcode = Opcode {code: 0x46, stack_input: 0, stack_output: 1, n: 0};
pub const SELFBALANCE : Opcode = Opcode {code: 0x47, stack_input: 0, stack_output: 1, n: 0};
pub const BASEFEE : Opcode = Opcode {code: 0x48, stack_input: 0, stack_output: 1, n: 0};


// 0x50 range - 'storage' and execution.
pub const POP : Opcode = Opcode {code: 0x50, stack_input: 1, stack_output: 0, n: 0};
pub const MLOAD : Opcode = Opcode {code: 0x51, stack_input: 1, stack_output: 1, n: 0};
pub const MSTORE : Opcode = Opcode {code: 0x52, stack_input: 2, stack_output: 0, n: 0};
pub const MSTORE8 : Opcode = Opcode {code: 0x53, stack_input: 2, stack_output: 0, n: 0};
pub const SLOAD : Opcode = Opcode {code: 0x54, stack_input: 1, stack_output: 1, n: 0};
pub const SSTORE : Opcode = Opcode {code: 0x55, stack_input: 2, stack_output: 0, n: 0};
pub const JUMP : Opcode = Opcode {code: 0x56, stack_input: 1, stack_output: 0, n: 0};
pub const JUMPI : Opcode = Opcode {code: 0x57, stack_input: 2, stack_output: 0, n: 0};
pub const PC : Opcode = Opcode {code: 0x58, stack_input: 0, stack_output: 1, n: 0};
pub const MSIZE : Opcode = Opcode {code: 0x59, stack_input: 0, stack_output: 1, n: 0};
pub const GAS : Opcode = Opcode {code: 0x5a, stack_input: 0, stack_output: 1, n: 0};
pub const JUMPDEST : Opcode = Opcode {code: 0x5b, stack_input: 0, stack_output: 0, n: 0};

// 0x60 range - pushes.
pub const PUSH1 : Opcode = Opcode {code: 0x60, stack_input: 0, stack_output: 1, n: 1};
pub const PUSH2 : Opcode = Opcode {code: 0x61, stack_input: 0, stack_output: 1, n: 2};
pub const PUSH3 : Opcode = Opcode {code: 0x62, stack_input: 0, stack_output: 1, n: 3};
pub const PUSH4 : Opcode = Opcode {code: 0x63, stack_input: 0, stack_output: 1, n: 4};
pub const PUSH5 : Opcode = Opcode {code: 0x64, stack_input: 0, stack_output: 1, n: 5};
pub const PUSH6 : Opcode = Opcode {code: 0x65, stack_input: 0, stack_output: 1, n: 6};
pub const PUSH7 : Opcode = Opcode {code: 0x66, stack_input: 0, stack_output: 1, n: 7};
pub const PUSH8 : Opcode = Opcode {code: 0x67, stack_input: 0, stack_output: 1, n: 8};
pub const PUSH9 : Opcode = Opcode {code: 0x68, stack_input: 0, stack_output: 1, n: 9};
pub const PUSH10 : Opcode = Opcode {code: 0x69, stack_input: 0, stack_output: 1, n: 10};
pub const PUSH11 : Opcode = Opcode {code: 0x6a, stack_input: 0, stack_output: 1, n: 11};
pub const PUSH12 : Opcode = Opcode {code: 0x6b, stack_input: 0, stack_output: 1, n: 12};
pub const PUSH13 : Opcode = Opcode {code: 0x6c, stack_input: 0, stack_output: 1, n: 13};
pub const PUSH14 : Opcode = Opcode {code: 0x6d, stack_input: 0, stack_output: 1, n: 14};
pub const PUSH15 : Opcode = Opcode {code: 0x6e, stack_input: 0, stack_output: 1, n: 15};
pub const PUSH16 : Opcode = Opcode {code: 0x6f, stack_input: 0, stack_output: 1, n: 16};
pub const PUSH17 : Opcode = Opcode {code: 0x70, stack_input: 0, stack_output: 1, n: 17};
pub const PUSH18 : Opcode = Opcode {code: 0x71, stack_input: 0, stack_output: 1, n: 18};
pub const PUSH19 : Opcode = Opcode {code: 0x72, stack_input: 0, stack_output: 1, n: 19};
pub const PUSH20 : Opcode = Opcode {code: 0x73, stack_input: 0, stack_output: 1, n: 20};
pub const PUSH21 : Opcode = Opcode {code: 0x74, stack_input: 0, stack_output: 1, n: 21};
pub const PUSH22 : Opcode = Opcode {code: 0x75, stack_input: 0, stack_output: 1, n: 22};
pub const PUSH23 : Opcode = Opcode {code: 0x76, stack_input: 0, stack_output: 1, n: 23};
pub const PUSH24 : Opcode = Opcode {code: 0x77, stack_input: 0, stack_output: 1, n: 24};
pub const PUSH25 : Opcode = Opcode {code: 0x78, stack_input: 0, stack_output: 1, n: 25};
pub const PUSH26 : Opcode = Opcode {code: 0x79, stack_input: 0, stack_output: 1, n: 26};
pub const PUSH27 : Opcode = Opcode {code: 0x7a, stack_input: 0, stack_output: 1, n: 27};
pub const PUSH28 : Opcode = Opcode {code: 0x7b, stack_input: 0, stack_output: 1, n: 28};
pub const PUSH29 : Opcode = Opcode {code: 0x7c, stack_input: 0, stack_output: 1, n: 29};
pub const PUSH30 : Opcode = Opcode {code: 0x7d, stack_input: 0, stack_output: 1, n: 30};
pub const PUSH31 : Opcode = Opcode {code: 0x7e, stack_input: 0, stack_output: 1, n: 31};
pub const PUSH32 : Opcode = Opcode {code: 0x7f, stack_input: 0, stack_output: 1, n: 32};

// 0x80 range - dups.
pub const DUP1 : Opcode = Opcode {code: 0x80, stack_input: 1, stack_output: 0, n: 1};
pub const DUP2 : Opcode = Opcode {code: 0x81, stack_input: 2, stack_output: 0, n: 2};
pub const DUP3 : Opcode = Opcode {code: 0x82, stack_input: 3, stack_output: 0, n: 3};
pub const DUP4 : Opcode = Opcode {code: 0x83, stack_input: 4, stack_output: 0, n: 4};
pub const DUP5 : Opcode = Opcode {code: 0x84, stack_input: 5, stack_output: 0, n: 5};
pub const DUP6 : Opcode = Opcode {code: 0x85, stack_input: 6, stack_output: 0, n: 6};
pub const DUP7 : Opcode = Opcode {code: 0x86, stack_input: 7, stack_output: 0, n: 7};
pub const DUP8 : Opcode = Opcode {code: 0x87, stack_input: 8, stack_output: 0, n: 8};
pub const DUP9 : Opcode = Opcode {code: 0x88, stack_input: 9, stack_output: 0, n: 9};
pub const DUP10: Opcode = Opcode {code: 0x89, stack_input: 10, stack_output: 0, n: 10};
pub const DUP11: Opcode = Opcode {code: 0x8a, stack_input: 11, stack_output: 0, n: 11};
pub const DUP12: Opcode = Opcode {code: 0x8b, stack_input: 12, stack_output: 0, n: 12};
pub const DUP13: Opcode = Opcode {code: 0x8c, stack_input: 13, stack_output: 0, n: 13};
pub const DUP14: Opcode = Opcode {code: 0x8d, stack_input: 14, stack_output: 0, n: 14};
pub const DUP15: Opcode = Opcode {code: 0x8e, stack_input: 15, stack_output: 0, n: 15};
pub const DUP16: Opcode = Opcode {code: 0x8f, stack_input: 16, stack_output: 0, n: 16};

// 0x90 range - swaps.
pub const SWAP1 : Opcode = Opcode {code: 0x90, stack_input: 1, stack_output: 0, n: 1};
pub const SWAP2 : Opcode = Opcode {code: 0x91, stack_input: 2, stack_output: 0, n: 2};
pub const SWAP3 : Opcode = Opcode {code: 0x92, stack_input: 3, stack_output: 0, n: 3};
pub const SWAP4 : Opcode = Opcode {code: 0x93, stack_input: 4, stack_output: 0, n: 4};
pub const SWAP5 : Opcode = Opcode {code: 0x94, stack_input: 5, stack_output: 0, n: 5};
pub const SWAP6 : Opcode = Opcode {code: 0x95, stack_input: 6, stack_output: 0, n: 6};
pub const SWAP7 : Opcode = Opcode {code: 0x96, stack_input: 7, stack_output: 0, n: 7};
pub const SWAP8 : Opcode = Opcode {code: 0x97, stack_input: 8, stack_output: 0, n: 8};
pub const SWAP9 : Opcode = Opcode {code: 0x98, stack_input: 9, stack_output: 0, n: 9};
pub const SWAP10 : Opcode = Opcode {code: 0x99, stack_input: 10, stack_output: 0, n: 10};
pub const SWAP11 : Opcode = Opcode {code: 0x9a, stack_input: 11, stack_output: 0, n: 11};
pub const SWAP12 : Opcode = Opcode {code: 0x9b, stack_input: 12, stack_output: 0, n: 12};
pub const SWAP13 : Opcode = Opcode {code: 0x9c, stack_input: 13, stack_output: 0, n: 13};
pub const SWAP14 : Opcode = Opcode {code: 0x9d, stack_input: 14, stack_output: 0, n: 14};
pub const SWAP15 : Opcode = Opcode {code: 0x9e, stack_input: 15, stack_output: 0, n: 15};
pub const SWAP16 : Opcode = Opcode {code: 0x9f, stack_input: 16, stack_output: 0, n: 16};

// 0xa0 range - logging ops.
pub const LOG0 : Opcode = Opcode {code: 0xa0, stack_input: 2, stack_output: 0, n: 0};
pub const LOG1 : Opcode = Opcode {code: 0xa1, stack_input: 3, stack_output: 0, n: 1};
pub const LOG2 : Opcode = Opcode {code: 0xa2, stack_input: 4, stack_output: 0, n: 2};
pub const LOG3 : Opcode = Opcode {code: 0xa3, stack_input: 5, stack_output: 0, n: 3};
pub const LOG4 : Opcode = Opcode {code: 0xa4, stack_input: 6, stack_output: 0, n: 4};

// 0xf0 range - closures.
pub const CREATE : Opcode = Opcode {code: 0xf0, stack_input: 3, stack_output: 1, n: 0};
pub const CALL : Opcode = Opcode {code: 0xf1, stack_input: 7, stack_output: 1, n: 0};
pub const CALLCODE : Opcode = Opcode {code: 0xf2, stack_input: 7, stack_output: 1, n: 0};
pub const RETURN : Opcode = Opcode {code: 0xf3, stack_input: 2, stack_output: 0, n: 0};
pub const DELEGATECALL : Opcode = Opcode {code: 0xf4, stack_input: 6, stack_output: 0, n: 0};
pub const CREATE2 : Opcode = Opcode {code: 0xf5, stack_input: 4, stack_output: 1, n: 0};

pub const STATICCALL : Opcode = Opcode {code: 0xfa, stack_input: 6, stack_output: 1, n: 0};

pub const REVERT : Opcode = Opcode {code: 0xfd, stack_input: 2, stack_output: 0, n: 0};

pub const SELFDESTRUCT : Opcode = Opcode {code: 0xff, stack_input: 1, stack_output: 0, n: 0};


impl Opcode {
    pub const fn from_code(code: u8) -> Self {
        match code {
            0x00 => STOP,
            0x01 => ADD,
            0x02 => MUL,
            0x03 => SUB,
            0x04 => DIV,
            0x05 => SDIV,
            0x06 => MOD,
            0x07 => SMOD,
            0x08 => ADDMOD,
            0x09 => MULMOD,
            0x0a => EXP,
            0x0b => SIGNEXTEND,
            
            0x10 => LT,
            0x11 => GT,
            0x12 => SLT,
            0x13 => SGT,
            0x14 => EQ,
            0x15 => ISZERO,
            0x16 => AND,
            0x17 => OR,
            0x18 => XOR,
            0x19 => NOT,
            0x1a => BYTE,
            0x1b => SHL,
            0x1c => SHR,
            0x1d => SAR,
            
            0x20 => SHA3,

            0x30 => ADDRESS,
            0x31 => BALANCE,
            0x32 => ORIGIN,
            0x33 => CALLER,
            0x34 => CALLVALUE,
            0x35 => CALLDATALOAD,
            0x36 => CALLDATASIZE,
            0x37 => CALLDATACOPY,
            0x38 => CODESIZE,
            0x39 => CODECOPY,
            0x3a => GASPRICE,
            0x3b => EXTCODESIZE,
            0x3c => EXTCODECOPY,
            0x3d => RETURNDATASIZE,
            0x3e => RETURNDATACOPY,
            0x3f => EXTCODEHASH,

            0x40 => BLOCKHASH,
            0x41 => COINBASE,
            0x42 => TIMESTAMP,
            0x43 => NUMBER,
            0x44 => DIFFICULTY,
            0x45 => GASLIMIT,
            0x46 => CHAINID,
            0x47 => SELFBALANCE,
            0x48 => BASEFEE,

            0x50 => POP,
            0x51 => MLOAD,
            0x52 => MSTORE,
            0x53 => MSTORE8,
            0x54 => SLOAD,
            0x55 => SSTORE,
            0x56 => JUMP,
            0x57 => JUMPI,
            0x58 => PC,
            0x59 => MSIZE,
            0x5a => GAS,
            0x5b => JUMPDEST,

            0x60 => PUSH1,
            0x61 => PUSH2,
            0x62 => PUSH3,
            0x63 => PUSH4,
            0x64 => PUSH5,
            0x65 => PUSH6,
            0x66 => PUSH7,
            0x67 => PUSH8,
            0x68 => PUSH9,
            0x69 => PUSH10,
            0x6a => PUSH11,
            0x6b => PUSH12,
            0x6c => PUSH13,
            0x6d => PUSH14,
            0x6e => PUSH15,
            0x6f => PUSH16,
            0x70 => PUSH17,
            0x71 => PUSH18,
            0x72 => PUSH19,
            0x73 => PUSH20,
            0x74 => PUSH21,
            0x75 => PUSH22,
            0x76 => PUSH23,
            0x77 => PUSH24,
            0x78 => PUSH25,
            0x79 => PUSH26,
            0x7a => PUSH27,
            0x7b => PUSH28,
            0x7c => PUSH29,
            0x7d => PUSH30,
            0x7e => PUSH31,
            0x7f => PUSH32,

            0x80 => DUP1,
            0x81 => DUP2,
            0x82 => DUP3,
            0x83 => DUP4,
            0x84 => DUP5,
            0x85 => DUP6,
            0x86 => DUP7,
            0x87 => DUP8,
            0x88 => DUP9,
            0x89 => DUP10,
            0x8a => DUP11,
            0x8b => DUP12,
            0x8c => DUP13,
            0x8d => DUP14,
            0x8e => DUP15,
            0x8f => DUP16,

            0x90 => SWAP1,
            0x91 => SWAP2,
            0x92 => SWAP3,
            0x93 => SWAP4,
            0x94 => SWAP5,
            0x95 => SWAP6,
            0x96 => SWAP7,
            0x97 => SWAP8,
            0x98 => SWAP9,
            0x99 => SWAP10,
            0x9a => SWAP11,
            0x9b => SWAP12,
            0x9c => SWAP13,
            0x9d => SWAP14,
            0x9e => SWAP15,
            0x9f => SWAP16,

            0xa0 => LOG0,
            0xa1 => LOG1,
            0xa2 => LOG2,
            0xa3 => LOG3,
            0xa4 => LOG4,
            
            0xf0 => CREATE,
            0xf1 => CALL,
            0xf2 => CALLCODE,
            0xf3 => RETURN,
            0xf4 => DELEGATECALL,
            0xf5 => CREATE2,

            0xfa => STATICCALL,

            0xfd => REVERT,

            0xff => SELFDESTRUCT,

            _ => Opcode {code, stack_input: 0, stack_output: 0, n: 0},
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "STOP" => STOP,
            "ADD" => ADD,
            "MUL" => MUL,
            "SUB" => SUB,
            "DIV" => DIV,
            "SDIV" => SDIV,
            "MOD" => MOD,
            "SMOD" => SMOD,
            "ADDMOD" => ADDMOD,
            "MULMOD" => MULMOD,
            "EXP" => EXP,
            "SIGNEXTEND" => SIGNEXTEND,
            
            "LT" => LT,
            "GT" => GT,
            "SLT" => SLT,
            "SGT" => SGT,
            "EQ" => EQ,
            "ISZERO" => ISZERO,
            "AND" => AND,
            "OR" => OR,
            "XOR" => XOR,
            "NOT" => NOT,
            "BYTE" => BYTE,
            "SHL" => SHL,
            "SHR" => SHR,
            "SAR" => SAR,
            
            "SHA3" => SHA3,

            "ADDRESS" => ADDRESS,
            "BALANCE" => BALANCE,
            "ORIGIN" => ORIGIN,
            "CALLER" => CALLER,
            "CALLVALUE" => CALLVALUE,
            "CALLDATALOAD" => CALLDATALOAD,
            "CALLDATASIZE" => CALLDATASIZE,
            "CALLDATACOPY" => CALLDATACOPY,
            "CODESIZE" => CODESIZE,
            "CODECOPY" => CODECOPY,
            "GASPRICE" => GASPRICE,
            "EXTCODESIZE" => EXTCODESIZE,
            "EXTCODECOPY" => EXTCODECOPY,
            "RETURNDATASIZE" => RETURNDATASIZE,
            "RETURNDATACOPY" => RETURNDATACOPY,
            "EXTCODEHASH" => EXTCODEHASH,

            "BLOCKHASH" => BLOCKHASH,
            "COINBASE" => COINBASE,
            "TIMESTAMP" => TIMESTAMP,
            "NUMBER" => NUMBER,
            "DIFFICULTY" => DIFFICULTY,
            "GASLIMIT" => GASLIMIT,
            "CHAINID" => CHAINID,
            "SELFBALANCE" => SELFBALANCE,
            "BASEFEE" => BASEFEE,

            "POP" => POP,
            "MLOAD" => MLOAD,
            "MSTORE" => MSTORE,
            "MSTORE8" => MSTORE8,
            "SLOAD" => SLOAD,
            "SSTORE" => SSTORE,
            "JUMP" => JUMP,
            "JUMPI" => JUMPI,
            "PC" => PC,
            "MSIZE" => MSIZE,
            "GAS" => GAS,
            "JUMPDEST" => JUMPDEST,

            "PUSH1" => PUSH1,
            "PUSH2" => PUSH2,
            "PUSH3" => PUSH3,
            "PUSH4" => PUSH4,
            "PUSH5" => PUSH5,
            "PUSH6" => PUSH6,
            "PUSH7" => PUSH7,
            "PUSH8" => PUSH8,
            "PUSH9" => PUSH9,
            "PUSH10" => PUSH10,
            "PUSH11" => PUSH11,
            "PUSH12" => PUSH12,
            "PUSH13" => PUSH13,
            "PUSH14" => PUSH14,
            "PUSH15" => PUSH15,
            "PUSH16" => PUSH16,
            "PUSH17" => PUSH17,
            "PUSH18" => PUSH18,
            "PUSH19" => PUSH19,
            "PUSH20" => PUSH20,
            "PUSH21" => PUSH21,
            "PUSH22" => PUSH22,
            "PUSH23" => PUSH23,
            "PUSH24" => PUSH24,
            "PUSH25" => PUSH25,
            "PUSH26" => PUSH26,
            "PUSH27" => PUSH27,
            "PUSH28" => PUSH28,
            "PUSH29" => PUSH29,
            "PUSH30" => PUSH30,
            "PUSH31" => PUSH31,
            "PUSH32" => PUSH32,

            "DUP1" => DUP1,
            "DUP2" => DUP2,
            "DUP3" => DUP3,
            "DUP4" => DUP4,
            "DUP5" => DUP5,
            "DUP6" => DUP6,
            "DUP7" => DUP7,
            "DUP8" => DUP8,
            "DUP9" => DUP9,
            "DUP10" => DUP10,
            "DUP11" => DUP11,
            "DUP12" => DUP12,
            "DUP13" => DUP13,
            "DUP14" => DUP14,
            "DUP15" => DUP15,
            "DUP16" => DUP16,

            "SWAP1" => SWAP1,
            "SWAP2" => SWAP2,
            "SWAP3" => SWAP3,
            "SWAP4" => SWAP4,
            "SWAP5" => SWAP5,
            "SWAP6" => SWAP6,
            "SWAP7" => SWAP7,
            "SWAP8" => SWAP8,
            "SWAP9" => SWAP9,
            "SWAP10" => SWAP10,
            "SWAP11" => SWAP11,
            "SWAP12" => SWAP12,
            "SWAP13" => SWAP13,
            "SWAP14" => SWAP14,
            "SWAP15" => SWAP15,
            "SWAP16" => SWAP16,

            "LOG0" => LOG0,
            "LOG1" => LOG1,
            "LOG2" => LOG2,
            "LOG3" => LOG3,
            "LOG4" => LOG4,
            
            "CREATE" => CREATE,
            "CALL" => CALL,
            "CALLCODE" => CALLCODE,
            "RETURN" => RETURN,
            "DELEGATECALL" => DELEGATECALL,
            "CREATE2" => CREATE2,

            "STATICCALL" => STATICCALL,

            "REVERT" => REVERT,

            "SELFDESTRUCT" => SELFDESTRUCT,
            _ => panic!("Invalid opcode name"),
        }
    }

    pub const fn get_name(&self) -> &'static str{
        match self {
            &STOP => "STOP",
            &ADD => "ADD",
            &MUL => "MUL",
            &SUB => "SUB",
            &DIV => "DIV",
            &SDIV => "SDIV",
            &MOD => "MOD",
            &SMOD => "SMOD",
            &ADDMOD => "ADDMOD",
            &MULMOD => "MULMOD",
            &EXP => "EXP",
            &SIGNEXTEND => "SIGNEXTEND",
            
            &LT => "LT",
            &GT => "GT",
            &SLT => "SLT",
            &SGT => "SGT",
            &EQ => "EQ",
            &ISZERO => "ISZERO",
            &AND => "AND",
            &OR => "OR",
            &XOR => "XOR",
            &NOT => "NOT",
            &BYTE => "BYTE",
            &SHL => "SHL",
            &SHR => "SHR",
            &SAR => "SAR",
            
            &SHA3 => "SHA3",

            &ADDRESS => "ADDRESS",
            &BALANCE => "BALANCE",
            &ORIGIN => "ORIGIN",
            &CALLER => "CALLER",
            &CALLVALUE => "CALLVALUE",
            &CALLDATALOAD => "CALLDATALOAD",
            &CALLDATASIZE => "CALLDATASIZE",
            &CALLDATACOPY => "CALLDATACOPY",
            &CODESIZE => "CODESIZE",
            &CODECOPY => "CODECOPY",
            &GASPRICE => "GASPRICE",
            &EXTCODESIZE => "EXTCODESIZE",
            &EXTCODECOPY => "EXTCODECOPY",
            &RETURNDATASIZE => "RETURNDATASIZE",
            &RETURNDATACOPY => "RETURNDATACOPY",
            &EXTCODEHASH => "EXTCODEHASH",

            &BLOCKHASH => "BLOCKHASH",
            &COINBASE => "COINBASE",
            &TIMESTAMP => "TIMESTAMP",
            &NUMBER => "NUMBER",
            &DIFFICULTY => "DIFFICULTY",
            &GASLIMIT => "GASLIMIT",
            &CHAINID => "CHAINID",
            &SELFBALANCE => "SELFBALANCE",
            &BASEFEE => "BASEFEE",

            &POP => "POP",
            &MLOAD => "MLOAD",
            &MSTORE => "MSTORE",
            &MSTORE8 => "MSTORE8",
            &SLOAD => "SLOAD",
            &SSTORE => "SSTORE",
            &JUMP => "JUMP",
            &JUMPI => "JUMPI",
            &PC => "PC",
            &MSIZE => "MSIZE",
            &GAS => "GAS",
            &JUMPDEST => "JUMPDEST",

            &PUSH1 => "PUSH1",
            &PUSH2 => "PUSH2",
            &PUSH3 => "PUSH3",
            &PUSH4 => "PUSH4",
            &PUSH5 => "PUSH5",
            &PUSH6 => "PUSH6",
            &PUSH7 => "PUSH7",
            &PUSH8 => "PUSH8",
            &PUSH9 => "PUSH9",
            &PUSH10 => "PUSH10",
            &PUSH11 => "PUSH11",
            &PUSH12 => "PUSH12",
            &PUSH13 => "PUSH13",
            &PUSH14 => "PUSH14",
            &PUSH15 => "PUSH15",
            &PUSH16 => "PUSH16",
            &PUSH17 => "PUSH17",
            &PUSH18 => "PUSH18",
            &PUSH19 => "PUSH19",
            &PUSH20 => "PUSH20",
            &PUSH21 => "PUSH21",
            &PUSH22 => "PUSH22",
            &PUSH23 => "PUSH23",
            &PUSH24 => "PUSH24",
            &PUSH25 => "PUSH25",
            &PUSH26 => "PUSH26",
            &PUSH27 => "PUSH27",
            &PUSH28 => "PUSH28",
            &PUSH29 => "PUSH29",
            &PUSH30 => "PUSH30",
            &PUSH31 => "PUSH31",
            &PUSH32 => "PUSH32",

            &DUP1 => "DUP1",
            &DUP2 => "DUP2",
            &DUP3 => "DUP3",
            &DUP4 => "DUP4",
            &DUP5 => "DUP5",
            &DUP6 => "DUP6",
            &DUP7 => "DUP7",
            &DUP8 => "DUP8",
            &DUP9 => "DUP9",
            &DUP10 => "DUP10",
            &DUP11 => "DUP11",
            &DUP12 => "DUP12",
            &DUP13 => "DUP13",
            &DUP14 => "DUP14",
            &DUP15 => "DUP15",
            &DUP16 => "DUP16",

            &SWAP1 => "SWAP1",
            &SWAP2 => "SWAP2",
            &SWAP3 => "SWAP3",
            &SWAP4 => "SWAP4",
            &SWAP5 => "SWAP5",
            &SWAP6 => "SWAP6",
            &SWAP7 => "SWAP7",
            &SWAP8 => "SWAP8",
            &SWAP9 => "SWAP9",
            &SWAP10 => "SWAP10",
            &SWAP11 => "SWAP11",
            &SWAP12 => "SWAP12",
            &SWAP13 => "SWAP13",
            &SWAP14 => "SWAP14",
            &SWAP15 => "SWAP15",
            &SWAP16 => "SWAP16",

            &LOG0 => "LOG0",
            &LOG1 => "LOG1",
            &LOG2 => "LOG2",
            &LOG3 => "LOG3",
            &LOG4 => "LOG4",
            
            &CREATE => "CREATE",
            &CALL => "CALL",
            &CALLCODE => "CALLCODE",
            &RETURN => "RETURN",
            &DELEGATECALL => "DELEGATECALL",
            &CREATE2 => "CREATE2",

            &STATICCALL => "STATICCALL",

            &REVERT => "REVERT",

            &SELFDESTRUCT => "SELFDESTRUCT",
            _ => "INVALID",
        }
    }

    pub const fn is_push(&self) -> bool {
        match self {
            &PUSH1 | &PUSH2 | &PUSH3 | &PUSH4 | &PUSH5 | &PUSH6 | &PUSH7 | &PUSH8 | &PUSH9
            | &PUSH10 | &PUSH11 | &PUSH12 | &PUSH13 | &PUSH14 | &PUSH15 | &PUSH16 | &PUSH17
            | &PUSH18 | &PUSH19 | &PUSH20 | &PUSH21 | &PUSH22 | &PUSH23 | &PUSH24 | &PUSH25
            | &PUSH26 | &PUSH27 | &PUSH28 | &PUSH29 | &PUSH30 | &PUSH31 | &PUSH32 => true,
            _ => false,
        }
    }

    pub const fn is_dup(&self) -> bool {
        match self {
            &DUP1 | &DUP2 | &DUP3 | &DUP4 | &DUP5 | &DUP6 | &DUP7 | &DUP8 | &DUP9 | &DUP10
            | &DUP11 | &DUP12 | &DUP13 | &DUP14 | &DUP15 | &DUP16 => true,
            _ => false,
        }
    }

    pub const fn is_swap(&self) -> bool {
        match self {
            &SWAP1 | &SWAP2 | &SWAP3 | &SWAP4 | &SWAP5 | &SWAP6 | &SWAP7 | &SWAP8 | &SWAP9
            | &SWAP10 | &SWAP11 | &SWAP12 | &SWAP13 | &SWAP14 | &SWAP15 | &SWAP16 => true,
            _ => false,
        }
    }

    pub const fn is_invalid(&self) -> bool {
        return !self.is_push() && !self.is_dup() && !self.is_swap() &&
        match self {
            _ => true,
        }
    }
}
