use crate::block::Block;

#[derive(Debug)]
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
