extern crate cryptochain;

use cryptochain::blockchain::Blockchain;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    let blockchain = Blockchain::new();
}