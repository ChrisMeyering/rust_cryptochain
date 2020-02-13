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
    match hex::decode(sub_hash) {
        Ok(binary_bytes) => {
            let mut leading_zeros = 0;
            for byte in binary_bytes.iter() {
                leading_zeros += byte.leading_zeros();
                if byte != &(0 as u8) {
                    break;
                }
            }
            return difficulty <= leading_zeros as usize;
        }
        _ => return false,
    }
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

#[cfg(test)]
mod test_is_valid_hash {
    use super::is_valid_hash;

    #[test]
    fn on_valid_hash() {
        assert_eq!(is_valid_hash(&String::from("05".repeat(32)), 5), true);
    }

    #[test]
    fn on_wrong_length_hash() {
        assert_eq!(is_valid_hash(&String::from("05".repeat(30)), 5), false);
    }

    #[test]
    fn on_hash_with_insufficient_leading_zeros() {
        assert_eq!(is_valid_hash(&String::from("05".repeat(30)), 6), false);
    }
}

#[cfg(test)]
mod test_hash {
    use super::hash;
    use std::time::SystemTime;

    #[test]
    fn generates_sha256_hash() {
        let timestamp = SystemTime::UNIX_EPOCH;
        let last_hash = String::new();
        let data = String::new();
        let nonce: usize = 0;
        let difficulty: usize = 0;
        assert_eq!(
            hash(&timestamp, &last_hash, &data, nonce, difficulty),
            String::from("b837bbff1e8e2196046b10761d8c62ea7a0ce2316b882c4d54783bd8b96085a1")
        );
    }
}
