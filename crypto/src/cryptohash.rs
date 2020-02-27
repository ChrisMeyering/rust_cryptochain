// use crypto::{digest::Digest, sha2::Sha256};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;


pub struct Sha256Hash {
    pub hash: [u8; 32]
}

pub fn is_valid_hash(hash: &[u8], difficulty: usize) -> bool {
    if hash.len() != 32 {
        return false;
    }
    let mut bytes_to_check = 1;
    const BITS_PER_BYTE: usize = 8;
    for _ in (BITS_PER_BYTE..difficulty).step_by(BITS_PER_BYTE) {
        bytes_to_check += 1;
    }
    let sub_hash = &hash[..bytes_to_check];
    let mut leading_zeros = 0;
    for byte in sub_hash.iter() {
        leading_zeros += byte.leading_zeros();
        if *byte != 0 {
            break;
        }
    }
    return difficulty <= leading_zeros as usize;
}

pub fn hash(data_map: &BTreeMap<String, String>, hashed_data: &mut [u8]) {
    let mut data_str = String::from("|");
    for (key, value) in data_map {
        data_str.push_str(&format!(" {}:{} |", key, value));
    }
    println!("HASHING THE FOLLOWING STRING: {}", data_str);
    let mut sha = Sha256::new();
    sha.input(&data_str);
    hashed_data.copy_from_slice(sha.result().as_slice());
}


// TODO: remove code below once new hash() works
// pub fn is_valid_hash(hash: &[u8], difficulty: usize) -> bool {
//     if hash.len() != 64 {
//         return false;
//     }
//     let mut bits_required = 8;
//     while bits_required < difficulty {
//         bits_required += 8;
//     }
//     let hex_digits_required = bits_required / 4;
//     let sub_hash = &hash[..hex_digits_required];
//     let binary_bytes = hex::decode(sub_hash).unwrap();
//     let mut leading_zeros = 0;
//     for byte in binary_bytes.iter() {
//         leading_zeros += byte.leading_zeros();
//         if byte != &(0 as u8) {
//             break;
//         }
//     }
//     return difficulty <= leading_zeros as usize;
// }
// fn sha256sum(data: &String) -> &[u8; 32] {
//     let mut hasher = Sha256::default();
//     hasher.input(&data);
//     return hasher.result().as_slice()[..32];
// }
// pub fn hash(
//     timestamp: &SystemTime,
//     last_hash: &String,
//     data: &String,
//     nonce: usize,
//     difficulty: usize,
// ) -> [u8; 32] {
//     let data_str = String::from(
//         format!(
//             "{timestamp:?}{last_hash}{data}{nonce}{difficulty}",
//             timestamp = timestamp,
//             last_hash = last_hash,
//             data = data,
//             nonce = nonce,
//             difficulty = difficulty
//         )
//         .as_str(),
//     );
//     return sha256sum(&data_str);
//     // let mut hasher = Sha256::Default();
//     // hasher.input(&data_str);
//     // return hasher.result();
// }
