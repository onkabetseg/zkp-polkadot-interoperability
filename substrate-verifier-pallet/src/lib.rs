// Result 9: Substrate/FRAME-style verifier pallet with XCM-style message simulation
// Research topic: Integrating ZKP to Enhance Secure Blockchain Interoperability in Polkadot
//
// This research prototype connects proof verification results to:
// 1. runtime verification records,
// 2. simulated interoperability decisions,
// 3. XCM-style message dispatch decisions.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod verifier {
    pub mod groth16_adapter;
}

pub mod runtime_model {
    pub mod proof_types;
    pub mod execution_model;
}

pub mod interoperability {
    pub mod action;
}

pub mod xcm_simulation {
    pub mod message;
}

use interoperability::action::{
    evaluate_interoperability_action,
    InteroperabilityAction,
    InteroperabilityActionResult,
    InteroperabilityActionType,
};

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

use xcm_simulation::message::{
    dispatch_xcm_style_message,
    prepare_xcm_style_message,
    XcmDispatchResult,
    XcmMessageType,
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

pub fn simulate_cross_chain_action_after_verification(
    verification_accepted: bool,
    source_chain: Vec<u8>,
    target_chain: Vec<u8>,
    payload_hash: Vec<u8>,
) -> InteroperabilityActionResult {
    let action = InteroperabilityAction {
        action_type: InteroperabilityActionType::CrossChainMessage,
        source_chain,
        target_chain,
        payload_hash,
    };

    evaluate_interoperability_action(verification_accepted, action)
}

pub fn simulate_xcm_dispatch_after_verification(
    verification_accepted: bool,
    source_parachain: Vec<u8>,
    destination_parachain: Vec<u8>,
    payload_hash: Vec<u8>,
    proof_hash: Vec<u8>,
    public_commitment: Vec<u8>,
) -> XcmDispatchResult {
    let message = prepare_xcm_style_message(
        XcmMessageType::SendInstruction,
        source_parachain,
        destination_parachain,
        payload_hash,
        proof_hash,
        public_commitment,
    );

    dispatch_xcm_style_message(verification_accepted, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use interoperability::action::InteroperabilityDecision;
    use xcm_simulation::message::XcmDispatchDecision;

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
    fn accepted_verification_allows_cross_chain_action() {
        let result = simulate_cross_chain_action_after_verification(
            true,
            b"Parachain-A".to_vec(),
            b"Parachain-B".to_vec(),
            vec![1, 2, 3],
        );

        match result.decision {
            InteroperabilityDecision::Allowed => assert!(true),
            _ => panic!("Expected cross-chain action to be allowed"),
        }
    }

    #[test]
    fn rejected_verification_blocks_cross_chain_action() {
        let result = simulate_cross_chain_action_after_verification(
            false,
            b"Parachain-A".to_vec(),
            b"Parachain-B".to_vec(),
            vec![1, 2, 3],
        );

        match result.decision {
            InteroperabilityDecision::Blocked => assert!(true),
            _ => panic!("Expected cross-chain action to be blocked"),
        }
    }

    #[test]
    fn accepted_verification_allows_xcm_dispatch() {
        let result = simulate_xcm_dispatch_after_verification(
            true,
            b"Parachain-A".to_vec(),
            b"Parachain-B".to_vec(),
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        );

        match result.decision {
            XcmDispatchDecision::DispatchAllowed => assert!(true),
            _ => panic!("Expected XCM-style dispatch to be allowed"),
        }
    }

    #[test]
    fn rejected_verification_blocks_xcm_dispatch() {
        let result = simulate_xcm_dispatch_after_verification(
            false,
            b"Parachain-A".to_vec(),
            b"Parachain-B".to_vec(),
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        );

        match result.decision {
            XcmDispatchDecision::DispatchBlocked => assert!(true),
            _ => panic!("Expected XCM-style dispatch to be blocked"),
        }
    }

    #[test]
    fn incomplete_xcm_data_blocks_dispatch() {
        let result = simulate_xcm_dispatch_after_verification(
            true,
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        );

        match result.decision {
            XcmDispatchDecision::DispatchBlocked => assert!(true),
            _ => panic!("Expected incomplete XCM-style dispatch to be blocked"),
        }
    }
}
