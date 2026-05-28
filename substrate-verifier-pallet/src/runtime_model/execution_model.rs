// Result 7: Execution model for Substrate/Polkadot integration
//
// This file documents the selected execution model for the prototype.
//
// Selected model:
// Off-chain Groth16 verification + on-chain verification-result submission.
//
// Flow:
// 1. User provides private inputs off-chain.
// 2. Circom/SnarkJS generates proof and public commitment.
// 3. Rust arkworks verifier verifies the Groth16 proof off-chain.
// 4. If verification succeeds, a proof hash and public commitment are submitted on-chain.
// 5. The pallet stores/emits the verification result.
// 6. The verified result can later support a cross-chain/XCM interoperability action.

pub enum VerificationExecutionModel {
    FullOnChainVerification,
    OffChainVerificationWithOnChainResult,
}

pub struct ExecutionModelDecision {
    pub selected_model: VerificationExecutionModel,
    pub reason: &'static str,
}

pub fn selected_execution_model() -> ExecutionModelDecision {
    ExecutionModelDecision {
        selected_model: VerificationExecutionModel::OffChainVerificationWithOnChainResult,
        reason: "Selected for the prototype because Groth16 verification is computationally heavy for early runtime integration. Off-chain verification allows the Rust verifier to validate proofs while the pallet records the verification result on-chain.",
    }
}
