use crate::block::Block;  // Importing the Block struct
use std::vec::Vec;

/// Struct representing the entire blockchain.
#[derive(Default, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,  // Vector of blocks forming the blockchain
    pub difficulty: usize,  // Mining difficulty (number of leading zeros required in the hash)
}

impl Blockchain {
    /// Creates a new `Blockchain` with a specified difficulty level.
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();  // Creates the genesis block (first block in the chain)
        blockchain
    }

    /// Creates the genesis (first) block and adds it to the blockchain.
    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "0", "Genesis Block");
        let mut mined_genesis_block = genesis_block;
        mined_genesis_block.mine_block(self.difficulty);
        self.chain.push(mined_genesis_block);
    }

    /// Adds a new block to the blockchain.
    pub fn add_block(&mut self, data: &str) {
        let previous_block = self.chain.last().expect("Blockchain should have at least one block.");
        let new_block = Block::new(
            previous_block.index + 1,
            &previous_block.hash,
            data,
        );
        let mut mined_block = new_block;
        mined_block.mine_block(self.difficulty);  // Mine the block before adding it to the chain
        self.chain.push(mined_block);
    }

    /// Validates the entire blockchain to ensure all blocks are valid.
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Validate the previous hash of the current block matches the hash of the previous block.
            if current_block.previous_hash != previous_block.hash {
                println!("\nBlockchain invalid: Block {} has an incorrect previous hash.", i);
                return false;  // Invalid blockchain: mismatched previous hash
            }

            // Validate the current block's hash is correctly calculated.
            if current_block.hash != current_block.calculate_hash() {
                println!("\nBlockchain invalid: Block {} has an incorrect hash.", i);
                return false;  // Invalid blockchain: incorrect block hash
            }

            // Validate the current block's hash meets the required difficulty.
            if !current_block.get_hash().starts_with(&"0".repeat(self.difficulty)) {
                println!("\nBlockchain invalid: Block {}'s hash does not meet the required difficulty.", i);
                return false;  // Invalid blockchain: hash does not meet difficulty
            }
        }
        println!("Blockchain is valid: All blocks are correctly linked and meet the difficulty requirements.");
        true  // Blockchain is valid
    }


    /// Prints the entire blockchain.
    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}
