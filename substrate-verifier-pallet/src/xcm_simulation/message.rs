// Result 9: XCM-style message simulation
//
// This module simulates how an accepted ZKP verification result
// can prepare and allow a Polkadot/XCM-style cross-chain message.
//
// This is not real XCM dispatch yet.
// It is a research prototype showing the decision logic.

#[derive(Clone, PartialEq, Eq)]
pub enum XcmMessageType {
    TransferAsset,
    SendInstruction,
    RequestStateAccess,
}

#[derive(Clone, PartialEq, Eq)]
pub enum XcmDispatchDecision {
    DispatchAllowed,
    DispatchBlocked,
}

#[derive(Clone, PartialEq, Eq)]
pub struct XcmStyleMessage {
    pub message_type: XcmMessageType,
    pub source_parachain: Vec<u8>,
    pub destination_parachain: Vec<u8>,
    pub payload_hash: Vec<u8>,
    pub proof_hash: Vec<u8>,
    pub public_commitment: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct XcmDispatchResult {
    pub decision: XcmDispatchDecision,
    pub reason: Vec<u8>,
}

pub fn prepare_xcm_style_message(
    message_type: XcmMessageType,
    source_parachain: Vec<u8>,
    destination_parachain: Vec<u8>,
    payload_hash: Vec<u8>,
    proof_hash: Vec<u8>,
    public_commitment: Vec<u8>,
) -> Option<XcmStyleMessage> {
    if source_parachain.is_empty()
        || destination_parachain.is_empty()
        || payload_hash.is_empty()
        || proof_hash.is_empty()
        || public_commitment.is_empty()
    {
        return None;
    }

    Some(XcmStyleMessage {
        message_type,
        source_parachain,
        destination_parachain,
        payload_hash,
        proof_hash,
        public_commitment,
    })
}

pub fn dispatch_xcm_style_message(
    verification_accepted: bool,
    message: Option<XcmStyleMessage>,
) -> XcmDispatchResult {
    if message.is_none() {
        return XcmDispatchResult {
            decision: XcmDispatchDecision::DispatchBlocked,
            reason: b"XCM-style dispatch blocked: message is incomplete.".to_vec(),
        };
    }

    if verification_accepted {
        XcmDispatchResult {
            decision: XcmDispatchDecision::DispatchAllowed,
            reason: b"XCM-style dispatch allowed: ZKP verification accepted.".to_vec(),
        }
    } else {
        XcmDispatchResult {
            decision: XcmDispatchDecision::DispatchBlocked,
            reason: b"XCM-style dispatch blocked: ZKP verification rejected.".to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepted_verification_allows_xcm_dispatch() {
        let message = prepare_xcm_style_message(
            XcmMessageType::SendInstruction,
            b"Parachain-A".to_vec(),
            b"Parachain-B".to_vec(),
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        );

        let result = dispatch_xcm_style_message(true, message);

        match result.decision {
            XcmDispatchDecision::DispatchAllowed => assert!(true),
            _ => panic!("Expected XCM-style dispatch to be allowed"),
        }
    }

    #[test]
    fn rejected_verification_blocks_xcm_dispatch() {
        let message = prepare_xcm_style_message(
            XcmMessageType::SendInstruction,
            b"Parachain-A".to_vec(),
            b"Parachain-B".to_vec(),
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        );

        let result = dispatch_xcm_style_message(false, message);

        match result.decision {
            XcmDispatchDecision::DispatchBlocked => assert!(true),
            _ => panic!("Expected XCM-style dispatch to be blocked"),
        }
    }

    #[test]
    fn incomplete_message_blocks_xcm_dispatch() {
        let message = prepare_xcm_style_message(
            XcmMessageType::SendInstruction,
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        );

        let result = dispatch_xcm_style_message(true, message);

        match result.decision {
            XcmDispatchDecision::DispatchBlocked => assert!(true),
            _ => panic!("Expected incomplete XCM-style message to be blocked"),
        }
    }
}
