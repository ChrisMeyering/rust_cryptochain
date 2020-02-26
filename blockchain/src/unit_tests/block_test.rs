use crypt::cryptohash::*;

use crate::{
    block::Block,
    config::*
};

use std::{time::SystemTime};


mod block_struct_data {
    use super::*;

    #[test]
    fn struct_has_proper_fields() {
        let timestamp = SystemTime::now();
        let last_hash = String::from("mockhash".repeat(16));
        let hash = String::from("hashmock".repeat(16));
        let data = String::from("mock data");
        let nonce: usize = 128;
        let difficulty: usize = 0;
        let block = Block {
            timestamp,
            last_hash: last_hash.clone(),
            hash: hash.clone(),
            data: data.clone(),
            nonce,
            difficulty,
        };
        assert_eq!(block.timestamp, timestamp);
        assert_eq!(block.last_hash, last_hash);
        assert_eq!(block.hash, hash);
        assert_eq!(block.data, data);
        assert_eq!(block.nonce, nonce);
        assert_eq!(block.difficulty, difficulty);
    }

    #[test]
    fn genesis_creates_block_instance_with_expected_data() {
        let timestamp = SystemTime::UNIX_EPOCH;
        let last_hash = String::from("none");
        let hash = String::from("genesis");
        let data = String::from("genesis block");
        let nonce = 0;
        let difficulty = 8;
        let genesis_block = Block::genesis();
        assert_eq!(genesis_block.timestamp, timestamp);
        assert_eq!(genesis_block.last_hash, last_hash);
        assert_eq!(genesis_block.hash, hash);
        assert_eq!(genesis_block.data, data);
        assert_eq!(genesis_block.nonce, nonce);
        assert_eq!(genesis_block.difficulty, difficulty);
    }
}

mod mine_block {
    use super::*;

    fn setup() -> (Block, String, Block) {
        let last_block = Block::genesis();
        let data = String::from("some data");
        let mined_block = Block::mine_block(&last_block, data.clone());
        return (last_block, data, mined_block);
    }

    #[test]
    fn sets_new_block_last_hash_to_last_block_hash() {
        let (last_block, _, mined_block) = setup();
        assert_eq!(mined_block.last_hash, last_block.hash);
    }

    #[test]
    fn sets_valid_new_difficulty() {
        let (last_block, _, mined_block) = setup();
        assert!(Block::is_valid_difficulty(
            last_block.difficulty,
            mined_block.difficulty
        ));
    }

    #[test]
    fn sets_data_field() {
        let (_, data, mined_block) = setup();
        assert_eq!(mined_block.data, data);
    }
    #[test]
    fn sets_hash_based_on_input() {
        let (last_block, data, mined_block) = setup();
        let expected_hash = hash(
            &mined_block.timestamp,
            &last_block.hash,
            &data,
            mined_block.nonce,
            mined_block.difficulty,
        );
        assert_eq!(mined_block.hash, expected_hash);
    }

    #[test]
    fn hash_has_256_bits_and_matches_difficulty_constraint() {
        let (_, _, mined_block) = setup();
        assert!(is_valid_hash(
            &mined_block.hash,
            mined_block.difficulty
        ));
    }
}

mod adjust_difficulty {
    use super::*;
    use std::time::Duration;

    #[test]
    fn raises_difficulty_for_quickly_mined_block() {
        let block = Block::mine_block(&Block::genesis(), String::from("some data"));
        let new_timestamp = block.timestamp + Duration::from_millis(MINE_RATE - 100);
        assert_eq!(
            Block::adjust_difficulty(&block, &new_timestamp),
            block.difficulty + 1
        );
    }

    #[test]
    fn lowers_difficulty_for_slowly_mined_block() {
        let block = Block::mine_block(&Block::genesis(), String::from("some data"));
        let new_timestamp = block.timestamp + Duration::from_millis(MINE_RATE + 100);
        assert_eq!(
            Block::adjust_difficulty(&block, &new_timestamp),
            block.difficulty - 1
        );
    }

    #[test]
    fn increases_difficulty_if_elapsed_time_is_negative() {
        let block = Block::mine_block(&Block::genesis(), String::from("some data"));
        let new_timestamp = block.timestamp - Duration::from_millis(MINE_RATE);
        assert_eq!(
            Block::adjust_difficulty(&block, &new_timestamp),
            block.difficulty + 1
        );
    }

    #[test]
    fn has_correct_lower_limit() {
        let mut block = Block::mine_block(&Block::genesis(), String::from("some data"));
        block.difficulty = DIFFICULTY_MIN;
        let new_timestamp = block.timestamp + Duration::from_millis(MINE_RATE + 100);
        assert_eq!(
            Block::adjust_difficulty(&block, &new_timestamp),
            block.difficulty
        );
    }

    #[test]
    fn has_correct_upper_limit() {
        let mut block = Block::mine_block(&Block::genesis(), String::from("some data"));
        block.difficulty = DIFFICULTY_MAX;
        let new_timestamp = block.timestamp + Duration::from_millis(MINE_RATE - 100);
        assert_eq!(
            Block::adjust_difficulty(&block, &new_timestamp),
            block.difficulty
        );
    }

    #[test]
    fn adjusts_difficulty_if_out_of_bounds() {
        let mut block = Block::mine_block(&Block::genesis(), String::from("some data"));
        block.difficulty = 0;
        assert_eq!(
            Block::adjust_difficulty(&block, &block.timestamp),
            DIFFICULTY_MIN
        );

        block.difficulty = std::usize::MAX;
        assert_eq!(
            Block::adjust_difficulty(&block, &block.timestamp),
            DIFFICULTY_MAX
        );
    }
}

