use hex;
use pellucid::bytecode_reader::bytecode;
use rand;
mod utils;

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
    let mut random_vec: Vec<u8> = Vec::new();
    for _ in 0..200 {
        random_vec.push(rand::random::<u8>());
    }
    let bytecode1 = hex::encode(random_vec);
    let mut bytecode2 = "0x".to_owned();
    bytecode2.push_str(&bytecode1);
    assert_eq!(
        bytecode::Bytecode::from(&bytecode1),
        bytecode::Bytecode::from(&bytecode2)
    );
}
