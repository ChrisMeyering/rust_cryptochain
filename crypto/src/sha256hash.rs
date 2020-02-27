use hex::decode;

pub struct Sha256Hash {
    pub hash: [u8; 32],
}

impl Sha256Hash {
    pub fn new() -> Sha256Hash {
        return Sha256Hash { hash: [0; 32] };
    }

    pub fn from_str(hash_str: &String) -> Sha256Hash {
        let mut hash_bytes: [u8; 32] = [255; 32];
        hash_bytes.copy_from_slice(&decode(hash_str).unwrap()[..32]);
        return Sha256Hash { hash: hash_bytes };
    }
}
