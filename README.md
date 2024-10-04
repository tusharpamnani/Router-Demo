# Tic-Tac-Toe Smart Contract on Router Chain

This project demonstrates a simple Tic-Tac-Toe smart contract written in Rust and deployed on the Router Chain.

## What is Router Protocol?

Router Protocol is an infrastructure layer enabling secure cross-chain communication between various blockchains. It allows developers to create interoperable decentralized applications (dApps) and smart contracts that work across multiple chains.

The Router Chain, which is part of this ecosystem, supports the deployment of smart contracts, making it a powerful tool for building decentralized applications.

## Why Rust?

Rust is a programming language known for its safety, speed, and performance. It is widely used for building secure and efficient smart contracts. In this project, the smart contract is written in Rust and compiled to WebAssembly (Wasm) to be deployed on Router Chain. Rust's strict memory safety guarantees make it ideal for blockchain development.

## Prerequisites (Installation on Windows)

Follow these steps to set up your development environment on Windows:

1. **Install WSL (Windows Subsystem for Linux):**

   First, enable WSL, which allows you to run a Linux environment on your Windows machine. To do this, open a terminal and run:

   ```bash
   wsl --install
   ```

   Restart your computer after installation if necessary.

2. **Install Rust:**

   Rust is the programming language used to write the smart contract. To install Rust, follow the instructions provided on the official website:

   - [Rust Installation Guide](https://www.rust-lang.org/tools/install)

3. **Install Cargo (Rust’s package manager):**

   Cargo is the Rust build system and package manager. You can find the installation guide for Cargo here:

   - [Cargo Installation Guide](https://doc.rust-lang.org/cargo/getting-started/installation.html)

4. **Add the WebAssembly (Wasm) target:**

   To compile the smart contract to Wasm, you'll need to add the WebAssembly target to your Rust toolchain:

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

5. **Install additional Cargo tools:**

   You'll also need some extra Cargo tools for managing and running scripts:

   - Install `cargo-generate` (useful for project generation):

     ```bash
     cargo install cargo-generate --features vendored-openssl
     ```

   - Install `cargo-run-script` (for running custom scripts in your Rust project):

     ```bash
     cargo install cargo-run-script
     ```

## Steps to Deploy Tic-Tac-Toe Smart Contract on Router Chain

1. **Clone the repository:**

   Start by cloning the repository containing the Tic-Tac-Toe smart contract.

   ```bash
   git clone https://github.com/tusharpamnani/Router-Demo.git
   ```

2. **Build the smart contract:**

   Navigate to the project directory and compile the contract to Wasm using Cargo. Make sure the target is set to `wasm32-unknown-unknown` for cross-compilation to WebAssembly.

   ```bash
   cd Router-Demo
   cargo build --target wasm32-unknown-unknown --release
   ```

3. **Deploy the contract:**

   After building the contract, you can deploy it to the Router Chain using the Router Station.

   - Go to [Router Station](https://station.routerprotocol.com/).
   - Create a new contract.
   - Select the `tic_tac_toe.wasm` file from the `target/wasm32-unknown-unknown/release/` folder.
   - Click on **Upload** and **Instantiate** the contract.

   This will deploy the contract on Router Chain.

4. **Verify deployment:**

   Once deployed, you will receive a transaction hash. You can verify the transaction by visiting [RouterScan](https://testnet.routerscan.io/transactions) and entering the transaction hash.

---

## Contract Interaction

Once deployed, you can interact with the Tic-Tac-Toe contract on Router Chain through the provided contract address. The contract allows players to make moves and determine the winner based on the game state.

---

## Additional Resources

- [Router Protocol Documentation](https://courses.routerprotocol.com/building-on-router-chain/module-1/)
- [Rust Documentation](https://doc.rust-lang.org/book/)
