
# ZKP-Polkadot-Interoperability

Prototype for a **Zero-Knowledge Proof (ZKP) based solution** to enhance secure and scalable blockchain interoperability in the **Polkadot** ecosystem.

## Project Goal

This repository builds a **Polkadot-style interoperability prototype** where:

- Zero-knowledge proofs are generated off-chain using **Circom** and **Groth16 zk-SNARKs**.
- Proofs are verified **on-chain** inside a custom **Substrate verifier pallet**.
- The verifier is wired into **cross-chain message flows**, so a destination chain only accepts a message if the attached proof is valid.
- On a Rococo-style test setup, the target is:
  - Proof verification in **under ~100 ms**, and  
  - **Small proof size**, so messages stay light and cheap.
- This reduces reliance on **trusted relayers** and moves trust back into cryptography and the chain itself.


## 📦 Project Overview

Most blockchains struggle to communicate securely due to privacy risks, weak trust models, and lack of common technical standards. This project explores how **Zero-Knowledge Proofs (ZKPs)** can help solve these problems by allowing blockchains to verify information without revealing sensitive data.

The prototype focuses on building and testing a ZKP-based model to improve secure cross-chain communication within Polkadot.

## ⚙️ Key Features

- ZKP-based verification for cross-chain message passing  
- Secure and privacy-preserving transaction validation  
- Prototype implemented using Substrate, Circom or ZoKrates  
- Simulated test environment for performance and security testing  

## 🛠️ Project Structure

```
zkp-polkadot-interoperability/
├── substrate-node/          # Substrate-based blockchain setup
├── zkp-circuits/            # ZKP circuits and proof-related files
├── scripts/                 # Helper scripts for deployment and testing
├── testnet-simulation/      # Local test environment setup
├── docs/                    # Design documents, diagrams
├── results/                 # Test results and performance benchmarks
```

## 🚀 Getting Started

1. Clone the repository:  
   `git clone https://github.com/onkabetseg/zkp-polkadot-interoperability.git`

2. Set up development tools:  
   - [Substrate](https://docs.substrate.io/)  
   - [Circom](https://docs.circom.io/) or [ZoKrates](https://zokrates.github.io/)  

3. Follow setup instructions in relevant folders (coming soon).  

## 📚 Project Status

- Project initialization complete  
- Folder structure prepared  
- Development of ZKP circuits and Substrate modules in progress  

## High-Level Flow

3.1. **Event on Source Chain**
   - A transaction happens on Chain A (for example, a balance lock or state update).

3.2. **Proof Generation (off-chain)**
   - A relayer or off-chain worker collects required data from Chain A.
   - It runs a **Circom circuit** and uses **Groth16** to generate:
     - a proof `π`
     - public inputs (e.g., state root, account, amount)

3.3. **Cross-Chain Message**
   - The relayer sends a message to Chain B containing:
     - the public inputs, and
     - the proof `π`.

3.4. **On-Chain Verification (Substrate pallet)**
   - Chain B calls the **zk-SNARK verifier pallet**.
   - The pallet checks `verify(π, public_inputs)` using the on-chain verification key.
   - If the proof is valid, the cross-chain action continues (e.g., mint, unlock, or update).

3.5. **Rococo-style Testing**
   - We run this flow on a local or Rococo-like testnet and record:
     - verification time (< ~100 ms)
     - proof size
     - overall transaction cost and throughput.
## 📄 License

This project is for academic research and follows an open-source license (to be confirmed).
