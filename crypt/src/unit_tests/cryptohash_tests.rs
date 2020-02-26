
#[cfg(test)]
mod is_valid_hash {
    use super::is_valid_hash;

    #[test]
    fn on_valid_hash() {
        assert!(is_valid_hash(&String::from("05".repeat(32)), 5));
    }

    #[test]
    fn on_wrong_length_hash() {
        assert!(!is_valid_hash(&String::from("05".repeat(30)), 5));
    }

    #[test]
    fn on_hash_with_insufficient_leading_zeros() {
        assert!(!is_valid_hash(&String::from("05".repeat(30)), 6));
    }
}

#[cfg(test)]
mod hash {
    use super::{hash, SystemTime};

    #[test]
    fn generates_sha256_hash() {
        let timestamp = SystemTime::UNIX_EPOCH;
        let last_hash = String::new();
        let data = String::new();
        let nonce: usize = 0;
        let difficulty: usize = 0;
        assert_eq!(
            hash(&timestamp, &last_hash, &data, nonce, difficulty),
            String::from("b837bbff1e8e2196046b10761d8c62ea7a0ce2316b882c4d54783bd8b96085a1")
        );
    }
}