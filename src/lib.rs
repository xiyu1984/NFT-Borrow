use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, BorshStorageKey};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::PanicOnDefault;
// use near_sdk::json_types::{U128};

mod metadata;
mod nft_666_token;

#[near_bindgen]
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
    AssetsOwnInfo,
    AssetsUsageInfo,
    Tokens,
    Approvals,
    AssetsOwnTable{account_hash: Vec<u8>},
    AssetsUsageTable{account_hash: Vec<u8>},
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

    // <ownership, ..<TokenID>>
    assets_own_info: LookupMap<AccountId, UnorderedSet<String>>,

    // <usage, ..<TokenID>>
    assets_usage_info: LookupMap<AccountId, UnorderedSet<String>>,

    // <TokenID, metadata>
    tokens: LookupMap<String, metadata::TokenMetaData>,

    // <TokenID, ..>
    approvals: LookupMap<String, AssetApprove>,
}

pub trait NFTBorrow{
    fn usageOf(&self, token_id: String)->AccountId;
}

pub trait NFTMetaData{
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn tokenURI(&self, token_id: String) -> String;
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
            assets_own_info: LookupMap::new(StorageRecord::AssetsOwnInfo),
            assets_usage_info: LookupMap::new(StorageRecord::AssetsUsageInfo),
            tokens: LookupMap::new(StorageRecord::Tokens),
            approvals: LookupMap::new(StorageRecord::Approvals),
        }
    }

    pub fn totalSupply(&self)-> u64{
        self.total_supply
    }

    pub fn balanceOf(&self, account_id: AccountId)-> u64{
        let v = self.assets_own_info.get(&account_id);
        if let Some(val) = v {
            val.len()
        }else{
            env::panic_str("None of the account!");
        }
    }

    pub fn ownerOf(&self, token_id: String)->AccountId{
        let v= self.owner_ship.get(&token_id);
        if let Some(val) = v {
            val.ownership
        }else{
            env::panic_str("The token_id dose not exist!");
        }
    }

    pub fn mint(&mut self, asset_rights: AssetRights, token_metadata: metadata::TokenMetaData) -> nft_666_token::NFT666Token{
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized");

        if !env::is_valid_account_id(asset_rights.ownership.as_bytes()){
            env::panic_str("invalid owner account!");
        }

        if !env::is_valid_account_id(asset_rights.usage_rights.as_bytes()){
            env::panic_str("invalid usage account!");
        }

        self.total_supply += 1;

        // create the unique token_id
        let token_id: String = token_metadata.to_hex_string();
        let token_id = token_id + env::current_account_id().as_str();
        let token_id = format!("{}+{}", token_id, self.total_supply);
        let token_id = hex::encode(env::sha256(token_id.as_bytes()));

        // the token_id must be unique!
        if self.tokens.contains_key(&token_id){
            env::panic_str("cannot create, check the metadata of the token!");
        }

        // insert tokens
        self.tokens.insert(&token_id, &token_metadata);

        // insert ownership
        self.owner_ship.insert(&token_id, &asset_rights);

        // insert assets_own_info
        let mut owned_tokens = self.assets_own_info.get(&asset_rights.ownership).unwrap_or_else(||{
            UnorderedSet::new(StorageRecord::AssetsOwnTable {
                account_hash: env::sha256(asset_rights.ownership.as_bytes()),
            })
        });
        owned_tokens.insert(&token_id);
        self.assets_own_info.insert(&asset_rights.ownership, &owned_tokens);

        // insert assets_usage_info
        let mut usage_tokens = self.assets_usage_info.get(&asset_rights.usage_rights).unwrap_or_else(||{
            UnorderedSet::new(StorageRecord::AssetsUsageTable{
                account_hash: env::sha256(asset_rights.usage_rights.as_bytes()),
            })
        });
        usage_tokens.insert(&token_id);
        self.assets_usage_info.insert(&asset_rights.usage_rights, &usage_tokens);

        // return value
        nft_666_token::NFT666Token{
            token_id,
            owner_id: asset_rights.ownership,
            usage_rights: asset_rights.usage_rights,
            metadata: Some(token_metadata),
        }
    }

    pub fn transferFrom(&mut self, from: AccountId, to: AccountId, token_id: String){
        if !env::is_valid_account_id(to.as_bytes()) {
            env::panic_str("Invalid `to` address!");
        }

        let asset_rights = self.owner_ship.get(&token_id);
        // token without approves
        if let Some(mut art) = asset_rights {
            // if in ownership, the token can only be transfered from the `predecessor_account_id`
            // that is, the `from` must be the same as the `predecessor_account_id`
            assert_eq!(env::predecessor_account_id(), from, "Unauthorized");
            // and the owner of the token must be the same as `from`
            assert_eq!(from, art.ownership, "Unauthorized");

            // update `self.assets_own_info`
            let mut cur_owned_tokens = self.assets_own_info.get(&from);
            if let Some(mut cur_o_t) = cur_owned_tokens {
                // delete from current owner
                cur_o_t.remove(&token_id);
                self.assets_own_info.insert(&from, &cur_o_t);

                // add into new owner
                let mut new_owned_tokens = self.assets_own_info.get(&to).unwrap_or_else(||{
                    UnorderedSet::new(StorageRecord::AssetsOwnTable {
                        account_hash: env::sha256(to.as_bytes()),
                    })
                });
                new_owned_tokens.insert(&token_id);
                self.assets_own_info.insert(&to, &new_owned_tokens);

            }else{
                env::panic_str("There's a bug, because someone has the ownership, but the asset is not existed in the asset_own_info table!");
            }

            // update `self.owner_ship`
            art.ownership = to;
            self.owner_ship.insert(&token_id, &art);

        }else{
            // token approved
            let asset_approve = self.approvals.get(&token_id);
            if let Some(aprv) = asset_approve {
                // The token can only be transfered from `AssetApprove::from` to `AssetApprove::to`
                assert_eq!(aprv.from, from, "`from` not equal!");
                assert_eq!(aprv.to, to, "`to` not equal!");

                // add into new owner
                let mut new_owned_tokens = self.assets_own_info.get(&to).unwrap_or_else(||{
                    UnorderedSet::new(StorageRecord::AssetsOwnTable {
                        account_hash: env::sha256(to.as_bytes()),
                    })
                });
                new_owned_tokens.insert(&token_id);
                self.assets_own_info.insert(&to, &new_owned_tokens);

                // update `self.owner_ship`
                // the usage_rights not change
                let art = AssetRights{
                    ownership: to,
                    usage_rights: aprv.usage_rights,
                };
                self.owner_ship.insert(&token_id, &art);

                // delete from approves
                self.approvals.remove(&token_id);

            }else{
                env::panic_str("Token not exist!");
            }
        }
    }

    // for test interfaces
    pub fn get_contract_meta_data(&self) -> metadata::ContractMetaData{
        self.contract_meta.clone()
    }
}

#[near_bindgen]
impl NFTBorrow for Contract{
    fn usageOf(&self, token_id: String)->AccountId{
        let v= self.owner_ship.get(&token_id);
        if let Some(val) = v {
            val.usage_rights
        }else{
            env::panic_str("The token_id dose not exist!");
        }
    }
}

#[near_bindgen]
impl NFTMetaData for Contract{
    fn name(&self) ->String{
        self.contract_meta.name.clone()
    }

    fn symbol(&self) ->String{
        self.contract_meta.symbol.clone()
    }

    fn tokenURI(&self, token_id: String)->String{
        let v = self.tokens.get(&token_id);

        if let Some(token_meta) = v {
            if let Some(media_uri) = token_meta.media {
                media_uri
            }else{
                "".to_string()
            }
        }else{
            env::panic_str("None of the token_id");
        }
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
