# 51% Attack

This repository contains a blockchain simulator in Rust, focusing on block creation, mining (proof-of-work), and simulating a 51% attack scenario. The code features a basic blockchain implementation that validates the integrity of its blockchain and simulates a situation where an attacker attempts to reverse transactions or gain control over the network.

## Features

1. **Block Creation**: The blockchain is composed of chained blocks, with each block referencing the previous block, forming a secure chain.
2. **Block Mining**: Mining is performed through a proof-of-work algorithm, where the miner must find a value (nonce) that causes the block's hash to start with a specific number of zeros (defined by the difficulty).
3. **Blockchain Integrity Check**: The code validates the integrity of the chain, ensuring that hashes match and that blocks have not been altered.
4. **51% Attack Simulation**: The code simulates a 51% attack, where an attacker tries to create a parallel (forked) version of the blockchain and eventually reverse transactions or take control of the network.

## Code Structure

### `block.rs` Module
Contains the definition of the `Block` structure that represents a block in the blockchain. Key functionalities include:

- **Creating a New Block** (`new`): A block is created with an index, the hash of the previous block, timestamp, transaction data, and an initial nonce value.

- **Calculating the Block's Hash** (`calculate_hash`): The block's hash is calculated based on its properties using the SHA-256 algorithm.

- **Mining** (`mine_block`): Block mining is done by adjusting the nonce until the block's hash meets the specified difficulty (number of leading zeros).

- **Block Validation** (`PartialEq`): Implements comparison of blocks based on their hash, ensuring equality between blocks.

- **Custom Debug Formatting**: The `fmt::Debug` implementation allows for the visualization of the block details, such as index, data, and hashes, in a compact form.

### `blockchain.rs` Module
Manages the creation and validation of the entire blockchain, which consists of a sequence of blocks.

- **Creating the Blockchain** (`new`): Initializes the blockchain with a genesis block (the first block) and the mining difficulty.

- **Adding Blocks** (`add_block`): Adds a new block to the blockchain, mining it before inserting it into the chain.

- **Blockchain Validation** (`is_valid`): Checks the integrity of the chain, ensuring that hashes are valid and that the referenced previous block matches the hash of the previous block.

- **Printing the Blockchain** (`print_chain`): Displays the entire chain of blocks.

### `attack.rs` Module
Simulates a 51% attack, where an attacker tries to mine their own version of the blockchain and compete with the legitimate blockchain.

- **Generating Random Transactions** (`generate_random_transaction`): Generates random transactions between two participants with a random amount (0.001 to 10.0 coins). It ensures the sender and receiver are different and returns the transaction as a formatted string.

- **Simulating a 51% Attack** (`simulate_51_attack`): Creates two parallel blockchains (legitimate and attacker). The attacker tries to surpass the legitimate blockchain in length and hash strength. If the attacker succeeds in creating a longer chain, it becomes the valid chain.

## How to Run the Code

To run the code, follow the steps below:

### 1. Install Rust

If you don’t have Rust installed yet, follow the steps below to install it:

