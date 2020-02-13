
mod block;

use block::Block;

fn main() {
    let block = Block::genesis();
    println!("{:#?}", block);

    let mut last_block = block;
    for i in 0..4 {
        let new_block = Block::mine_block(&last_block, String::from(format!("block {}", i).as_str()));
        println!("{:#?}", new_block);
        last_block = new_block;
    }
}
