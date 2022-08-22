use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

use scale_info::TypeInfo;

/// A unit of a PBC-recored message that wraps the actual data.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode, TypeInfo)]
pub struct MessageDeliveryRecord {
    pub chain: String,
    pub message: DeliverableMessage,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode, TypeInfo)]
pub enum DeliverableMessage {
    FungibleTokenTransfer(FungibleTokenTransfer),
    NonFungibleTokenTransfer(NonFungibleTokenTransfer),
    Custom(Custom),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode, TypeInfo)]
pub struct FungibleTokenTransfer {
    pub token_id: String,
    pub amount: u128,
    pub receiver_address: String,
    pub contract_sequence: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode, TypeInfo)]
pub struct NonFungibleTokenTransfer {
    pub collection_address: String,
    pub token_index: String,
    pub receiver_address: String,
    pub contract_sequence: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode, TypeInfo)]
pub struct Custom {
    pub message: String,
    pub contract_sequence: u64,
}