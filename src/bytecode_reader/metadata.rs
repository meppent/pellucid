#![allow(dead_code)]
use cbor::Decoder as CBOR_Decoder;
use hex;
use multibase::Base;
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub enum Metadata {
    SolcVersion(u8, u8, u8),
    IPFS(String),
    BZZR(u8, String),
    Unknown(String, Vec<u8>),
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: String = String::from("");
        match self {
            Metadata::SolcVersion(major, minor, patch) => {
                res.push_str(&format!("Solc {}.{}.{}", major, minor, patch))
            }
            Metadata::IPFS(ipfs_hash) => res.push_str(&format!("IPFS {}", ipfs_hash)),
            Metadata::BZZR(0, bzzr_hash) => res.push_str(&format!("BZZR0 {}", bzzr_hash)),
            Metadata::BZZR(_, bzzr_hash) => res.push_str(&format!("BZZR1 {}", bzzr_hash)),
            Metadata::Unknown(name, value) => {
                res.push_str(&format!("{}: {}", name, hex::encode(value)))
            }
        }
        f.write_str(&res)?;
        Ok(())
    }
}

pub fn get_metadata(source_code: &str) -> Vec<Metadata> {
    let decoded_source_code = hex::decode(source_code).unwrap();
    let metadata_bytes = get_metadata_bytes(&decoded_source_code);
    if metadata_bytes.len() > 0 {
        return decode_metadata(metadata_bytes);
    } else {
        return Vec::new();
    }
}

fn get_metadata_bytes(source_code: &[u8]) -> &[u8] {
    let length = source_code.len();
    let metadata_size = (source_code[length - 1] as usize) + (source_code[length - 2] as usize);
    let metadata_end = length - 2;
    if metadata_size > metadata_end {
        return &[];
    }
    let metadata_start = metadata_end - metadata_size;
    return &source_code[metadata_start..metadata_end];
}

fn decode_metadata(encoded_metadata: &[u8]) -> Vec<Metadata> {
    let mut decoder = CBOR_Decoder::from_bytes(encoded_metadata);
    let decoded = decoder.decode::<HashMap<String, Vec<u8>>>().next();
    match decoded {
        Some(Ok(decoded_metadata)) => {
            let mut metadata: Vec<Metadata> = Vec::new();
            for (key, value) in decoded_metadata.into_iter() {
                metadata.push(match key.as_str() {
                    "solc" => decode_solc_version(&value),
                    "ipfs" => decode_ipfs(&value),
                    "bzzr0" => decode_bzzr(0, &value),
                    "bzzr1" => decode_bzzr(1, &value),
                    _ => Metadata::Unknown(key, value),
                })
            }
            return metadata;
        },
        _ => {return Vec::new()}
    }
}

fn decode_ipfs(ipfs_bytes: &[u8]) -> Metadata {
    let decoded: String = multibase::encode(Base::Base58Btc, ipfs_bytes);
    return Metadata::IPFS(decoded[1..].to_string());
}

fn decode_solc_version(solc_bytes: &[u8]) -> Metadata {
    return Metadata::SolcVersion(solc_bytes[0], solc_bytes[1], solc_bytes[2]);
}

fn decode_bzzr(version: u8, bzzr_bytes: &[u8]) -> Metadata {
    return Metadata::BZZR(version, hex::encode(bzzr_bytes));
}
