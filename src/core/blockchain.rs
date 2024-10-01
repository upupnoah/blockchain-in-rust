use crate::core::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>, // A vector of blocks forming the blockchain
    pub difficulty: usize,  // Difficulty level for mining
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    /// Creates the genesis (first) block in the blockchain
    fn create_genesis_block(&mut self) {
        let mut genesis_block = Block::new(0, "0".to_string(), "Genesis Block".to_string());
        genesis_block.mine_block(self.difficulty);
        self.blocks.push(genesis_block);
    }

    /// Gets the latest block in the blockchain
    pub fn latest_block(&self) -> &Block {
        self.blocks.last().expect("Blockchain has no blocks")
    }

    /// Adds a new block to the blockchain
    pub fn add_block(&mut self, data: String) {
        let latest_block = self.latest_block();
        let mut new_block = Block::new(latest_block.index + 1, latest_block.hash.clone(), data);
        new_block.mine_block(self.difficulty);
        self.blocks.push(new_block);
    }

    /// Validates the integrity of the blockchain
    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];
            if current_block.hash
                != Block::calculate_hash(
                    current_block.index,
                    current_block.timestamp,
                    &current_block.prev_hash,
                    current_block.nonce,
                    &current_block.data,
                )
            {
                return false;
            }
            if current_block.prev_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}
