use crypto::cryptohash;

use crate::{
    block::Block,
    blockchain::Blockchain
};

mod blockchain_struct_data {
    use super::*;
    #[test]
    fn new_initializes_chain_properly() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain[0], Block::genesis());
    }
}

mod add_block {
    use super::*;
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

mod is_valid_chain {
    use super::*;

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
        blockchain.chain[2].last_hash = [13; 32];
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
        let mut data_map = Block::get_data_map(&timestamp, &last_hash, &data, nonce, difficulty); 
        let mut hash: [u8; 32] = [13; 32];
        cryptohash::hash(&data_map, &mut hash);
        blockchain.chain.push(Block {
            timestamp,
            last_hash,
            hash,
            data,
            nonce,
            difficulty,
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
        let data_map = Block::get_data_map(&timestamp, &last_hash, &data, nonce, difficulty);
        let mut hash: [u8; 32] = [0; 32];
        cryptohash::hash(&data_map, &mut hash);
        blockchain.chain.push(Block {
            timestamp,
            last_hash,
            hash,
            data,
            nonce,
            difficulty,
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
