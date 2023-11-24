# Bytecode Deployer

The Bytecode Deployer is a versatile tool designed to transform a running bytecode into creation bytecode suitable for deployment on an Ethereum Virtual Machine (EVM) blockchain.

## Usage

### Prerequisites

To use this tool, ensure that you have Rust and Cargo installed on your system. If not, you can install them by following the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Build the Project

Execute the following command to build the project:

```bash
cargo build
```

### Run the Tool

Use the following command to run the Bytecode Deployer:

```bash
cargo run -- --bytecode <BYTECODE>
```

Replace <BYTECODE> with a valid hexadecimal string containing characters from 0 to F, ensuring it has an even length. Do not include the 0x prefix

### Example 

```bash
cargo run -- --bytecode 2a3b1f8c...
```