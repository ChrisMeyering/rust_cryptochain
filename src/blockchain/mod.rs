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
mod test_blockchain {
    use super::{Block, Blockchain};

    #[test]
    fn new_initializes_chain_properly() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain[0], Block::genesis());
    }

    #[test]
    fn adds_a_new_block_to_the_chain() {
        let mut blockchain = Blockchain::new();
        let initial_length = blockchain.chain.len();
        let new_block_data = String::from("some important data");
        blockchain.add_block(new_block_data.clone());
        assert_eq!(blockchain.chain.len(), initial_length + 1);
        assert_eq!(
            blockchain.chain[blockchain.chain.len() - 1].data,
            new_block_data
        );
    }
    
}
