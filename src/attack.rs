use crate::blockchain::Blockchain;
use std::{sync::{Arc, Mutex}, thread, time};

pub fn simulate_51_attack() {
    let difficulty = 4;
    let mut original_chain = Blockchain::new(difficulty);

    // Adding 5 legitimate blocks to the original chain
    for i in 1..=5 {
        original_chain.add_block(&format!("Transaction {}", i));
    }

    // Cloning the original chain for the attacker's chain
    let attacker_chain = original_chain.clone();

    // Wrapping both chains in Arc<Mutex> for safe access by multiple threads
    let original_chain = Arc::new(Mutex::new(original_chain));
    let attacker_chain = Arc::new(Mutex::new(attacker_chain));

    // Creating threads to mine blocks simultaneously
    let original_chain_thread = thread::spawn({
        let original_chain = Arc::clone(&original_chain);
        move || {
            for i in 6..=10 {
                let mut chain = original_chain.lock().unwrap();
                chain.add_block(&format!("Original Transaction {}", i));
                println!("Legitimate: Mining Block {}", i);
                thread::sleep(time::Duration::from_secs(1)); // Simulates mining time
            }
        }
    });

    let attacker_chain_thread = thread::spawn({
        let attacker_chain = Arc::clone(&attacker_chain);
        move || {
            for i in 6..=12 {
                let mut chain = attacker_chain.lock().unwrap();
                chain.add_block(&format!("Malicious Transaction {}", i));
                println!("Attack: Mining Block {}", i);
                thread::sleep(time::Duration::from_secs(1)); // Simulates mining time
            }
        }
    });

    // Waiting for both threads to finish mining
    original_chain_thread.join().unwrap();
    attacker_chain_thread.join().unwrap();

    // Accessing and printing both chains after mining
    let original_chain = original_chain.lock().unwrap();
    let attacker_chain = attacker_chain.lock().unwrap();

    println!("\nOriginal Chain");
    original_chain.print_chain();

    println!("\nAttacker Chain");
    attacker_chain.print_chain();

    // Checking which chain is longer and thus valid
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
