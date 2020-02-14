mod cryptohash;

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
            hash = cryptohash::hash(&timestamp, &last_hash, &data, nonce, difficulty);
            if cryptohash::is_valid_hash(&hash, difficulty) {
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
        if hash != &cryptohash::hash(timestamp, last_hash, data, *nonce, *difficulty) {
            return false;
        }

        if !cryptohash::is_valid_hash(hash, *difficulty) {
            return false;
        }

        return true;
    }
}
