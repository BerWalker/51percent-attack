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

- **Simulating a 51% Attack** (`simulate_51_attack`): Creates two parallel blockchains (legitimate and attacker) and simulates mining on both chains. The attacker tries to surpass the legitimate blockchain in length and hash strength. If the attacker succeeds in creating a longer chain, it becomes the valid chain.

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

### 5. Observing Results

The program will print the following details during execution:

- The progress of mining legitimate and malicious blocks.
```
Attack: Mining Block 1
Legitimate: Mining Block 1
Attack: Mining Block 2
Attack: Mining Block 3
```
- The validity status of both blockchains after the simulation.
```
Original Chain: 7 Blocks
Blockchain is valid: All blocks are correctly linked and meet the difficulty requirements.
Block 0 [Data: Genesis Block, Hash: 000050bea341966f, Previous Hash: 0, Timestamp: 2024-12-02T04:46:40.143]
Block 1 [Data: Transaction 1, Hash: 0000b865ae269a2e, Previous Hash: 000050bea341966f, Timestamp: 2024-12-02T04:46:40.804]
Block 2 [Data: Transaction 2, Hash: 0000f4f6928be726, Previous Hash: 0000b865ae269a2e, Timestamp: 2024-12-02T04:46:41.496]
Block 3 [Data: Transaction 3, Hash: 00002a374863fdcd, Previous Hash: 0000f4f6928be726, Timestamp: 2024-12-02T04:46:41.608]
Block 4 [Data: Transaction 4, Hash: 00001032b4bb5715, Previous Hash: 00002a374863fdcd, Timestamp: 2024-12-02T04:46:42.031]
Block 5 [Data: Transaction 5, Hash: 000066660d103243, Previous Hash: 00001032b4bb5715, Timestamp: 2024-12-02T04:46:47.166]
Block 6 [Data: Dave is transferring 2.808 BTC to Eve, Hash: 0000fb9e423b0dcc, Previous Hash: 000066660d103243, Timestamp: 2024-12-02T04:46:47.337]

Attacker Chain: 9 Blocks
Blockchain is valid: All blocks are correctly linked and meet the difficulty requirements.
Block 0 [Data: Genesis Block, Hash: 000050bea341966f, Previous Hash: 0, Timestamp: 2024-12-02T04:46:40.143]
Block 1 [Data: Transaction 1, Hash: 0000b865ae269a2e, Previous Hash: 000050bea341966f, Timestamp: 2024-12-02T04:46:40.804]
Block 2 [Data: Transaction 2, Hash: 0000f4f6928be726, Previous Hash: 0000b865ae269a2e, Timestamp: 2024-12-02T04:46:41.496]
Block 3 [Data: Transaction 3, Hash: 00002a374863fdcd, Previous Hash: 0000f4f6928be726, Timestamp: 2024-12-02T04:46:41.608]
Block 4 [Data: Transaction 4, Hash: 00001032b4bb5715, Previous Hash: 00002a374863fdcd, Timestamp: 2024-12-02T04:46:42.031]
Block 5 [Data: Transaction 5, Hash: 000066660d103243, Previous Hash: 00001032b4bb5715, Timestamp: 2024-12-02T04:46:47.166]
Block 6 [Data: Bob is transferring 9.149 BTC to Charlie, Hash: 00003ec0021fae9d, Previous Hash: 000066660d103243, Timestamp: 2024-12-02T04:46:47.337]
Block 7 [Data: Charlie is transferring 3.193 BTC to Dave, Hash: 0000340811e6d402, Previous Hash: 00003ec0021fae9d, Timestamp: 2024-12-02T04:46:51.971]
Block 8 [Data: Grace is transferring 1.775 BTC to Dave, Hash: 000028b5fda4afc1, Previous Hash: 0000340811e6d402, Timestamp: 2024-12-02T04:46:58.216]
```
- The resulting chain, with invalid blocks identified after the attack.
```
The attacker's fork has become the valid chain!

Invalid blocks after the attack (not found in the attacker's chain):
Transaction: Dave is transferring 2.808 BTC to Eve
```

## How the 51% Attack Simulation Works

The simulation runs in two threads:

- **Legitimate Mining Thread**: Simulates mining blocks by the legitimate network.
- **Attacker Mining Thread**: Simulates parallel mining of blocks by an attacker trying to gain control over the blockchain.

Both threads compete to mine blocks, and when the attacker's chain surpasses the legitimate one, the attacker's blockchain becomes the "valid" chain, invalidating the blocks from the original chain.
