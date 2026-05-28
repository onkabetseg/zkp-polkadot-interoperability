use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;

fn read_json(path: &str) -> Result<Value> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))?;

    let json: Value = serde_json::from_str(&data)
        .with_context(|| format!("Failed to parse JSON file: {}", path))?;

    Ok(json)
}

fn main() -> Result<()> {
    let proof_path = "../zkp-circuits/proof_commitment.json";
    let public_path = "../zkp-circuits/public_commitment.json";
    let vk_path = "../zkp-circuits/verification_key_commitment.json";

    println!("===== Result 4: Real Groth16 Verifier Preparation =====");

    let proof = read_json(proof_path)?;
    let public_inputs = read_json(public_path)?;
    let verification_key = read_json(vk_path)?;

    println!("Proof file loaded: {}", proof_path);
    println!("Public input file loaded: {}", public_path);
    println!("Verification key file loaded: {}", vk_path);

    println!("\n===== Proof JSON keys =====");
    if let Some(obj) = proof.as_object() {
        for key in obj.keys() {
            println!("proof key: {}", key);
        }
    }

    println!("\n===== Public inputs =====");
    println!("{}", public_inputs);

    println!("\n===== Verification key JSON keys =====");
    if let Some(obj) = verification_key.as_object() {
        for key in obj.keys() {
            println!("vk key: {}", key);
        }
    }

    println!("\nStatus: Groth16 proof artifacts are available for Rust/Substrate verifier conversion.");

    Ok(())
}
