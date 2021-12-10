use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, BorshStorageKey};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::PanicOnDefault;

mod metadata;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetRights{
    ownership: AccountId,
    usage_rights: AccountId,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetApprove{
    from: AccountId,
    to: AccountId,
    usage_rights: AccountId,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageRecord{
    Ownership,
    AssetsInfo,
    Tokens,
    Approvals,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // SETUP CONTRACT STATE
    // owner of the contract, the deployer of thecontract. The only one has the right to mint
    owner_id: AccountId,
    // Increment for every mint
    total_supply: u64,

    contract_meta: metadata::ContractMetaData,

    // token management
    // <TokenID, ...>
    owner_ship: LookupMap<String, AssetRights>,

    // <AccountID, ..<TokenID>>
    assets_info: LookupMap<AccountId, UnorderedSet<String>>,

    // <TokenID, metadata>
    tokens: LookupMap<String, metadata::TokenMetaData>,

    // <TokenID, ..>
    approvals: LookupMap<String, AssetApprove>,
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    #[init]
    pub fn new(contract_meta: metadata::ContractMetaData) ->Self{
        Self{
            owner_id: env::predecessor_account_id(),
            total_supply: 0,
            contract_meta,
            owner_ship: LookupMap::new(StorageRecord::Ownership),
            assets_info: LookupMap::new(StorageRecord::AssetsInfo),
            tokens: LookupMap::new(StorageRecord::Tokens),
            approvals: LookupMap::new(StorageRecord::Approvals),
        }
    }

    // for test interfaces
    pub fn get_contract_meta_data(&self) -> metadata::ContractMetaData{
        self.contract_meta.clone()
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
