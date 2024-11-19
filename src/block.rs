use sha2::{Sha256, Digest};  // SHA-256 for hashing
use chrono::prelude::*;      // For timestamp handling
use std::fmt;                // For custom formatting

/// Struct representing a blockchain block.
#[derive(Clone)]
pub struct Block {
    pub index: u32,              // Block index in the blockchain
    pub previous_hash: String,   // Hash of the previous block
    pub timestamp: String,       // Block creation timestamp
    pub data: String,            // Transaction or block data
    pub nonce: u64,              // Proof of work value
    pub hash: String,            // Block's hash
}

impl Block {
    /// Creates a new `Block` and calculates its initial hash.
    pub fn new(index: u32, previous_hash: &str, data: &str) -> Self {
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string(); // Format timestamp without milliseconds
        let millis = Utc::now().timestamp_millis() % 1000;  // Get milliseconds
        let timestamp = format!("{}.{:03}", timestamp, millis);  // Append milliseconds with 3 digits

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

    /// Calculates the SHA-256 hash of the block.
    pub fn calculate_hash(&self) -> String {
        let block_string = format!(
            "{}{}{}{}{}",
            self.index, self.previous_hash, self.timestamp, self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_string.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// Mines the block by finding a hash with the required difficulty.
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);  // Target for proof of work
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();  // Recalculate the hash with updated nonce
        }
    }

    /// Returns the block's hash.
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        // Two blocks are equal if they have the same hash.
        self.hash == other.hash
    }
}

/// Custom `Debug` implementation for `Block`.
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Block {} [Data: {}, Hash: {}, Previous Hash: {}, Timestamp: {}]",
            self.index,
            self.data,
            self.hash.get(..16).unwrap_or(&self.hash), // Display the first 16 characters or the full hash
            self.previous_hash.get(..16).unwrap_or(&self.previous_hash), // Display the first 16 characters or the full hash
            self.timestamp
        )
    }
}
