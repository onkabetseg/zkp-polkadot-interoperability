// Result 7: Substrate/FRAME-style verifier pallet with runtime execution model
// Research topic: Integrating ZKP to Enhance Secure Blockchain Interoperability in Polkadot
//
// This research prototype connects proof submission to a reusable Groth16 verifier adapter
// and introduces a runtime-compatible verification-result model.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod verifier {
    pub mod groth16_adapter;
}

pub mod runtime_model {
    pub mod proof_types;
    pub mod execution_model;
}

use runtime_model::proof_types::{
    create_runtime_record,
    RuntimeVerificationDecision,
    RuntimeVerificationRecord,
};

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

pub fn submit_offchain_verification_result(
    proof_hash: Vec<u8>,
    public_commitment: Vec<u8>,
    verifier_id: Vec<u8>,
    timestamp: u64,
    accepted: bool,
) -> Option<RuntimeVerificationRecord> {
    let decision = if accepted {
        RuntimeVerificationDecision::Accepted
    } else {
        RuntimeVerificationDecision::Rejected
    };

    let reason = if accepted {
        b"Off-chain Groth16 verification accepted".to_vec()
    } else {
        b"Off-chain Groth16 verification rejected".to_vec()
    };

    create_runtime_record(
        proof_hash,
        public_commitment,
        verifier_id,
        timestamp,
        decision,
        reason,
    )
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
    fn accepted_offchain_result_creates_runtime_record() {
        let record = submit_offchain_verification_result(
            vec![1, 2, 3],
            vec![4, 5, 6],
            b"rust-arkworks-verifier".to_vec(),
            123456,
            true,
        );

        assert!(record.is_some());
    }

    #[test]
    fn missing_offchain_result_data_is_rejected() {
        let record = submit_offchain_verification_result(
            vec![],
            vec![],
            vec![],
            123456,
            true,
        );

        assert!(record.is_none());
    }
}
