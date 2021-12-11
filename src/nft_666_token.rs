use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
// use std::collections::HashMap;

use crate::metadata;

/// In this implementation, the Token struct takes two extensions standards (metadata and approval) as optional fields, as they are frequently used in modern NFTs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct NFT666Token {
    pub token_id: String,
    pub owner_id: AccountId,
    pub usage_rights: AccountId,
    pub metadata: Option<metadata::TokenMetaData>,
    // pub approved_account_ids: Option<HashMap<AccountId, u64>>,
}