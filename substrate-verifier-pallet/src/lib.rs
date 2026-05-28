// Result 6: Substrate/FRAME-style verifier pallet connected to Groth16 adapter
// Research topic: Integrating ZKP to Enhance Secure Blockchain Interoperability in Polkadot
//
// This research prototype connects the proof submission flow to a reusable
// Groth16 verifier adapter. The adapter prepares Result 5 Rust verification
// logic for later Substrate/FRAME runtime integration.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod verifier {
    pub mod groth16_adapter;
}

use verifier::groth16_adapter::{
    verify_groth16_proof,
    Groth16ProofData,
    Groth16VerificationOutcome,
};

pub struct ZkpProofSubmission {
    pub proof: Vec<u8>,
    pub public_inputs: Vec<u8>,
    pub verification_key: Vec<u8>,
    pub commitment_hash: Vec<u8>,
}

pub enum VerificationStatus {
    Pending,
    Accepted,
    Rejected,
}

pub struct VerificationResult {
    pub status: VerificationStatus,
    pub message: &'static str,
}

pub fn submit_proof_for_verification(
    proof: Vec<u8>,
    public_inputs: Vec<u8>,
    verification_key: Vec<u8>,
    commitment_hash: Vec<u8>,
) -> VerificationResult {
    if commitment_hash.is_empty() {
        return VerificationResult {
            status: VerificationStatus::Rejected,
            message: "Proof rejected: missing commitment hash.",
        };
    }

    let proof_data = Groth16ProofData {
        proof,
        public_inputs,
        verification_key,
    };

    match verify_groth16_proof(proof_data) {
        Groth16VerificationOutcome::Verified => VerificationResult {
            status: VerificationStatus::Accepted,
            message: "Proof accepted by Groth16 verifier adapter.",
        },
        Groth16VerificationOutcome::Failed => VerificationResult {
            status: VerificationStatus::Rejected,
            message: "Proof rejected by Groth16 verifier adapter.",
        },
        Groth16VerificationOutcome::InvalidInput => VerificationResult {
            status: VerificationStatus::Rejected,
            message: "Proof rejected: invalid or missing Groth16 proof data.",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_submission_is_accepted() {
        let result = submit_proof_for_verification(
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        );

        match result.status {
            VerificationStatus::Accepted => assert!(true),
            _ => panic!("Expected complete proof submission to be accepted"),
        }
    }

    #[test]
    fn missing_commitment_hash_is_rejected() {
        let result = submit_proof_for_verification(
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![],
        );

        match result.status {
            VerificationStatus::Rejected => assert!(true),
            _ => panic!("Expected missing commitment hash to be rejected"),
        }
    }

    #[test]
    fn missing_proof_data_is_rejected() {
        let result = submit_proof_for_verification(
            vec![],
            vec![],
            vec![],
            vec![10, 11, 12],
        );

        match result.status {
            VerificationStatus::Rejected => assert!(true),
            _ => panic!("Expected missing proof data to be rejected"),
        }
    }
}
