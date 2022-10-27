use itertools::{sorted, Itertools};
use primitive_types::U256;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use walkdir::WalkDir;

pub fn u256_to_hex(value: U256) -> String {
    return format!("0x{:02x}", value);
}

pub fn remove_0x(s: &str) -> &str {
    if s.len() >= 2 && &s[0..2] == "0x" {
        return &s[2..];
    } else {
        return s;
    }
}

pub fn hex_to_usize(hex: &str) -> usize {
    let hex: &str = remove_0x(hex);
    return usize::from_str_radix(hex, 16).expect("Error converting hex to usize.");
}

pub fn usize_to_hex(n: usize) -> String {
    return format!("0x{:x}", n);
}

pub fn remove_value<T: PartialEq>(vec: &mut Vec<T>, value: &T) {
    let index: usize = vec.iter().position(|x| *x == *value).unwrap();
    vec.remove(index);
    assert!(!vec.contains(value));
}
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s: DefaultHasher = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn get_sorted_keys<A: Clone + Ord, B>(hash_map: &HashMap<A, B>) -> Vec<A> {
    return sorted(hash_map.keys().cloned().collect::<Vec<A>>()).collect();
}

pub fn max_mapped_value<'a, A: 'a, S: Iterator<Item = &'a A>, B: Ord + Clone, F: Fn(&A) -> B>(
    elements: &mut S,
    map: F,
) -> Option<B> {
    // input: `elements` = a, b, c... and a function `map`
    // output: max(map(a), map(b), map(c)...)

    let mut current_max = map(elements.next()?);
    for e in elements.into_iter() {
        let candidate = map(e);
        if candidate > current_max {
            current_max = candidate;
        }
    }
    return Some(current_max.clone());
}

pub fn map_values_to_index<A: Copy + Hash + Eq>(vec: &Vec<A>) -> HashMap<A, usize> {
    return vec
        .iter()
        .enumerate()
        .map(|(index, value)| (*value, index))
        .collect();
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

pub fn random_u8(min: u8, max: u8) -> u8 {
    return rand::thread_rng().gen_range(min..max);
}

pub fn hash_hashset<A: PartialEq + Eq + Hash>(set: &HashSet<A>) -> u64 {
    let hashes: Vec<u64> = sorted(set.iter().map(|e| calculate_hash(e))).collect();
    return calculate_hash(&hashes);
}

pub fn is_empty_iter<A, I: Iterator<Item = A>>(iterator: I) -> bool {
    return iterator.peekable().peek().is_none();
}

pub fn write_file(file: &str, data: &str) {
    fs::write(file, data).expect("Unable to write file.");
}

pub fn read_file(file: &str) -> String {
    return fs::read_to_string(file).expect("Unable to read file.");
}
pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn find_files<F: Fn(&str) -> bool>(dir: &str, condition_on_file: F) -> Vec<String> {
    return WalkDir::new(dir)
        .into_iter()
        .filter_map(|file| file.ok())
        .filter(|file| condition_on_file(file.file_name().to_str().unwrap()))
        .map(|file| file.path().display().to_string())
        .collect();
}

pub fn shift_text(text: &str) -> String {
    let mut res: String = String::new();
    for line in text.split("\n") {
        res.push_str("    ");
        res.push_str(line);
        res.push_str("\n");
    }
    return res;
}

pub fn concat_and_consume<A>(mut vector_0: Vec<A>, mut vector_1: Vec<A>) -> Vec<A> {
    // TODO find the proper built-in function
    vector_1.reverse();
    while let Some(e) = vector_1.pop() {
        vector_0.push(e);
    }
    return vector_0;
}

pub fn concat_to_str<V: ToString>(values: &Vec<V>, sep: &str) -> String {
    return values.iter().map(|v: &V| v.to_string()).join(sep);
}
pub fn rename_keys<K: Clone + Hash + Eq, V: Clone + Eq>(
    hash_map: &HashMap<K, V>,
    key_mapping: &HashMap<K, K>,
    delete_missing_keys: bool,
) -> HashMap<K, V> {
    let mut new_hash_map: HashMap<K, V> = HashMap::new();
    for (prev_key, value) in hash_map {
        if let Some(new_key) = key_mapping.get(prev_key) {
            new_hash_map.insert(new_key.clone(), value.clone());
        } else if !delete_missing_keys {
            new_hash_map.insert(prev_key.clone(), value.clone());
        }
    }
    return new_hash_map;
}

pub fn dedup_all<E: Hash + Eq + Clone>(vector: &mut Vec<E>) {
    let mut new_vector: Vec<E> = Vec::new();
    let mut seen_elements: HashSet<E> = HashSet::new();
    for element in vector.iter() {
        if !seen_elements.contains(element) {
            seen_elements.insert(element.clone());
            new_vector.push(element.clone());
        }
    }
    *vector = new_vector;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_all_bytecode_paths() -> impl Iterator<Item = String> {
        let mut paths: Vec<String> = Vec::new();
        for file in find_files("./contracts", |f| f == "bytecode.txt") {
            paths.push(file);
        }
        return paths.into_iter();
    }
}
