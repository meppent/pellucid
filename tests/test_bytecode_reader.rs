use hex;
use pellucid::bytecode_reader::bytecode;
use rand;
mod utils;

#[test]
pub fn test_simple_contract_bytecode() {
    let contract: utils::Contract = utils::Contract::SIMPLE_CONTRACT;
    let bytecode: bytecode::Bytecode = bytecode::Bytecode::from(&contract.get_bytecode());
    dbg!(contract.get_opcodes().len());
    dbg!(bytecode.to_string().len());
    assert!(bytecode.to_string() == contract.get_opcodes());
}

#[test]
#[should_panic]
fn test_odd_size_bytecode1() {
    bytecode::Bytecode::from("abc");
}

#[test]
#[should_panic]
fn test_odd_size_bytecode2() {
    bytecode::Bytecode::from("0xabc");
}

#[test]
#[should_panic]
fn test_invalid_character_in_bytecode1() {
    bytecode::Bytecode::from("abcg");
}

#[test]
#[should_panic]
fn test_invalid_character_in_bytecode2() {
    bytecode::Bytecode::from("0xabcg");
}

#[test]
fn test_0x_support() {
    let mut random_vec1: Vec<u8> = Vec::new();
    let mut random_vec2: Vec<u8> = Vec::new();
    for _ in 0..200 {
        random_vec1.push(rand::random::<u8>());
        random_vec2.push(rand::random::<u8>());
    }
    let bytecode1 = hex::encode(random_vec1);
    let mut bytecode2 = "0x".to_owned();
    bytecode2.push_str(&hex::encode(random_vec2));
    assert_eq!(
        bytecode::Bytecode::from(&bytecode1),
        bytecode::Bytecode::from(&bytecode2)
    );
}
