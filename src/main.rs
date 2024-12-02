mod blockchain;
mod block;
mod attack;

use std::env;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    let difficulty = if args.len() > 1 {
        args[1].parse::<usize>().unwrap()
    } else {
        4  // Default difficulty
    };

    attack::simulate_51_attack(difficulty);
}