use crate::block::Block;  // Importing Block struct
use std::vec::Vec;

/// Blockchain struct representing the entire chain of blocks
#[derive(Default)]
pub struct Blockchain {
    pub chain: Vec<Block>,  // Vector of blocks forming the blockchain
    pub difficulty: usize,   // Difficulty for mining (number of leading zeros in the hash)
}

impl Blockchain {
    /// Creates a new blockchain with an optional difficulty level
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();  // Creates the first block (genesis block)
        blockchain
    }

    /// Creates the genesis (first) block and adds it to the chain
    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "0", "Genesis Block");
        self.chain.push(genesis_block);
    }

    /// Adds a new block to the blockchain
    pub fn add_block(&mut self, data: &str) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            &previous_block.hash,
            data
        );
        let mut block_to_add = new_block;
        block_to_add.mine_block(self.difficulty);  // Mine the block before adding it to the chain
        self.chain.push(block_to_add);
    }

    /// Validates the entire blockchain to ensure all blocks are valid
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check if the current block's previous_hash matches the hash of the previous block
            if current_block.previous_hash != previous_block.hash {
                return false;  // Blockchain is invalid if the hashes don't match
            }

            // Check if the current block's hash is valid (must be correctly calculated)
            if current_block.hash != current_block.calculate_hash() {
                return false;  // Blockchain is invalid if the hash is not valid
            }

            // Check if the current block's hash starts with the required number of leading zeros,
            // based on the blockchain's mining difficulty. The blockchain is invalid if it doesn't.
            if !current_block.get_hash().starts_with(&"0".repeat(self.difficulty)) {
                return false;  // Blockchain is invalid if the hash does not meet the difficulty
            }
        }
        true  // Blockchain is valid
    }

    /// Prints the entire blockchain for display purposes
    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}