- Go to [https://www.rust-lang.org/](https://www.rust-lang.org/) for installation instructions.

Or use the following command in the terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone the Repository

Clone this repository to your local environment:

```bash
git clone https://github.com/berwalker/51percent-attack.git
cd 51percent-attack/
```

### 3. Build the Code

> **Note:** Make sure a system C compiler (`cc`) is installed.

Build the code with the following command:

```bash
cargo build --release
```

### 4. Run the 51% Attack Simulator

Run the 51% attack simulator with the following command:

```bash
cargo run <difficulty> // default 4
```

This will start the 51% attack simulation, showing the mining of both the legitimate and attacking chains and checking which one eventually becomes the valid chain.

- `<difficulty>`: (Optional) The difficulty defines how many leading zeros must appear in the hash of each block. The default value is 4.
- On average, the time to mine a block is calculated as `2 ^ difficulty` attempts.
- For example, with a difficulty of 4, it will take about `2^4 = 16` attempts to mine a valid block.

### 5. Observing Results

The program will print the following details during execution:

- The progress of mining legitimate and malicious blocks.
```
Legitimate: Mining Block 1
Attack: Mining Block 1
Attack: Mining Block 2
Legitimate: Mining Block 2
Legitimate: Mining Block 3
Attack: Mining Block 3
Attack: Mining Block 4
Legitimate: Mining Block 4
Attack: Mining Block 5
```
- The validity status of both blockchains after the simulation.
```
Original Chain: 10 Blocks
Block 0 [Data: Genesis Block, Hash: 0000013d731a16ee, Previous Hash: 0, Timestamp: 2024-12-03T02:51:09.688]
Block 1 [Data: Transaction 1, Hash: 0000013534423f3a, Previous Hash: 0000013d731a16ee, Timestamp: 2024-12-03T02:51:21.966]
Block 2 [Data: Transaction 2, Hash: 000004b577b4994b, Previous Hash: 0000013534423f3a, Timestamp: 2024-12-03T02:51:31.198]
Block 3 [Data: Transaction 3, Hash: 000000f0a5310c24, Previous Hash: 000004b577b4994b, Timestamp: 2024-12-03T02:52:22.385]
Block 4 [Data: Transaction 4, Hash: 000003f8557f5271, Previous Hash: 000000f0a5310c24, Timestamp: 2024-12-03T02:52:31.375]
Block 5 [Data: Transaction 5, Hash: 000007704fdbc640, Previous Hash: 000003f8557f5271, Timestamp: 2024-12-03T02:52:35.969]
Block 6 [Data: Grace is transferring 1.521 Coins to Bob, Hash: 0000070167282de0, Previous Hash: 000007704fdbc640, Timestamp: 2024-12-03T02:52:53.276]
Block 7 [Data: Grace is transferring 8.430 Coins to Charlie, Hash: 000009b1e8abf043, Previous Hash: 0000070167282de0, Timestamp: 2024-12-03T02:53:28.857]
Block 8 [Data: Grace is transferring 9.166 Coins to Eve, Hash: 000009b535f1b7d1, Previous Hash: 000009b1e8abf043, Timestamp: 2024-12-03T02:54:14.459]
Block 9 [Data: Frank is transferring 6.941 Coins to Alice, Hash: 000000d0030b3adc, Previous Hash: 000009b535f1b7d1, Timestamp: 2024-12-03T02:54:57.778]

Attacker Chain: 11 Blocks
Block 0 [Data: Genesis Block, Hash: 0000013d731a16ee, Previous Hash: 0, Timestamp: 2024-12-03T02:51:09.688]
Block 1 [Data: Transaction 1, Hash: 0000013534423f3a, Previous Hash: 0000013d731a16ee, Timestamp: 2024-12-03T02:51:21.966]
Block 2 [Data: Transaction 2, Hash: 000004b577b4994b, Previous Hash: 0000013534423f3a, Timestamp: 2024-12-03T02:51:31.198]
Block 3 [Data: Transaction 3, Hash: 000000f0a5310c24, Previous Hash: 000004b577b4994b, Timestamp: 2024-12-03T02:52:22.385]
Block 4 [Data: Transaction 4, Hash: 000003f8557f5271, Previous Hash: 000000f0a5310c24, Timestamp: 2024-12-03T02:52:31.375]
Block 5 [Data: Transaction 5, Hash: 000007704fdbc640, Previous Hash: 000003f8557f5271, Timestamp: 2024-12-03T02:52:35.969]
Block 6 [Data: Frank is transferring 6.351 Coins to Charlie, Hash: 0000039d8ad17d3d, Previous Hash: 000007704fdbc640, Timestamp: 2024-12-03T02:52:53.276]
Block 7 [Data: Frank is transferring 7.093 Coins to Bob, Hash: 00000349d4a9727b, Previous Hash: 0000039d8ad17d3d, Timestamp: 2024-12-03T02:53:32.129]
Block 8 [Data: Alice is transferring 8.198 Coins to Bob, Hash: 000008c75e010201, Previous Hash: 00000349d4a9727b, Timestamp: 2024-12-03T02:53:58.459]
Block 9 [Data: Grace is transferring 5.258 Coins to Charlie, Hash: 000009741364465f, Previous Hash: 000008c75e010201, Timestamp: 2024-12-03T02:54:47.165]
Block 10 [Data: Bob is transferring 5.311 Coins to Alice, Hash: 0000092386bfbb59, Previous Hash: 000009741364465f, Timestamp: 2024-12-03T02:56:35.611]
```
- The resulting chain, with invalid blocks identified after the attack.
```
The attacker's fork has become the valid chain!

Invalid transactions after the attack (not found in the attacker's chain):
Transaction: Grace is transferring 1.521 Coins to Bob
Transaction: Grace is transferring 8.430 Coins to Charlie
Transaction: Grace is transferring 9.166 Coins to Eve
Transaction: Frank is transferring 6.941 Coins to Alice
```

## How the 51% Attack Simulation Works

The simulation runs in two threads:

- **Legitimate Mining Thread**: Simulates mining blocks by the legitimate network.
- **Attacker Mining Thread**: Simulates parallel mining of blocks by an attacker trying to gain control over the blockchain.

Both threads compete to mine blocks, and when the attacker's chain surpasses the legitimate one, the attacker's blockchain becomes the "valid" chain, invalidating the blocks from the original chain.
