// Result 7: Runtime-compatible proof structures
//
// This file defines simplified proof structures that can later be adapted
// for Substrate/FRAME runtime storage, extrinsics and events.
//
// Design choice:
// For the current prototype, proof verification is performed off-chain.
// The on-chain pallet receives the verification result, public commitment,
// and supporting proof metadata.

#[derive(Clone, PartialEq, Eq)]
pub struct RuntimeProofMetadata {
    pub proof_hash: Vec<u8>,
    pub public_commitment: Vec<u8>,
    pub verifier_id: Vec<u8>,
    pub verification_timestamp: u64,
}

#[derive(Clone, PartialEq, Eq)]
pub enum RuntimeVerificationDecision {
    Accepted,
    Rejected,
}

#[derive(Clone, PartialEq, Eq)]
pub struct RuntimeVerificationRecord {
    pub metadata: RuntimeProofMetadata,
    pub decision: RuntimeVerificationDecision,
    pub reason: Vec<u8>,
}

pub fn create_runtime_record(
    proof_hash: Vec<u8>,
    public_commitment: Vec<u8>,
    verifier_id: Vec<u8>,
    verification_timestamp: u64,
    decision: RuntimeVerificationDecision,
    reason: Vec<u8>,
) -> Option<RuntimeVerificationRecord> {
    if proof_hash.is_empty() || public_commitment.is_empty() || verifier_id.is_empty() {
        return None;
    }

    Some(RuntimeVerificationRecord {
        metadata: RuntimeProofMetadata {
            proof_hash,
            public_commitment,
            verifier_id,
            verification_timestamp,
        },
        decision,
        reason,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_runtime_record_is_created() {
        let record = create_runtime_record(
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            123456,
            RuntimeVerificationDecision::Accepted,
            b"Verified off-chain".to_vec(),
        );

        assert!(record.is_some());
    }

    #[test]
    fn invalid_runtime_record_is_rejected() {
        let record = create_runtime_record(
            vec![],
            vec![],
            vec![],
            123456,
            RuntimeVerificationDecision::Rejected,
            b"Missing data".to_vec(),
        );

        assert!(record.is_none());
    }
}
