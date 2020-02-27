use crate::cryptohash;

use std::collections::BTreeMap;
use hex::decode;

#[cfg(test)]
mod is_valid_hash {
    use super::*;

    #[test]
    fn on_valid_hash() {
        let mut mock_hash: [u8; 32] = [255; 32];
        mock_hash[0] = 0;
        mock_hash[1] = 127;
        assert!(cryptohash::is_valid_hash(&mock_hash, 9));
    }

    #[test]
    fn on_wrong_length_hash() {
        let mock_hash: [u8; 31] = [0; 31];
        assert!(!cryptohash::is_valid_hash(&mock_hash, 0));
    }

    #[test]
    fn on_hash_with_insufficient_leading_zeros() {
        let mut mock_hash: [u8; 32] = [0; 32];
        mock_hash[1] = 128;
        assert!(!cryptohash::is_valid_hash(&mock_hash, 9));
    }
}

#[cfg(test)]
mod hash {
    use super::*;

    #[test]
    fn generates_sha256_hash() {
        let mut data_map = BTreeMap::<String, String>::new();
        data_map.insert(String::from("foo"), String::from("data"));

        let mut hash: [u8; 32] = [0; 32];
        cryptohash::hash(&data_map, &mut hash);

        let mut expected_hash: [u8; 32] = [255; 32];
        let bytes = &decode("403ddf7b8ee78743e1eecf3474f6e4c9e71b5ce4611dfb475f27ab58deab1b9c").unwrap()[..32];
        expected_hash.copy_from_slice(bytes);

        assert_eq!(hash, expected_hash);
    }
}