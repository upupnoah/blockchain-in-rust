use serde::{Deserialize, Serialize};

/// Represents a simple block in the blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,        // Block index (height)
    pub timestamp: u128,   // Timestamp of block creation
    pub prev_hash: String, // Hash of the previous block
    pub hash: String,      // Hash of the current block
    pub data: String,      // Data stored in the block (simplified as a string)
    pub nonce: u64,        // Nonce used for proof-of-work (mining)
}

impl Block {
    /// Creates a new block
    pub fn new(index: u64, prev_hash: String, data: String) -> Self {
        let timestamp = Self::current_timestamp();
        let nonce = 0;
        let hash = Self::calculate_hash(index, timestamp, &prev_hash, nonce, &data);
        Self {
            index,
            timestamp,
            prev_hash,
            hash,
            data,
            nonce,
        }
    }

    /// Gets the current timestamp
    fn current_timestamp() -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};

        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    /// Calculates the hash of the block's contents
    pub fn calculate_hash(
        index: u64,
        timestamp: u128,
        prev_hash: &str,
        nonce: u64,
        data: &str,
    ) -> String {
        use sha3::{Digest, Sha3_256};
        let content = format!(
            "{:?}{:?}{:?}{:?}{:?}",
            index, timestamp, prev_hash, nonce, data
        );
        let mut hasher = Sha3_256::new(); // hash function
        hasher.update(content);
        let hash = hasher.finalize();

        base16ct::lower::encode_string(&hash)
    }

    /// Mines the block by finding a hash that meets the difficulty requirement
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = Self::calculate_hash(
                self.index,
                self.timestamp,
                &self.prev_hash,
                self.nonce,
                &self.data,
            );
        }
        println!("Block mined: {}", self.hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block() {
        let block = Block::new(0, "0".to_string(), "Genesis Block".to_string());
        assert_eq!(block.index, 0);
        assert_eq!(block.prev_hash, "0");
        assert_eq!(block.data, "Genesis Block");
        assert!(!block.hash.is_empty());
    }

    #[test]
    fn test_calculate_hash_consistency() {
        let index = 1;
        let timestamp = Block::current_timestamp();
        let prev_hash = "prev_hash_example";
        let nonce = 0;
        let data = "Test Data";

        let hash1 = Block::calculate_hash(index, timestamp, prev_hash, nonce, data);
        let hash2 = Block::calculate_hash(index, timestamp, prev_hash, nonce, data);

        assert_eq!(
            hash1, hash2,
            "The same input should produce a consistent hash value."
        );
    }

    #[test]
    fn test_calculate_hash_variation() {
        let index = 1;
        let timestamp = Block::current_timestamp();
        let prev_hash = "prev_hash_example";
        let nonce = 0;
        let data = "Test Data";

        let hash1 = Block::calculate_hash(index, timestamp, prev_hash, nonce, data);
        let hash2 = Block::calculate_hash(index + 1, timestamp, prev_hash, nonce, data);

        assert_ne!(
            hash1, hash2,
            "Different inputs should produce different hash values."
        );
    }

    #[test]
    fn test_mine_block() {
        let mut block = Block::new(1, "0".to_string(), "Some Transaction Data".to_string());
        let difficulty = 2; // For example, the difficulty is set to the first two characters being '00'.
        block.mine_block(difficulty);

        assert!(
            block.hash[..difficulty] == "0".repeat(difficulty),
            "The hash value after mining should meet the difficulty requirements"
        );
        println!("Mining successful, hash value is: {}", block.hash);
    }

    #[test]
    fn test_block_nonce_increments_during_mining() {
        let mut block = Block::new(2, "0".to_string(), "Another Transaction".to_string());
        let initial_nonce = block.nonce;
        block.mine_block(2);

        assert!(
            block.nonce > initial_nonce,
            "The mining process should increase the nonce value."
        );
    }

    #[test]
    fn test_current_timestamp() {
        let timestamp1 = Block::current_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let timestamp2 = Block::current_timestamp();

        assert!(
            timestamp2 >= timestamp1,
            "The timestamp obtained later should not be less than the previous one."
        );
    }

    #[test]
    fn test_block_chain_integration() {
        // 测试区块与前一区块的关联
        let genesis_block = Block::new(0, "0".to_string(), "Genesis Block".to_string());
        let mut second_block = Block::new(
            genesis_block.index + 1,
            genesis_block.hash.clone(),
            "Second Block Data".to_string(),
        );
        second_block.mine_block(2);

        assert_eq!(
            second_block.prev_hash, genesis_block.hash,
            "The previous hash of the new block should be equal to the hash of the previous block."
        );
    }
}
