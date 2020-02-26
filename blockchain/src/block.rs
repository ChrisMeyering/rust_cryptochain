use crypt::cryptohash;

use crate::config::*;

use std::{cmp, time::SystemTime};



#[derive(Debug)]
pub struct Block {
    pub timestamp: SystemTime,
    pub last_hash: String,
    pub hash: String,
    pub data: String,
    pub nonce: usize,
    pub difficulty: usize,
}

impl Block {
    pub fn genesis() -> Block {
        Block {
            timestamp: SystemTime::UNIX_EPOCH,
            last_hash: String::from("none"),
            hash: String::from("genesis"),
            data: String::from("genesis block"),
            nonce: 0,
            difficulty: 8,
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
            difficulty = Block::adjust_difficulty(&last_block, &timestamp);
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

    pub fn adjust_difficulty(last_block: &Block, new_timestamp: &SystemTime) -> usize {
        let mut difficulty = last_block.difficulty;
        if last_block.difficulty < DIFFICULTY_MIN {
            return DIFFICULTY_MIN;
        } else if last_block.difficulty > DIFFICULTY_MAX {
            return DIFFICULTY_MAX;
        }
        match new_timestamp.duration_since(last_block.timestamp) {
            Ok(elapsed) => {
                if elapsed.as_millis() < MINE_RATE as u128 {
                    difficulty += 1;
                } else {
                    difficulty -= 1;
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                difficulty += 1;
            }
        }
        return cmp::max(cmp::min(DIFFICULTY_MAX, difficulty), DIFFICULTY_MIN);
    }

    pub fn is_valid_difficulty(last_difficulty: usize, new_difficulty: usize) -> bool {
        if last_difficulty < DIFFICULTY_MIN {
            return new_difficulty == DIFFICULTY_MIN;
        }
        if last_difficulty > DIFFICULTY_MAX {
            return new_difficulty == DIFFICULTY_MAX;
        }
        if new_difficulty == last_difficulty + 1 || new_difficulty == last_difficulty - 1 {
            return new_difficulty >= DIFFICULTY_MIN && new_difficulty <= DIFFICULTY_MAX;
        }
        if new_difficulty == DIFFICULTY_MIN || new_difficulty == DIFFICULTY_MAX {
            return last_difficulty == new_difficulty;
        }
        return false;
    }

    pub fn is_valid_block(
        block: &Block,
        last_block_hash: &String,
        last_block_difficulty: usize,
    ) -> bool {
        let Block {
            timestamp,
            last_hash,
            data,
            nonce,
            difficulty,
            hash,
        } = block;
        if last_hash != last_block_hash {
            return false;
        }
        if !Block::is_valid_difficulty(last_block_difficulty, block.difficulty) {
            return false;
        }
        if hash != &cryptohash::hash(timestamp, last_hash, data, *nonce, *difficulty) {
            return false;
        }
        if !cryptohash::is_valid_hash(hash, *difficulty) {
            return false;
        }

        return true;
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        return (self.timestamp == other.timestamp)
            && (self.data == other.data)
            && (self.hash == other.hash)
            && (self.last_hash == other.last_hash)
            && (self.difficulty == other.difficulty)
            && (self.nonce == other.nonce);
    }
}

impl Clone for Block {
    fn clone(&self) -> Block {
        return Block {
            timestamp: self.timestamp.clone(),
            last_hash: self.last_hash.clone(),
            hash: self.hash.clone(),
            data: self.data.clone(),
            nonce: self.nonce,
            difficulty: self.difficulty,
        };
    }
}
