use crate::blockchain::Blockchain;
use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}},
    thread,
    time,
};

/// Simulates a 51% attack
pub fn simulate_51_attack() {
    let difficulty = 4;
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
                {
                    let mut chain = original_chain.lock().unwrap();
                    chain.add_block(&format!("Original Transaction {}", i));
                    println!("Legitimate: Mining Block {}", i);
                }
                thread::sleep(time::Duration::from_secs(1));
                i += 1;
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
                {
                    let mut attacker = attacker_chain.lock().unwrap();
                    attacker.add_block(&format!("Malicious Transaction {}", i));
                    println!("Attack: Mining Block {}", i);
                }
                {
                    let attacker = attacker_chain.lock().unwrap();
                    let original = original_chain.lock().unwrap();

                    // Check if the attacker's chain is longer.
                    if attacker.chain.len() > original.chain.len() + 1 {
                        stop_flag.store(true, Ordering::Relaxed);
                    }
                }
                i += 1;
            }
        })
    };

    // Wait for both threads to complete.
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
