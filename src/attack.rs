use crate::blockchain::Blockchain;
use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}},
    thread,
    time::Duration,
};
use rand::Rng;

/// Generates random transactions between two participants with a random amount.
fn generate_random_transaction() -> String {
    // List of participants (fictional names).
    let names = vec!["Alice", "Bob", "Charlie", "Dave", "Eve", "Frank", "Grace"];
    let mut rng = rand::thread_rng();

    // Randomly selects the sender and receiver, ensuring they are different.
    let sender_index = rng.gen_range(0..names.len());
    let receiver_index = rng.gen_range(0..names.len());
    let amount: f64 = rng.gen_range(0.001..=10.0); // Transaction amount between 0.001 and 10.0 coins.

    // If the sender and receiver are the same, adjust the receiver index.
    let receiver_index = if sender_index == receiver_index {
        (receiver_index + 1) % names.len()
    } else {
        receiver_index
    };

    let sender = names[sender_index];
    let receiver = names[receiver_index];

    // Returns a string representing the transaction.
    format!("{} is transferring {:.3} Coins to {}", sender, amount, receiver)
}

/// Simulates a 51% attack, where the attacker attempts to outpace the original blockchain
/// by creating a longer chain than the legitimate one.
pub fn simulate_51_attack(difficulty: usize) {
    // Creates the original chain with the provided difficulty level.
    let mut original_chain = Blockchain::new(difficulty);

    // Adds 5 legitimate blocks to the original chain.
    for i in 1..=5 {
        original_chain.add_block(&format!("Transaction {}", i));
    }

    // Creates a copy of the original chain for the attacker's chain.
    let attacker_chain = original_chain.clone();

    // Uses Arc and Mutex to allow shared access between threads.
    let original_chain = Arc::new(Mutex::new(original_chain));
    let attacker_chain = Arc::new(Mutex::new(attacker_chain));

    // Atomic flag to signal when the attack has succeeded.
    let stop_flag = Arc::new(AtomicBool::new(false));

    // Thread simulating legitimate mining on the original chain.
    let original_chain_thread = {
        let original_chain = Arc::clone(&original_chain);
        let stop_flag = Arc::clone(&stop_flag);

        thread::spawn(move || {
            let mut i = 1;
            while !stop_flag.load(Ordering::Relaxed) {
                let transaction = generate_random_transaction();
                {
                    // Adds a new block to the original chain.
                    let mut chain = original_chain.lock().unwrap();
                    chain.add_block(&transaction);
                    println!("Legitimate: Mining Block {}", i);
                }
                i += 1;

                // Sleeps for a time proportional to the difficulty.
                thread::sleep(Duration::from_secs(2u64.pow(difficulty as u32)));
            }
        })
    };

    // Thread simulating malicious mining on the attacker's chain.
    let attacker_chain_thread = {
        let attacker_chain = Arc::clone(&attacker_chain);
        let original_chain = Arc::clone(&original_chain);
        let stop_flag = Arc::clone(&stop_flag);

        thread::spawn(move || {
            let mut i = 1;
            while !stop_flag.load(Ordering::Relaxed) {
                let transaction = generate_random_transaction();
                {
                    // Adds a new block to the attacker's chain.
                    let mut attacker = attacker_chain.lock().unwrap();
                    attacker.add_block(&transaction);
                    println!("Attack: Mining Block {}", i);
                }

                // Only checks the chain lengths in a critical section to avoid race conditions.
                let (attacker_len, original_len) = {
                    let attacker = attacker_chain.lock().unwrap();
                    let original = original_chain.lock().unwrap();
                    (attacker.chain.len(), original.chain.len())
                };

                // If the attacker's chain becomes longer than the original, the attack is considered successful.
                if attacker_len > original_len && original_len > 7 {
                    stop_flag.store(true, Ordering::SeqCst);
                }

                // Sleeps for a time proportional to the difficulty before continuing.
                thread::sleep(Duration::from_secs((2u64.pow(difficulty as u32))/2));
                i += 1;
            }
        })
    };

    // Waits for both threads to finish.
    original_chain_thread.join().unwrap();
    attacker_chain_thread.join().unwrap();

    // After mining, accesses and prints both chains.
    let original_chain = original_chain.lock().unwrap();
    let attacker_chain = attacker_chain.lock().unwrap();

    println!("\nOriginal Chain: {} Blocks", original_chain.chain.len());
    original_chain.is_valid();  // Validates the original chain.
    original_chain.print_chain();  // Prints the blocks of the original chain.

    println!("\nAttacker Chain: {} Blocks", attacker_chain.chain.len());
    attacker_chain.is_valid();  // Validates the attacker's chain.
    attacker_chain.print_chain();  // Prints the blocks of the attacker's chain.

    // Compares both chains to determine which one is valid.
    if attacker_chain.chain.len() > original_chain.chain.len() {
        println!("\nThe attacker's fork has become the valid chain!");

        // Identifies which blocks in the original chain were invalidated by the attack.
        let invalidated_blocks: Vec<_> = original_chain
            .chain
            .iter()
            .enumerate()
            .filter_map(|(i, block)| {
                // If the block in the attacker's chain is different, mark it as invalid.
                if attacker_chain.chain.get(i).map_or(true, |attacker_block| attacker_block != block) {
                    Some(block)
                } else {
                    None
                }
            })
            .collect();

        // If there are invalidated blocks, prints the transactions.
        if !invalidated_blocks.is_empty() {
            println!("\nInvalid transactions after the attack (not found in the attacker's chain):");
            for block in invalidated_blocks {
                println!("Transaction: {}", block.data);
            }
        }
    } else {
        // If the original chain is longer or equal, it remains valid.
        println!("\nThe original chain remains valid.");
    }
}
