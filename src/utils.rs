#![allow(dead_code)]
use itertools::sorted;
use primitive_types::U256;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

const HEX_CHARS: [char; 22] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'a', 'b', 'c',
    'd', 'e', 'f',
];

pub fn u256_to_hex(value: U256) -> String {
    let mut hex_value: String = String::from("0x");
    hex_value.push_str(&format!("{:x}", value));
    return hex_value;
}

pub fn remove_0x(s: &str) -> &str {
    if s.len() >= 2 && &s[0..2] == "0x" {
        return &s[2..];
    } else {
        return s;
    }
}
pub fn hex_to_u256(hex: &str) -> U256 {
    let hex: &str = remove_0x(hex);
    assert_hex(hex);
    return U256::from_str_radix(hex, 16).expect("Error converting hex to U256.");
}

pub fn hex_to_usize(hex: &str) -> usize {
    let hex: &str = remove_0x(hex);
    assert_hex(hex);
    return usize::from_str_radix(hex, 16).expect("Error converting hex to usize.");
}

pub fn assert_hex(hex: &str) {
    assert!(hex.chars().all(|c| HEX_CHARS.contains(&c)));
}

pub fn usize_to_hex(n: usize) -> String {
    let mut res: String = String::from("0x");
    res.push_str(&format!("{:x}", n));
    return res;
}

pub fn remove_values_where<T: PartialEq, F>(vec: &mut Vec<T>, condition: F)
where
    F: Fn(&T) -> bool,
{
    let mut indexes_to_remove: Vec<usize> = vec![];
    for (index, value) in vec.iter().enumerate() {
        if condition(value) {
            indexes_to_remove.push(index);
        }
    }
    reverse_sort(&mut indexes_to_remove);
    for index_to_remove in indexes_to_remove {
        vec.remove(index_to_remove);
    }
}

pub fn reverse_sort<T: PartialEq + Ord>(vec: &mut Vec<T>) {
    vec.sort_by(|a, b| b.cmp(a))
}

pub fn remove_value<T: PartialEq>(vec: &mut Vec<T>, value: &T) {
    let index: usize = vec.iter().position(|x| *x == *value).unwrap();
    vec.remove(index);
}
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s: DefaultHasher = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn vect_contains<T: PartialEq>(vec: &Vec<T>, value: &T) -> bool {
    return vec.iter().any(|i| i == value);
}

pub fn contains_value<A: PartialEq, B: PartialEq>(hash_map: &HashMap<A, B>, value: &B) -> bool {
    return hash_map.values().any(|val| val == value);
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn get_sorted_keys<A: Clone + Ord, B>(hash_map: &HashMap<A, B>) -> Vec<A> {
    return sorted(hash_map.keys().cloned().collect::<Vec<A>>()).collect();
}

pub fn max_mapped_value<'a, A: Clone + Copy, B: Ord + Clone>(
    set: &'a HashSet<A>,
    map: &'a dyn Fn(A) -> B,
) -> Option<B> {
    // input: `set` = {a, b, c...} and a function `map`
    // output: max(map(a), map(b), map(c)...)

    let res0 = set.clone();
    let res1 = res0.iter().map(|a| map(*a)).collect::<Vec<B>>();
    let res2 = res1.iter().max();
    if let Some(b) = res2 {
        return Some(b.clone());
    } else {
        return None;
    }
}

pub fn map_values_to_index<A: Copy + Hash + Eq>(vec: &Vec<A>) -> HashMap<A, usize> {
    return vec
        .iter()
        .enumerate()
        .map(|(index, value)| (*value, index))
        .collect::<HashMap<A, usize>>();
}

pub fn iter_int(beginning: usize, end: usize) -> impl Iterator<Item = usize> {
    /*
    if beginning < end  -> (beginning, beginning + 1, ..., end - 1)
    if end < beginning  -> (beginning, beginning - 1, ..., end + 1)
    if beginning == end -> ()
    */
    struct IntIterator {
        current: usize,
        end: usize,
    }
    impl Iterator for IntIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current == self.end {
                return None;
            }
            let old: usize = self.current;
            if self.current < self.end {
                self.current += 1;
            } else {
                self.current -= 1;
            };
            return Some(old);
        }
    }
    return IntIterator {
        current: beginning,
        end,
    };
}

pub fn get_max_key<A: PartialEq + PartialOrd + Copy, B>(hash_map: &HashMap<A, B>) -> Option<A> {
    let mut res: Option<A> = None;
    for key in hash_map.keys() {
        if res == None || key > &res.unwrap() {
            res = Some(*key);
        }
    }
    return res;
}
pub fn write_file(file: &str, data: &str) {
    fs::write(file, data).expect("Unable to write file.");
}

pub fn read_file(file: &str) -> String {
    return fs::read_to_string(file).expect("Unable to read file.");
}
