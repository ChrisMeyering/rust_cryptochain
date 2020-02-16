mod block;

use block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let new_block = Block::mine_block(&self.chain[self.chain.len() - 1], data);
        self.chain.push(new_block);
    }

    pub fn is_valid_chain(chain: &Vec<Block>) -> bool {
        if chain[0] != Block::genesis() {
            return false;
        }
        for i in 1..chain.len() {
            if !Block::is_valid_block(&chain[i], &chain[i - 1].hash, chain[i - 1].difficulty) {
                return false;
            }
        }
        return true;
    }

    pub fn replace_chain(&mut self, new_chain: Vec<Block>) {
        if new_chain.len() > self.chain.len() && Blockchain::is_valid_chain(&new_chain) {
            self.chain = new_chain;
        }
    }
}

#[cfg(test)]
mod blockchain_struct_data {
    use super::{Block, Blockchain};

    #[test]
    fn new_initializes_chain_properly() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain[0], Block::genesis());
    }
}

#[cfg(test)]
mod add_block {
    use super::{Block, Blockchain};

    #[test]
    fn adds_a_new_block_to_the_chain() {
        let mut blockchain = Blockchain::new();
        let initial_length = blockchain.chain.len();
        let new_block_data = String::from("some important data");
        blockchain.add_block(new_block_data.clone());
        assert_eq!(blockchain.chain.len(), initial_length + 1);
    }

    #[test]
    fn new_block_is_valid() {
        let mut blockchain = Blockchain::new();
        let new_block_data = String::from("some important data");
        blockchain.add_block(new_block_data.clone());
        let len = blockchain.chain.len();
        assert!(Block::is_valid_block(
            &blockchain.chain[len - 1],
            &blockchain.chain[len - 2].hash,
            blockchain.chain[len - 2].difficulty
        ));
    }
}

#[cfg(test)]
mod is_valid_chain {
    use super::{Blockchain, Block, block::cryptohash};

    fn setup() -> Blockchain {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(String::from("Raccoons are cool"));
        blockchain.add_block(String::from("Skunks smell bad"));
        blockchain.add_block(String::from("Bears are big"));
        return blockchain;
    }

    #[test]
    fn false_if_first_block_neq_genesis() {
        let mut blockchain = setup();
        blockchain.chain[0].data = String::from("fake data");
        assert!(!Blockchain::is_valid_chain(&blockchain.chain));
    }

    #[test]
    fn false_if_a_last_hash_reference_has_changed() {
        let mut blockchain = setup();
        blockchain.chain[2].last_hash = String::from("fake last_hash");
        assert!(!Blockchain::is_valid_chain(&blockchain.chain));
    }

    #[test]
    fn false_if_chain_contains_block_with_jumped_difficulty() {
        let mut blockchain = setup();
        let timestamp = std::time::SystemTime::now();
        let data = String::new();
        let last_hash = blockchain.chain[blockchain.chain.len() - 1].hash.clone();
        let nonce = 0;
        let difficulty = blockchain.chain[blockchain.chain.len() - 1].difficulty + 3;
        let hash = cryptohash::hash(&timestamp, &last_hash, &data, nonce, difficulty);
        blockchain.chain.push(Block {
            timestamp,
            last_hash,
            hash,
            data,
            nonce,
            difficulty
        });
        assert!(!Blockchain::is_valid_chain(&blockchain.chain));
    }

    #[test]
    fn false_if_chain_contains_block_with_invalid_field() {
        let mut blockchain = setup();
        blockchain.chain[2].data = String::from("Sunks smell good");
        assert!(!Blockchain::is_valid_chain(&blockchain.chain));
    }

    #[test]
    fn false_if_chain_contains_block_with_difficulty_constraint_violated() {
        let mut blockchain = setup();
        let timestamp = std::time::SystemTime::now();
        let data = String::new();
        let last_hash = blockchain.chain[blockchain.chain.len() - 1].hash.clone();
        let nonce = 0;
        let difficulty = blockchain.chain[blockchain.chain.len() - 1].difficulty + 1;
        let hash = cryptohash::hash(&timestamp, &last_hash, &data, nonce, difficulty);
        blockchain.chain.push(Block {
            timestamp,
            last_hash,
            hash,
            data,
            nonce,
            difficulty
        });
        assert!(!Blockchain::is_valid_chain(&blockchain.chain));
    }

    #[test]
    fn true_if_chain_contains_only_valid_block() {
        let mut blockchain = setup();
        blockchain.add_block(String::from("This chain is valid!"));
        assert!(Blockchain::is_valid_chain(&blockchain.chain));
    }
}

#[cfg(test)]
mod replace_chain {
    use super::Blockchain;

    fn setup() -> Blockchain {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(String::from("Raccoons are cool"));
        blockchain.add_block(String::from("Skunks smell bad"));
        blockchain.add_block(String::from("Bears are big"));
        return blockchain;
    }

    #[test]
    fn does_not_replace_chain_when_new_chain_is_not_longer() {
        let mut blockchain = setup();
        let new_blockchain = Blockchain::new();
        let original_chain = blockchain.chain.clone();
        blockchain.replace_chain(new_blockchain.chain);
        assert_eq!(blockchain.chain, original_chain);
    }

    #[test]
    fn does_not_replace_chain_when_new_chain_is_longer_but_contains_invalid_block() {
        let mut blockchain = Blockchain::new();
        let mut new_blockchain = setup();
        new_blockchain.chain[2].data = String::from("Skunks smell good");
        let original_chain = blockchain.chain.clone();
        blockchain.replace_chain(new_blockchain.chain);
        assert_eq!(blockchain.chain, original_chain);
    }

    #[test]
    fn replaces_chain_when_new_chain_is_longer_and_valid() {
        let mut blockchain = Blockchain::new();
        let new_blockchain = setup();
        let original_chain = blockchain.chain.clone();
        blockchain.replace_chain(new_blockchain.chain);
        assert_ne!(blockchain.chain, original_chain);
    }
}