mod block;        // Import the `block` module
mod blockchain;   // Import the `blockchain` module

fn main() {
    // Create a new blockchain with a difficulty of 4 leading zeros in the hash
    let mut blockchain = blockchain::Blockchain::new(4);

    // Add some blocks to the blockchain
    for i in 0..10 {
        blockchain.add_block(&format!("Block {} Data", i));
    }

    // Print the blockchain
    blockchain.print_chain();

    // Validate the blockchain
    if blockchain.is_valid() {
        println!("Blockchain is valid!");
    } else {
        println!("Blockchain is invalid!");
    }
}
