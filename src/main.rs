use sha2::{Sha256, Digest};  // SHA-256 for hashing
use chrono::prelude::*;  // For timestamp handling
use rand::Rng;  // For random number generation
use std::fmt;  // For custom formatting

// Block struct representing a blockchain block
#[derive(Clone)]
struct Block {
    index: u32,          // Block index
    previous_hash: String,  // Previous block's hash
    timestamp: String,     // Block creation timestamp
    data: String,          // Transaction or block data
    nonce: u64,            // Proof of work value
    hash: String,          // Block's hash
}

impl Block {
    // Creates a new Block and calculates its hash
    fn new(index: u32, previous_hash: &str, data: &str) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut block = Block {
            index,
            previous_hash: previous_hash.to_string(),
            timestamp,
            data: data.to_string(),
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();  // Initial hash calculation
        block
    }

    // Calculates SHA-256 hash of the block
    fn calculate_hash(&self) -> String {
        let block_string = format!(
            "{}{}{}{}{}",
            self.index, self.previous_hash, self.timestamp, self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_string.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // Mines the block by finding a hash with required difficulty
    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

// Custom Debug output for Block
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Block {} [Data: {}, Hash: {}, Previous Hash: {}, Timestamp: {}]",
            self.index, self.data, self.hash, self.previous_hash, self.timestamp
        )
    }
}

// Generates a random name from a list
fn generate_random_name() -> &'static str {
    let names = ["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Hannah", "Ivy", "Jack"];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..names.len());
    names[index]
}

// Generates a random transaction string
fn generate_random_transaction() -> String {
    let sender = generate_random_name();
    let mut receiver = generate_random_name();
    while sender == receiver {
        receiver = generate_random_name();
    }
    let amount = rand::thread_rng().gen_range(0.0001..10.0);
    format!("{} paid {} {:.8} coins", sender, receiver, amount)
}

fn main() {
    let mut previous_hash = "0".to_string();  // Genesis block starts with hash "0"
    let difficulty = 4;  // Difficulty for mining (number of leading zeros)

    // Generate and mine 10 blocks
    for i in 0..10 {
        let data = if i == 0 {
            "Genesis Block".to_string()  // First block has fixed data
        } else {
            generate_random_transaction()  // Subsequent blocks have random transactions
        };

        let mut block = Block::new(i + 1, &previous_hash, &data);  // Create new block
        block.mine_block(difficulty);  // Mine the block

        println!("{:?}", block);  // Output block details
        previous_hash = block.hash.clone();  // Set previous hash for next block
    }
}
