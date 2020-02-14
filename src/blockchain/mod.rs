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


}

#[cfg(test)]
mod blockchain_new {
    use super::{Block, Blockchain};

    #[test]
    fn new_initializes_chain_properly() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain[0], Block::genesis());
    }
}
