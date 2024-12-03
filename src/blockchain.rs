use crate::block::Block;
use std::vec::Vec;

#[derive(Default, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,  // Vector of blocks that form the entire blockchain.
    pub difficulty: usize,  // Mining difficulty, represents the required number of leading zeros in the block hash.
}

impl Blockchain {
    /// Creates a new `Blockchain` with a specified difficulty level.
    /// The difficulty level will affect how hard it is to mine a block (more zeros in the hash).
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),  // Initializes the blockchain with an empty chain.
            difficulty,
        };
        blockchain.create_genesis_block();  // Adds the genesis block (first block in the chain).
        blockchain
    }

    /// Creates the genesis block (the very first block) and adds it to the blockchain.
    /// The genesis block has no previous block, so its previous hash is set to "0".
    fn create_genesis_block(&mut self) {
        // Create the first block with index 0 and a "0" previous hash.
        let genesis_block = Block::new(0, "0", "Genesis Block");

        // Mine the genesis block (find a valid hash that meets the difficulty).
        let mut mined_genesis_block = genesis_block;
        mined_genesis_block.mine_block(self.difficulty);

        // Add the mined genesis block to the blockchain.
        self.chain.push(mined_genesis_block);
    }

    /// Adds a new block to the blockchain, with the specified data.
    /// The new block’s index will be the previous block’s index + 1.
    pub fn add_block(&mut self, data: &str) {
        // Get the last block in the current chain (previous block).
        let previous_block = self.chain.last().expect("Blockchain should have at least one block.");

        // Create a new block, with the previous block’s hash and new data.
        let new_block = Block::new(
            previous_block.index + 1,  // Index of the new block (previous block’s index + 1).
            &previous_block.hash,       // Hash of the previous block.
            data,                        // Data to be stored in the new block.
        );

        // Mine the new block and add it to the blockchain.
        self.mine_and_add_block(new_block);
    }

    /// Mines and adds a new block to the blockchain.
    /// This method finds the valid hash for the block by mining it (proof of work).
    fn mine_and_add_block(&mut self, mut new_block: Block) {
        // Mine the block by finding a valid hash that meets the difficulty.
        new_block.mine_block(self.difficulty);

        // Add the mined block to the blockchain.
        self.chain.push(new_block);
    }

    /// Validates the entire blockchain by checking that:
    /// 1. Each block correctly links to its previous block.
    /// 2. Each block's hash is valid (it matches the calculated hash).
    /// 3. The hash meets the required difficulty (starts with a number of zeros).
    pub fn is_valid(&self) -> bool {
        // Iterate over the blocks starting from the second block (index 1).
        self.chain.iter()
            .enumerate()
            .skip(1)  // Skip the genesis block since it has no previous block to validate.
            .all(|(i, current_block)| {
                let previous_block = &self.chain[i - 1];  // Get the previous block.

                // Check if the current block’s previous hash matches the previous block’s hash.
                current_block.previous_hash == previous_block.hash
                    // Check if the block’s hash is valid by recalculating it and comparing.
                    && current_block.hash == current_block.calculate_hash()
                    // Check if the block’s hash meets the required difficulty (starts with enough zeros).
                    && current_block.get_hash().starts_with(&"0".repeat(self.difficulty))
            })
    }

    /// Prints the entire blockchain, displaying each block.
    pub fn print_chain(&self) {
        for block in &self.chain {
            // Prints the block's details using the Debug trait implementation.
            println!("{:?}", block);
        }
    }
}
