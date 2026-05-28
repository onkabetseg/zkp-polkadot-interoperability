// Result 8: Simulated interoperability action
//
// This module shows how a verified ZKP result can control
// whether a cross-chain/interoperability action is allowed or blocked.
//
// In a full Polkadot implementation, this logic could later be linked
// to XCM message routing, asset transfer approval, or cross-chain state access.

#[derive(Clone, PartialEq, Eq)]
pub enum InteroperabilityActionType {
    CrossChainMessage,
    AssetTransfer,
    StateAccess,
}

#[derive(Clone, PartialEq, Eq)]
pub enum InteroperabilityDecision {
    Allowed,
    Blocked,
}

#[derive(Clone, PartialEq, Eq)]
pub struct InteroperabilityAction {
    pub action_type: InteroperabilityActionType,
    pub source_chain: Vec<u8>,
    pub target_chain: Vec<u8>,
    pub payload_hash: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct InteroperabilityActionResult {
    pub decision: InteroperabilityDecision,
    pub reason: Vec<u8>,
}

pub fn evaluate_interoperability_action(
    verification_accepted: bool,
    action: InteroperabilityAction,
) -> InteroperabilityActionResult {
    if action.source_chain.is_empty()
        || action.target_chain.is_empty()
        || action.payload_hash.is_empty()
    {
        return InteroperabilityActionResult {
            decision: InteroperabilityDecision::Blocked,
            reason: b"Action blocked: missing source chain, target chain, or payload hash.".to_vec(),
        };
    }

    if verification_accepted {
        InteroperabilityActionResult {
            decision: InteroperabilityDecision::Allowed,
            reason: b"Action allowed: ZKP verification accepted.".to_vec(),
        }
    } else {
        InteroperabilityActionResult {
            decision: InteroperabilityDecision::Blocked,
            reason: b"Action blocked: ZKP verification rejected.".to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepted_verification_allows_action() {
        let action = InteroperabilityAction {
            action_type: InteroperabilityActionType::CrossChainMessage,
            source_chain: b"Polkadot-Parachain-A".to_vec(),
            target_chain: b"Polkadot-Parachain-B".to_vec(),
            payload_hash: vec![1, 2, 3],
        };

        let result = evaluate_interoperability_action(true, action);

        match result.decision {
            InteroperabilityDecision::Allowed => assert!(true),
            _ => panic!("Expected interoperability action to be allowed"),
        }
    }

    #[test]
    fn rejected_verification_blocks_action() {
        let action = InteroperabilityAction {
            action_type: InteroperabilityActionType::CrossChainMessage,
            source_chain: b"Polkadot-Parachain-A".to_vec(),
            target_chain: b"Polkadot-Parachain-B".to_vec(),
            payload_hash: vec![1, 2, 3],
        };

        let result = evaluate_interoperability_action(false, action);

        match result.decision {
            InteroperabilityDecision::Blocked => assert!(true),
            _ => panic!("Expected interoperability action to be blocked"),
        }
    }

    #[test]
    fn incomplete_action_is_blocked() {
        let action = InteroperabilityAction {
            action_type: InteroperabilityActionType::CrossChainMessage,
            source_chain: vec![],
            target_chain: vec![],
            payload_hash: vec![],
        };

        let result = evaluate_interoperability_action(true, action);

        match result.decision {
            InteroperabilityDecision::Blocked => assert!(true),
            _ => panic!("Expected incomplete interoperability action to be blocked"),
        }
    }
}
