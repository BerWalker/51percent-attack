use crate::blockchain::Blockchain;
use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}},
    thread,
    time,
};
use rand::Rng;

/// Function to generate random transactions with two participants and a random amount.
fn generate_random_transaction() -> String {
    // List of generic names for the participants.
    let names = vec!["Alice", "Bob", "Grace", "Eve", "Charlie", "Dave"];

    // Generate two different random indices to select two people from the list.
    let mut rng = rand::thread_rng();
    let sender_index = rng.gen_range(0..names.len());
    let mut receiver_index = rng.gen_range(0..names.len());

    // Ensure that the sender and the receiver are different
    while sender_index == receiver_index {
        receiver_index = rng.gen_range(0..names.len());
    }

    let sender = names[sender_index];
    let receiver = names[receiver_index];

    // Generate a random transfer amount between 0.001 and 10.0
    let amount: f64 = rng.gen_range(0.001..=10.0);

    format!("{} is transferring {:.3} BTC to {}", sender, amount, receiver)
}

/// Simulates a 51% attack
pub fn simulate_51_attack(difficulty: usize) {
    let mut original_chain = Blockchain::new(difficulty);

    // Adding 5 legitimate blocks to the original chain.
    for i in 1..=5 {
        original_chain.add_block(&format!("Transaction {}", i));
    }

    // Cloning the original chain to create the attacker's fork.
    let attacker_chain = original_chain.clone();

    // Wrapping both chains in `Arc<Mutex>` for safe access by multiple threads.
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
                    let mut chain = original_chain.lock().unwrap();
                    chain.add_block(&transaction);
                    println!("Legitimate: Mining Block {}", i);
                }
                i += 1;
                thread::sleep(time::Duration::from_secs(difficulty as u64 * difficulty as u64));
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
                    let mut attacker = attacker_chain.lock().unwrap();
                    attacker.add_block(&transaction);
                    println!("Attack: Mining Block {}", i);
                }

                // Check if the attacker's chain is longer.
                let attacker_len;
                let original_len;
                {
                    let attacker = attacker_chain.lock().unwrap();
                    let original = original_chain.lock().unwrap();
                    attacker_len = attacker.chain.len();
                    original_len = original.chain.len();
                }

                if attacker_len > original_len + 1 {
                    stop_flag.store(true, Ordering::Relaxed);
                }

                i += 1;
                thread::sleep(time::Duration::from_secs(difficulty as u64));
            }
        })
    };

    // Wait for both threads to finish.
    original_chain_thread.join().unwrap();
    attacker_chain_thread.join().unwrap();

    // Access and print both chains after mining.
    let original_chain = original_chain.lock().unwrap();
    let attacker_chain = attacker_chain.lock().unwrap();

    println!("\nOriginal Chain: {} Blocks", original_chain.chain.len());
    original_chain.is_valid();
    original_chain.print_chain();

    println!("\nAttacker Chain: {} Blocks", attacker_chain.chain.len());
    attacker_chain.is_valid();
    attacker_chain.print_chain();

    // Determine which chain is now considered valid based on length.
    if attacker_chain.chain.len() > original_chain.chain.len() {
        println!("\nThe attacker's fork has become the valid chain!");

        let mut invalidated_blocks = Vec::new();

        for i in 0..original_chain.chain.len() {
            if original_chain.chain[i] != attacker_chain.chain[i] {
                invalidated_blocks.push(&original_chain.chain[i]);
            }
        }

        if !invalidated_blocks.is_empty() {
            println!("\nInvalid blocks after the attack (not found in the attacker's chain):");
            for block in invalidated_blocks {
                println!("Transaction: {}", block.data);
            }
        }
    } else {
        println!("\nThe original chain remains valid.");
    }
}
