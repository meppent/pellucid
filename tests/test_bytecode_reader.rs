use hex;
use pellucid::bytecode_reader::bytecode::Bytecode;
use pellucid::bytecode_reader::opcode::Opcode;
use primitive_types::U256;
use rand;
mod utils;

#[test]
pub fn test_simple_contract_bytecode() {
    let contract: utils::Contract = utils::Contract::SIMPLE_CONTRACT;
    let bytecode: Bytecode = Bytecode::from(&contract.get_bytecode());
    dbg!(contract.get_opcodes().len());
    dbg!(bytecode.to_string().len());
    assert!(bytecode.to_string() == contract.get_opcodes());
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

#[test]
fn test_0x_support() {
    let mut random_vec: Vec<u8> = Vec::new();
    for _ in 0..200 {
        random_vec.push(rand::random::<u8>());
    }
    let bytecode1 = hex::encode(random_vec);
    let mut bytecode2 = "0x".to_owned();
    bytecode2.push_str(&bytecode1);
    assert_eq!(
        Bytecode::from(&bytecode1),
        Bytecode::from(&bytecode2)
    );
}

#[test]
fn test_last_push() {
    let bytecode = Bytecode::from("6001");
    let last_vopcode = bytecode.get_vopcode_at(bytecode.get_last_pc());
    assert!(
        last_vopcode.is_last &&
        last_vopcode.opcode == Opcode::from(0x60) &&
        last_vopcode.value == Some(U256::from(1))
    );

    let bytecode = Bytecode::from("60");
    let last_vopcode = bytecode.get_vopcode_at(bytecode.get_last_pc());
    assert!(
        last_vopcode.is_last &&
        last_vopcode.opcode == Opcode::from(0x60) &&
        last_vopcode.value == None
    );

    let bytecode = Bytecode::from("000065000000000002");
    let last_vopcode = bytecode.get_vopcode_at(bytecode.get_last_pc());
    assert!(
        last_vopcode.is_last &&
        last_vopcode.opcode == Opcode::from(0x65) &&
        last_vopcode.value == Some(U256::from(2))
    );

    let bytecode = Bytecode::from("0000650000000002");
    let last_vopcode = bytecode.get_vopcode_at(bytecode.get_last_pc());
    assert!(
        last_vopcode.is_last &&
        last_vopcode.opcode == Opcode::from(0x65) &&
        last_vopcode.value == None
    );
}