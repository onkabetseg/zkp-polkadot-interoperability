# ZKP Polkadot Interoperability

A research prototype for **privacy-preserving cross-chain verification** using **Zero-Knowledge Proofs (ZKPs)** in **Polkadot/Substrate environments**.

This project supports the paper:

> **A Framework for Privacy-Preserving Cross-Chain Verification Using Zero-Knowledge Proofs in Polkadot/Substrate Environments**

## Overview

Blockchain interoperability allows different chains to exchange messages, assets, and transaction information. However, cross-chain verification can expose sensitive information such as balances, transfer amounts, identities, or business transaction details.

This project explores how **Zero-Knowledge Proofs** can be used to verify cross-chain transaction validity without revealing private data. The proposed framework combines:

- **Circom** for ZKP circuit design
- **SnarkJS** for Groth16 proof generation
- **Substrate FRAME pallet** for on-chain proof verification
- **Polkadot/XCM-style messaging** for future cross-chain communication between parachains

The main idea is simple: private data stays off-chain, while only the proof and public signals are submitted on-chain for verification.

## Project Goal

The goal is to design and prototype a Polkadot/Substrate-based framework where:

1. Private transaction data is processed off-chain.
2. A Groth16 zk-SNARK proof is generated using Circom and SnarkJS.
3. The proof is submitted to a custom Substrate verifier pallet.
4. The pallet verifies the proof on-chain.
5. A valid proof can support future XCM-based cross-chain actions.

This reduces the need to expose private transaction data during cross-chain verification.

## Key Features

- Privacy-preserving cross-chain transaction verification
- Off-chain proof generation using Circom and SnarkJS
- On-chain proof checking using a Substrate FRAME verifier pallet
- Separation of private computation and public blockchain verification
- XCM-ready design for future parachain interoperability
- Synthetic financial transaction data for safe testing

## High-Level Workflow

1. **Source-chain event**
   - A transaction or state update happens on the source chain.

2. **Private proof generation**
   - Private data such as balances, transfer amounts, or nullifiers stays off-chain.
   - Circom and SnarkJS generate a Groth16 proof.

3. **Proof submission**
   - The proof and public inputs are submitted to a Substrate verifier pallet.

4. **On-chain verification**
   - The verifier pallet checks the proof against the stored verification key.
   - If valid, the pallet emits a successful verification event.

5. **Cross-chain action**
   - A verified result can be used to support an XCM-linked action between parachains.

## Project Structure

```text
zkp-polkadot-interoperability/
├── substrate-node/          # Substrate node and runtime setup
├── zkp-circuits/            # Circom circuits and ZKP artifacts
├── scripts/                 # Build, proof generation, and test scripts
├── testnet-simulation/      # Local testnet or Rococo-style simulation setup
├── docs/                    # Architecture diagrams and design notes
├── results/                 # Benchmark results and evaluation outputs
└── README.md                # Project documentation
```

## Main Components

| Component | Purpose |
|---|---|
| Private input data | Sensitive transaction details kept off-chain |
| Circom circuit | Defines the rule that must be proven |
| Witness generator | Builds witness values from private and public inputs |
| Trusted setup | Produces proving and verification keys for Groth16 |
| SnarkJS prover | Generates the zk-SNARK proof off-chain |
| Off-chain converter | Formats proof data for Substrate submission |
| Verifier pallet | Verifies the proof on-chain and emits result events |
| XCM layer | Supports future cross-chain actions after verification |

## Suggested Software Stack

| Software | Purpose |
|---|---|
| Rust | Runtime and pallet development |
| Polkadot SDK / Substrate | Blockchain runtime and node framework |
| Circom | ZKP circuit design and compilation |
| SnarkJS | Groth16 proof generation and local verification |
| Node.js and npm | Script execution and JavaScript dependency management |

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/onkabetseg/zkp-polkadot-interoperability.git
cd zkp-polkadot-interoperability
```

### 2. Install required tools

Install the following tools before running the prototype:

- Rust and Cargo
- Polkadot SDK / Substrate dependencies
- Node.js and npm
- Circom
- SnarkJS

### 3. Build ZKP circuits

Circuit files will be placed under:

```text
zkp-circuits/
```

The expected flow is:

```bash
circom circuit.circom --r1cs --wasm --sym
snarkjs groth16 setup circuit.r1cs pot.ptau circuit_0000.zkey
snarkjs groth16 prove circuit_final.zkey witness.wtns proof.json public.json
snarkjs groth16 verify verification_key.json public.json proof.json
```

### 4. Submit proof to Substrate

After proof generation, an off-chain converter will format the proof for the verifier pallet. The pallet will verify the proof and emit either a success or rejection event.

## Research Evaluation

The framework will be evaluated using:

- Proof generation time
- Local verification time
- On-chain verification feasibility
- Proof size
- Transaction inclusion latency
- XCM-linked workflow feasibility
- Valid proof acceptance
- Invalid or tampered proof rejection

The current paper uses analytical feasibility assessment based on published benchmarks and Polkadot cross-chain latency studies. Prototype testing and full implementation results will be added after development and evaluation.

## Current Status

- [x] Repository structure created
- [x] Research framework defined
- [x] Paper draft prepared
- [ ] Circom circuits implementation
- [ ] Groth16 proof generation pipeline
- [ ] Substrate verifier pallet implementation
- [ ] Proof converter for Substrate submission
- [ ] XCM/custom message integration
- [ ] Testnet simulation and benchmarking
- [ ] Results documentation

## Roadmap

1. Implement simple balance-check and commitment-check circuits.
2. Generate and verify Groth16 proofs using Circom and SnarkJS.
3. Build a Substrate FRAME verifier pallet.
4. Connect proof verification result to a cross-chain workflow.
5. Run local testnet simulation.
6. Record performance results.
7. Update the research paper with implementation and evaluation findings.

## Academic Use

This repository is part of academic research on privacy-preserving blockchain interoperability. It is intended for research, experimentation, and prototype development.

## License

License to be confirmed.
