mod block;

use block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain {
            chain: vec![Block::genesis()];
        }
    }

}