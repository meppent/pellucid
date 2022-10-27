/*
Credits: Some parts of the code bellow comes from https://github.com/rust-blockchain/evm
*/

use core::cmp::Ordering;
use core::convert::TryInto;
use primitive_types::{U256, U512};
use std::ops::{Div, Rem};

const SIGN_BIT_MASK: U256 = U256([
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x7fffffffffffffff,
]);
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Sign {
    Plus,
    Minus,
    Zero,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct I256(pub Sign, pub U256);

impl I256 {
    /// Zero value of I256.
    pub fn zero() -> I256 {
        I256(Sign::Zero, U256::zero())
    }
    /// Minimum value of I256.
    pub fn min_value() -> I256 {
        I256(Sign::Minus, (U256::MAX & SIGN_BIT_MASK) + U256::from(1u64))
    }
}

impl Ord for I256 {
    fn cmp(&self, other: &I256) -> Ordering {
        match (self.0, other.0) {
            (Sign::Zero, Sign::Zero) => Ordering::Equal,
            (Sign::Zero, Sign::Plus) => Ordering::Less,
            (Sign::Zero, Sign::Minus) => Ordering::Greater,
            (Sign::Minus, Sign::Zero) => Ordering::Less,
            (Sign::Minus, Sign::Plus) => Ordering::Less,
            (Sign::Minus, Sign::Minus) => self.1.cmp(&other.1).reverse(),
            (Sign::Plus, Sign::Minus) => Ordering::Greater,
            (Sign::Plus, Sign::Zero) => Ordering::Greater,
            (Sign::Plus, Sign::Plus) => self.1.cmp(&other.1),
        }
    }
}
impl From<I256> for U256 {
    fn from(value: I256) -> U256 {
        let sign = value.0;
        if sign == Sign::Zero {
            U256::zero()
        } else if sign == Sign::Plus {
            value.1
        } else {
            !value.1 + U256::from(1u64)
        }
    }
}
impl From<U256> for I256 {
    fn from(val: U256) -> I256 {
        if val == U256::zero() {
            I256::zero()
        } else if val & SIGN_BIT_MASK == val {
            I256(Sign::Plus, val)
        } else {
            I256(Sign::Minus, !val + U256::from(1u64))
        }
    }
}
impl PartialOrd for I256 {
    fn partial_cmp(&self, other: &I256) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Div for I256 {
    type Output = I256;

    fn div(self, other: I256) -> I256 {
        if other == I256::zero() {
            return I256::zero();
        }

        if self == I256::min_value() && other.1 == U256::from(1u64) {
            return I256::min_value();
        }

        let d = (self.1 / other.1) & SIGN_BIT_MASK;

        if d == U256::zero() {
            return I256::zero();
        }

        match (self.0, other.0) {
            (Sign::Zero, Sign::Plus)
            | (Sign::Plus, Sign::Zero)
            | (Sign::Zero, Sign::Zero)
            | (Sign::Plus, Sign::Plus)
            | (Sign::Minus, Sign::Minus) => I256(Sign::Plus, d),
            (Sign::Zero, Sign::Minus)
            | (Sign::Plus, Sign::Minus)
            | (Sign::Minus, Sign::Zero)
            | (Sign::Minus, Sign::Plus) => I256(Sign::Minus, d),
        }
    }
}

impl Rem for I256 {
    type Output = I256;

    fn rem(self, other: I256) -> I256 {
        let r = (self.1 % other.1) & SIGN_BIT_MASK;

        if r == U256::zero() {
            return I256::zero();
        }

        I256(self.0, r)
    }
}

pub fn byte(a: U256, b: U256) -> U256 {
    let mut ret = U256::zero();

    for i in 0..256 {
        if i < 8 && a < 32.into() {
            let o: usize = a.as_usize();
            let t = 255 - (7 - i + 8 * o);
            let bit_mask = U256::one() << t;
            let value = (b & bit_mask) >> t;
            ret = ret.overflowing_add(value << i).0;
        }
    }

    ret
}

pub fn eval_add(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    return input[0].overflowing_add(input[1]).0;
}

pub fn eval_mul(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    return input[0].overflowing_mul(input[1]).0;
}

pub fn eval_sub(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    return input[0].overflowing_sub(input[1]).0;
}

pub fn eval_div(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if input[1] == U256::zero() {
        return U256::zero();
    }
    return input[0] / input[1];
}

pub fn eval_sdiv(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    let op1: I256 = input[0].into();
    let op2: I256 = input[1].into();
    let ret: I256 = op1 / op2;
    ret.into()
}

pub fn eval_mod(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if input[1] == U256::zero() {
        U256::zero()
    } else {
        input[0].rem(input[1])
    }
}

pub fn eval_smod(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if input[1] == U256::zero() {
        U256::zero()
    } else {
        let op1: I256 = input[0].into();
        let op2: I256 = input[1].into();
        let ret: I256 = op1.rem(op2);
        ret.into()
    }
}
pub fn eval_addmod(input: Vec<U256>) -> U256 {
    assert!(input.len() == 3);
    let op1: U512 = input[0].into();
    let op2: U512 = input[1].into();
    let op3: U512 = input[2].into();

    if op3 == U512::zero() {
        U256::zero()
    } else {
        let v = (op1 + op2) % op3;

        v.try_into()
            .expect("op3 is less than U256::MAX, thus it never overflows; qed")
    }
}

pub fn eval_mulmod(input: Vec<U256>) -> U256 {
    assert!(input.len() == 3);
    let op1: U512 = input[0].into();
    let op2: U512 = input[1].into();
    let op3: U512 = input[2].into();

    if op3 == U512::zero() {
        U256::zero()
    } else {
        let v: U512 = (op1 * op2) % op3;

        v.try_into()
            .expect("op3 is less than U256::MAX, thus it never overflows; qed")
    }
}
pub fn eval_exp(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    let mut op1: U256 = input[0];
    let mut op2: U256 = input[1];
    let mut r: U256 = 1.into();

    while op2 != 0.into() {
        if op2 & 1.into() != 0.into() {
            r = r.overflowing_mul(op1).0;
        }
        op2 >>= 1;
        op1 = op1.overflowing_mul(op1).0;
    }

    r
}
pub fn eval_signextend(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if input[0] < U256::from(32) {
        // `low_u32` works since op1 < 32
        let bit_index = (8 * input[0].low_u32() + 7) as usize;
        let bit = input[1].bit(bit_index);
        let mask = (U256::one() << bit_index) - U256::one();
        if bit {
            input[1] | !mask
        } else {
            input[1] & mask
        }
    } else {
        input[1]
    }
}

pub fn eval_lt(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if input[0] < input[0] {
        return U256::from(1);
    } else {
        return U256::zero();
    }
}

pub fn eval_gt(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if input[0] > input[0] {
        return U256::from(1);
    } else {
        return U256::zero();
    }
}
pub fn eval_slt(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if I256::from(input[0]) < I256::from(input[1]) {
        return U256::from(1);
    } else {
        return U256::zero();
    }
}

pub fn eval_sgt(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if I256::from(input[0]) > I256::from(input[1]) {
        return U256::from(1);
    } else {
        return U256::zero();
    }
}

pub fn eval_eq(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if input[0] == input[1] {
        return U256::from(1);
    } else {
        return U256::zero();
    }
}

pub fn eval_iszero(input: Vec<U256>) -> U256 {
    assert!(input.len() == 1);
    if input[0] == U256::zero() {
        return U256::from(1);
    } else {
        return U256::zero();
    }
}

pub fn eval_and(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    return input[0] & input[1];
}

pub fn eval_or(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    return input[0] | input[1];
}
pub fn eval_xor(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    return input[0] ^ input[1];
}
pub fn eval_not(input: Vec<U256>) -> U256 {
    assert!(input.len() == 1);
    return !input[0];
}
pub fn eval_shl(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if input[1] == U256::zero() || input[0] >= U256::from(256) {
        U256::zero()
    } else {
        let shift: u64 = input[0].as_u64();
        input[1] << shift as usize
    }
}
pub fn eval_shr(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    if input[1] == U256::zero() || input[0] >= U256::from(256) {
        U256::zero()
    } else {
        let shift: u64 = input[0].as_u64();
        input[1] >> shift as usize
    }
}

pub fn eval_sar(input: Vec<U256>) -> U256 {
    assert!(input.len() == 2);
    let value = I256::from(input[1]);

    if value == I256::zero() || input[0] >= U256::from(256) {
        let I256(sign, _) = value;
        match sign {
            // value is 0 or >=1, pushing 0
            Sign::Plus | Sign::Zero => U256::zero(),
            // value is <0, pushing -1
            Sign::Minus => I256(Sign::Minus, U256::one()).into(),
        }
    } else {
        let shift: u64 = input[0].as_u64();

        match value.0 {
            Sign::Plus | Sign::Zero => value.1 >> shift as usize,
            Sign::Minus => {
                let shifted = ((value.1.overflowing_sub(U256::one()).0) >> shift as usize)
                    .overflowing_add(U256::one())
                    .0;
                I256(Sign::Minus, shifted).into()
            }
        }
    }
}
