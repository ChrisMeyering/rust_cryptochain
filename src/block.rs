use std::time::SystemTime;

const MINE_RATE: u128 = 1_000;

#[derive(Debug)]
pub struct Block {
    timestamp: SystemTime,
    last_hash: String,
    hash: String,
    data: String,
    nonce: usize,
    difficulty: usize,
}

impl Block {
    pub fn genesis() -> Block {
        Block {
            timestamp: SystemTime::now(),
            last_hash: String::from("none"),
            hash: String::from("genesis"),
            data: String::from("genesis block"),
            nonce: 0,
            difficulty: 15,
        }
    }

    pub fn mine_block(last_block: &Block, data: String) -> Block {
        let mut timestamp: SystemTime;
        let mut difficulty: usize;
        let mut nonce: usize = 0;
        let last_hash: String = last_block.hash.clone();

        let mut hash: String;
        loop {
            nonce += 1;
            timestamp = SystemTime::now();
            match last_block.timestamp.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_millis() < MINE_RATE {
                        difficulty = last_block.difficulty + 1;
                    } else {
                        difficulty = last_block.difficulty - 1;
                    }
                }
                Err(e) => {
                    panic!("Error: {:?}", e);
                }
            }
            hash = crypto::hash(&timestamp, &last_hash, &data, nonce, difficulty);
            if crypto::is_valid_hash(&hash, difficulty) {
                break;
            }
        }

        Block {
            timestamp,
            last_hash,
            hash,
            data,
            nonce,
            difficulty,
        }
    }

    pub fn is_valid_block(block: &Block, last_block_hash: &String) -> bool {
        if &block.last_hash != last_block_hash {
            return false;
        }
        let Block {
            timestamp,
            last_hash,
            data,
            nonce,
            difficulty,
            hash,
        } = block;
        if hash != &crypto::hash(timestamp, last_hash, data, *nonce, *difficulty) {
            return false;
        }

        if !crypto::is_valid_hash(hash, *difficulty) {
            return false;
        }

        return true;
    }
}

mod crypto {
    use std::time::SystemTime;
    
    pub fn is_valid_hash(hash: &String, difficulty: usize) -> bool {
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
        use crypto::{digest::Digest, sha2::Sha256};

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
}