mod is_valid_difficulty {
    use super::*;

    mod difficulties_offest_by_one {
        use super::*;

        #[test]
        fn true_when_new_difficulty_within_bounds() {
            assert!(Block::is_valid_difficulty(8, 9));
            assert!(Block::is_valid_difficulty(12, 11));
        }
        #[test]
        fn false_when_new_difficulty_outside_of_bounds() {
            assert!(!Block::is_valid_difficulty(
                DIFFICULTY_MIN,
                DIFFICULTY_MIN - 1
            ));
            assert!(!Block::is_valid_difficulty(
                DIFFICULTY_MIN - 1,
                DIFFICULTY_MIN - 2
            ));
            assert!(!Block::is_valid_difficulty(
                DIFFICULTY_MAX,
                DIFFICULTY_MAX + 1
            ));
        }
    }

    mod difficulties_are_equal {
        use super::*;

        #[test]
        fn false_if_not_min_or_max() {
            assert!(!Block::is_valid_difficulty(12, 12));
            assert!(!Block::is_valid_difficulty(
                DIFFICULTY_MIN - 1,
                DIFFICULTY_MIN - 1
            ));
            assert!(!Block::is_valid_difficulty(
                DIFFICULTY_MAX + 1,
                DIFFICULTY_MAX + 1
            ));
        }
        #[test]
        fn true_if_min_or_max() {
            assert!(Block::is_valid_difficulty(DIFFICULTY_MIN, DIFFICULTY_MIN));
            assert!(Block::is_valid_difficulty(DIFFICULTY_MAX, DIFFICULTY_MAX));
        }
    }

    mod difficulties_are_offset_by_more_than_one {
        use super::*;
        #[test]
        fn false_if_both_difficulties_within_bounds() {
            assert!(!Block::is_valid_difficulty(24, 26))
        }

        #[test]
        fn false_if_new_difficulty_out_of_bounds() {
            assert!(!Block::is_valid_difficulty(25, DIFFICULTY_MIN - 1));
            assert!(!Block::is_valid_difficulty(25, DIFFICULTY_MAX + 1));
        }

        #[test]
        fn false_if_both_difficulties_out_of_bounds() {
            assert!(!Block::is_valid_difficulty(
                DIFFICULTY_MIN - 1,
                DIFFICULTY_MAX + 1
            ));
        }

        mod only_new_difficulty_within_bounds {
            use super::*;

            #[test]
            fn true_if_new_difficulty_equals_closest_bound() {
                assert!(Block::is_valid_difficulty(0, DIFFICULTY_MIN));
                assert!(Block::is_valid_difficulty(
                    DIFFICULTY_MAX + 4,
                    DIFFICULTY_MAX
                ));
            }

            #[test]
            fn false_if_new_difficulty_not_equal_to_closest_bound() {
                assert!(!Block::is_valid_difficulty(0, DIFFICULTY_MAX));
                assert!(!Block::is_valid_difficulty(
                    DIFFICULTY_MAX + 1,
                    DIFFICULTY_MIN
                ));
                assert!(!Block::is_valid_difficulty(0, 12));
            }
        }
    }
}

mod is_valid_block {
    use super::*;

    #[test]
    fn false_if_new_block_last_hash_neq_last_block_hash() {
        let mut last_block: Block =
            Block::mine_block(&Block::genesis(), String::from("dummy data 1"));
        let new_block = Block::mine_block(&last_block, String::from("dummy data 2"));
        last_block.hash = String::from("tampered hash");
        assert!(!Block::is_valid_block(
            &new_block,
            &last_block.hash,
            last_block.difficulty
        ));
    }

    #[test]
    fn false_if_new_block_difficulty_invalid() {
        let mut last_block: Block =
            Block::mine_block(&Block::genesis(), String::from("dummy data 1"));
        let new_block = Block::mine_block(&last_block, String::from("dummy data 2"));
        last_block.difficulty = 20;
        assert!(!Block::is_valid_block(
            &new_block,
            &last_block.hash,
            last_block.difficulty
        ));
    }

    #[test]
    fn false_if_new_block_contents_modified() {
        let last_block: Block = Block::mine_block(&Block::genesis(), String::from("dummy data 1"));
        let mut new_block = Block::mine_block(&last_block, String::from("dummy data 2"));
        new_block.data = String::from("tampred data");
        assert!(!Block::is_valid_block(
            &new_block,
            &last_block.hash,
            last_block.difficulty
        ));
    }

    #[test]
    fn false_if_new_block_hash_violates_difficulty_constraint() {
        let last_block: Block = Block::mine_block(&Block::genesis(), String::from("dummy data 1"));

        let timestamp: SystemTime = SystemTime::now();
        let last_hash: String = last_block.hash.clone();
        let data: String = String::from("dummy data 2");
        let nonce: usize = 0;
        let difficulty: usize = last_block.difficulty + 1;
        let hash = hash(&timestamp, &last_hash, &data, nonce, difficulty);
        let new_block = Block {
            timestamp,
            last_hash: last_hash,
            hash: hash.clone(),
            data: data.clone(),
            nonce,
            difficulty,
        };
        assert!(!Block::is_valid_block(
            &new_block,
            &last_block.hash,
            last_block.difficulty
        ));
    }

    #[test]
    fn true_if_new_block_is_valid() {
        let last_block: Block = Block::mine_block(&Block::genesis(), String::from("dummy data 1"));
        let new_block: Block = Block::mine_block(&last_block, String::from("dummy data 2"));
        assert!(Block::is_valid_block(
            &new_block,
            &last_block.hash,
            last_block.difficulty
        ));
    }
}
