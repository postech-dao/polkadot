
use borsh::{BorshDeserialize, BorshSerialize};
use pdao_beacon_chain_common::message;
use serde::{Deserialize, Serialize};

/// TODO: replace this with the proper type.
pub type Header = String;
/// TODO: replace this with the proper type.
pub type BlockFinalizationProof = String;
/// TODO: replace this with the proper type.
pub type MerkleProof = String;

/// A light client.
///
/// NOTE: this is a dummy implementation.
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug)]
pub struct LightClient {
    pub height: u64,
    pub last_header: Header,
    pub chain_name: String,
}

impl LightClient {
    /// Intializes a new light client with the initial header.
    pub fn new(initial_header: Header, chain_name: String) -> Self {
        Self {
            height: 0,
            last_header: initial_header,
            chain_name,
        }
    }

    /// Updates the header by providing the next block and the proof of it. Returns `true` if succeeded.
    pub fn update(&mut self, header: Header, proof: BlockFinalizationProof) -> bool {
        if proof.as_str() == "valid" {
            self.height += 1;
            self.last_header = header;
            true
        } else {
            false
        }
    }

    /// Verifies the given data with its proof.
    ///
    /// The data is opaque here, because all requests for verification are from other contracts,
    /// (remind that the light client contract is standalone)
    /// so the communication between the contracts would be a binary packet exchange (not a Rust code-level invocation).
    pub fn verify_commitment(
        &self,
        message: message::DeliverableMessage,
        block_height: u64,
        proof: MerkleProof,
    ) -> bool {
        let _record = message::MessageDeliveryRecord {
            chain: self.chain_name.clone(),
            message,
        };
        proof.as_str() == "valid" && block_height == self.height
    }
}