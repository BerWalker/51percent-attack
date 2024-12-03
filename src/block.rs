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
    /// Creates a new `Block` with the specified index, previous block's hash, and data.
    /// It also calculates the initial hash of the block.
    pub fn new(index: u32, previous_hash: &str, data: &str) -> Self {
        // Generate the timestamp in the format: "YYYY-MM-DDTHH:MM:SS.mmm"
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let millis = Utc::now().timestamp_millis() % 1000;  // Get the millisecond portion of the timestamp
        let timestamp = format!("{}.{:03}", timestamp, millis);  // Append milliseconds

        // Create a new block with the specified parameters and an initial empty hash.
        let mut block = Block {
            index,
            previous_hash: previous_hash.to_string(),
            timestamp,
            data: data.to_string(),
            nonce: 0,  // Initial nonce (proof of work value)
            hash: String::new(),
        };

        // Calculate and assign the initial hash to the block
        block.hash = block.calculate_hash();
        block
    }

    /// Calculates the SHA-256 hash of the block by hashing its relevant fields (index, previous hash, timestamp, data, nonce).
    pub fn calculate_hash(&self) -> String {
        // Concatenate the block's fields into a single string for hashing
        let block_string = format!(
            "{}{}{}{}{}",  // Format block fields into a string
            self.index, self.previous_hash, self.timestamp, self.data, self.nonce
        );

        // Create a SHA-256 hasher and compute the hash
        let mut hasher = Sha256::new();
        hasher.update(block_string.as_bytes());
        format!("{:x}", hasher.finalize())  // Return the hash as a hexadecimal string
    }

    /// Mines the block by repeatedly adjusting the nonce and recalculating the hash
    /// until it meets the required difficulty (starts with a certain number of zeros).
    pub fn mine_block(&mut self, difficulty: usize) {
        // Create a target string of zeros that the block hash must start with
        let target = "0".repeat(difficulty);

        // Continuously update the nonce and recalculate the hash until the block meets the target
        while !self.hash.starts_with(&target) {
            self.nonce += 1;  // Increment nonce to try a new hash
            self.hash = self.calculate_hash();  // Recalculate the hash with the updated nonce
        }
    }

    /// Returns the block's hash as a reference.
    pub fn get_hash(&self) -> &str {
        &self.hash  // Return a reference to the block's hash, avoiding unnecessary cloning
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        // Two blocks are considered equal if their hashes are the same.
        self.hash == other.hash
    }
}

/// Custom `Debug` implementation for `Block` to format its output in a readable way.
impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write the formatted string for the block, displaying key information
        write!(
            f,
            "Block {} [Data: {}, Hash: {}, Previous Hash: {}, Timestamp: {}]",
            self.index,
            self.data,
            &self.hash.get(..16).unwrap_or(&self.hash),  // Display the first 16 characters or the full hash
            &self.previous_hash.get(..16).unwrap_or(&self.previous_hash),  // Display the first 16 characters or the full hash
            self.timestamp
        )
    }
}
