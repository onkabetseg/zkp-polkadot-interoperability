use anyhow::{anyhow, Context, Result};
use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G2Affine};
use ark_groth16::{prepare_verifying_key, Groth16, Proof, VerifyingKey};
use serde_json::Value;
use std::fs;
use std::str::FromStr;

fn read_json(path: &str) -> Result<Value> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path))?;

    let json: Value = serde_json::from_str(&data)
        .with_context(|| format!("Failed to parse JSON file: {}", path))?;

    Ok(json)
}

fn parse_fq(value: &Value) -> Result<Fq> {
    let s = value
        .as_str()
        .ok_or_else(|| anyhow!("Expected field element as string"))?;

    Fq::from_str(s).map_err(|_| anyhow!("Failed to parse Fq value: {}", s))
}

fn parse_fr(value: &Value) -> Result<Fr> {
    let s = value
        .as_str()
        .ok_or_else(|| anyhow!("Expected public input as string"))?;

    Fr::from_str(s).map_err(|_| anyhow!("Failed to parse Fr value: {}", s))
}

fn parse_g1(value: &Value) -> Result<G1Affine> {
    let arr = value
        .as_array()
        .ok_or_else(|| anyhow!("Expected G1 point as array"))?;

    let x = parse_fq(&arr[0])?;
    let y = parse_fq(&arr[1])?;

    Ok(G1Affine::new_unchecked(x, y))
}

fn parse_g2(value: &Value, swap_fq2: bool) -> Result<G2Affine> {
    let arr = value
        .as_array()
        .ok_or_else(|| anyhow!("Expected G2 point as array"))?;

    let x_arr = arr[0]
        .as_array()
        .ok_or_else(|| anyhow!("Expected G2 x as array"))?;

    let y_arr = arr[1]
        .as_array()
        .ok_or_else(|| anyhow!("Expected G2 y as array"))?;

    let x0 = parse_fq(&x_arr[0])?;
    let x1 = parse_fq(&x_arr[1])?;
    let y0 = parse_fq(&y_arr[0])?;
    let y1 = parse_fq(&y_arr[1])?;

    let x = if swap_fq2 {
        Fq2::new(x1, x0)
    } else {
        Fq2::new(x0, x1)
    };

    let y = if swap_fq2 {
        Fq2::new(y1, y0)
    } else {
        Fq2::new(y0, y1)
    };

    Ok(G2Affine::new_unchecked(x, y))
}

fn build_proof(proof_json: &Value, swap_fq2: bool) -> Result<Proof<Bn254>> {
    let a = parse_g1(&proof_json["pi_a"])?;
    let b = parse_g2(&proof_json["pi_b"], swap_fq2)?;
    let c = parse_g1(&proof_json["pi_c"])?;

    Ok(Proof { a, b, c })
}

fn build_vk(vk_json: &Value, swap_fq2: bool) -> Result<VerifyingKey<Bn254>> {
    let alpha_g1 = parse_g1(&vk_json["vk_alpha_1"])?;
    let beta_g2 = parse_g2(&vk_json["vk_beta_2"], swap_fq2)?;
    let gamma_g2 = parse_g2(&vk_json["vk_gamma_2"], swap_fq2)?;
    let delta_g2 = parse_g2(&vk_json["vk_delta_2"], swap_fq2)?;

    let ic_array = vk_json["IC"]
        .as_array()
        .ok_or_else(|| anyhow!("Expected IC as array"))?;

    let mut gamma_abc_g1 = Vec::new();

    for point in ic_array {
        gamma_abc_g1.push(parse_g1(point)?);
    }

    Ok(VerifyingKey {
        alpha_g1,
        beta_g2,
        gamma_g2,
        delta_g2,
        gamma_abc_g1,
    })
}

fn build_public_inputs(public_json: &Value) -> Result<Vec<Fr>> {
    let arr = public_json
        .as_array()
        .ok_or_else(|| anyhow!("Expected public inputs as array"))?;

    let mut inputs = Vec::new();

    for value in arr {
        inputs.push(parse_fr(value)?);
    }

    Ok(inputs)
}

fn try_verify(
    proof_json: &Value,
    public_json: &Value,
    vk_json: &Value,
    swap_fq2: bool,
) -> Result<bool> {
    let proof = build_proof(proof_json, swap_fq2)?;
    let public_inputs = build_public_inputs(public_json)?;
    let vk = build_vk(vk_json, swap_fq2)?;

    let pvk = prepare_verifying_key(&vk);

    let verified = Groth16::<Bn254>::verify_proof(
        &pvk,
        &proof,
        &public_inputs,
    )?;

    Ok(verified)
}

fn main() -> Result<()> {
    let proof_path = "../zkp-circuits/proof_commitment.json";
    let public_path = "../zkp-circuits/public_commitment.json";
    let vk_path = "../zkp-circuits/verification_key_commitment.json";

    println!("===== Result 5: Full Rust Groth16 Verification =====");

    let proof_json = read_json(proof_path)?;
    let public_json = read_json(public_path)?;
    let vk_json = read_json(vk_path)?;

    println!("Proof loaded: {}", proof_path);
    println!("Public inputs loaded: {}", public_path);
    println!("Verification key loaded: {}", vk_path);

    println!("\nPublic input:");
    println!("{}", public_json);

    println!("\nTrying verification using normal Fq2 order...");
    match try_verify(&proof_json, &public_json, &vk_json, false) {
        Ok(true) => {
            println!("SUCCESS: Groth16 proof verified in Rust using arkworks BN254.");
            return Ok(());
        }
        Ok(false) => {
            println!("Normal Fq2 order failed.");
        }
        Err(e) => {
            println!("Normal Fq2 order error: {:?}", e);
        }
    }

    println!("\nTrying verification using swapped Fq2 order...");
    match try_verify(&proof_json, &public_json, &vk_json, true) {
        Ok(true) => {
            println!("SUCCESS: Groth16 proof verified in Rust using swapped Fq2 order.");
            return Ok(());
        }
        Ok(false) => {
            println!("Swapped Fq2 order failed.");
        }
        Err(e) => {
            println!("Swapped Fq2 order error: {:?}", e);
        }
    }

    Err(anyhow!(
        "Groth16 verification failed using both normal and swapped Fq2 order."
    ))
}
