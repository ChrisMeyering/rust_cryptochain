use crypto::{digest::Digest, sha2::Sha256};
use std::time::SystemTime;

pub fn is_valid_hash(hash: &String, difficulty: usize) -> bool {
    if hash.len() != 64 {
        return false;
    }
    let mut bits_required = 8;
    while bits_required < difficulty {
        bits_required += 8;
    }
    let hex_digits_required = bits_required / 4;
    let sub_hash = &hash[..hex_digits_required];
    let binary_bytes = hex::decode(sub_hash).unwrap();
    let mut leading_zeros = 0;
    for byte in binary_bytes.iter() {
        leading_zeros += byte.leading_zeros();
        if byte != &(0 as u8) {
            break;
        }
    }
    return difficulty <= leading_zeros as usize;
}

pub fn hash(
    timestamp: &SystemTime,
    last_hash: &String,
    data: &String,
    nonce: usize,
    difficulty: usize,
) -> String {
    let data_str = String::from(
        format!(
            "{timestamp:?}{last_hash}{data}{nonce}{difficulty}",
            timestamp = timestamp,
            last_hash = last_hash,
            data = data,
            nonce = nonce,
            difficulty = difficulty
        )
        .as_str(),
    );
    let mut sha = Sha256::new();
    sha.input_str(&data_str);
    return sha.result_str();
}

