mod blockchain;
mod block;
mod attack;

use std::env;

fn main() {
    // Get the difficulty argument from the command-line input, with a default value of 4 if not provided
    let difficulty: usize = env::args()  // Collect command-line arguments into a vector
        .nth(1)  // Get the second argument (first argument is the program name)
        .and_then(|arg| arg.parse().ok())  // Attempt to parse the argument as a usize
        .unwrap_or(4);  // If parsing fails, default to difficulty of 4

    // Simulate the 51% attack with the specified difficulty
    attack::simulate_51_attack(difficulty);
}