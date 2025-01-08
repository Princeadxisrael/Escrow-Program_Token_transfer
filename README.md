# Escrow Program for Token Transfer

This repository contains a Solana-based escrow program written in Rust. The program facilitates secure token transfers between a sender and a receiver by holding tokens in escrow until predefined conditions are met.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Directory Structure](#directory-structure)
- [Installation](#installation)
- [Usage](#usage)
- [Program Instructions](#program-instructions)
- [Error Handling](#error-handling)
- [Contributing](#contributing)
- [License](#license)

## Overview

The Escrow Program is a smart contract deployed on the Solana blockchain. It allows a sender to deposit tokens into an escrow account, which are then released to a designated receiver upon fulfillment of specific conditions. This mechanism ensures trustless transactions between parties.

## Features

- **Secure Escrow**: Tokens are securely held in an escrow account until conditions are met.
- **Conditional Release**: Tokens are released to the receiver only when predefined conditions are satisfied.
- **State Management**: The program maintains the state of each escrow transaction on-chain.

## Directory Structure
programs/escrow_vault/src/ ├── instruction.rs # Defines program instructions ├── lib.rs # Program entry point and module organization ├── processor.rs # Processes incoming instructions ├── state.rs # Defines state structures for escrow accounts └── error.rs # Custom error definitions

### Key Components

- **`lib.rs`**: Entry point for the program, organizing modules and defining program logic.
- **`instruction.rs`**: Defines the instructions that the program can process, such as initializing an escrow or releasing funds.
- **`processor.rs`**: Contains the logic to process each instruction and manage state transitions.
- **`state.rs`**: Defines the data structures representing the state of escrow accounts.
- **`error.rs`**: Handles custom error definitions to provide informative feedback.

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Princeadxisrael/Escrow-Program_Token_transfer.git
   cd Escrow-Program_Token_transfer/programs/escrow_vault
    ```
2. Install the Rust toolchain:
  bash ```  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup install stable
  rustup default stable
  ```
3. Install Solana CLI tools:
  bash ```
  sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
  ```
4. Build the program:

bash ```
cargo build-bpf ```

Usage
Deploy the program to the Solana blockchain:

bash ```
solana program deploy ./target/deploy/escrow_vault.so ```
Interact with the program:

### Initialize Escrow: 
- Create an escrow account and deposit tokens.
- Release Funds: Release tokens to the receiver upon fulfillment of conditions.
- Cancel Escrow: Cancel the escrow and return tokens to the sender if conditions are not met.
- Interaction can be performed using Solana CLI commands or through a frontend interface that communicates with the deployed program.

## Program Instructions
### 1. Initialize Escrow
Sets up a new escrow account with the following parameters:

- Sender: The account initiating the escrow.
- Receiver: The designated recipient of the tokens.
- Token Amount: The number of tokens to be held in escrow.
- Conditions: Specific conditions that must be met for the release of tokens.
### 2. Release Funds
Transfers the escrowed tokens to the receiver when the predefined conditions are satisfied.

### 3. Cancel Escrow
Allows the sender to cancel the escrow and retrieve the tokens if the conditions are not met within a specified timeframe.

## Error Handling
The program uses custom error definitions in error.rs to provide meaningful feedback. Common errors include:

- Invalid Instruction: The instruction provided is not recognized.
- Unauthorized Access: An account without the necessary permissions attempted an action.
- Condition Not Met: Attempted to release funds before conditions were fulfilled.
  
### Contributing
Contributions are welcome! To contribute:

Fork the repository.
Create a feature branch.
Commit your changes.
Submit a pull request.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

Feel free to reach out with questions, feedback, or issues regarding the Escrow Program. Happy coding!



