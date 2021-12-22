use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, BorshStorageKey, Gas, Promise, PromiseResult, log};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::PanicOnDefault;
use near_sdk::json_types::{Base64VecU8};
// use near_sdk::json_types::{U128};

mod metadata;
mod nft_666_token;

const GAS_FOR_FUNCTION_CALL: Gas = Gas(5_000_000_000_000);
const GAS_FOR_CALLBACK: Gas = Gas(5_000_000_000_000);

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetRights{
    ownership: AccountId,
    usage_rights: AccountId,
}

// #[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct AssetApprove{
//     from: AccountId,
//     to: AccountId,
//     usage_rights: AccountId,
// }

#[derive(PartialEq)]
enum Authority{
    From,
    Approved,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageRecord{
    Ownership,
    AssetsOwnInfo,
    AssetsUsageInfo,
    Tokens,
    Approvals,
    UsageApprovals,
    AssetsOwnTable{account_hash: Vec<u8>},
    AssetsUsageTable{account_hash: Vec<u8>},
    LeasingPeriod,
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
    approvals: LookupMap<String, AccountId>,

    // <TokenID, ..>
    usage_approvals: LookupMap<String, AccountId>,

    // <TokenID, u64(the time up block_height)>
    leasing_period: LookupMap<String, u64>,
}

pub trait NFTBorrow{
    fn usageOf(&self, token_id: String)->AccountId;
}

pub trait NFTMetaData{
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn tokenURI(&self, token_id: String) -> String;
}

// For NFT Usage
pub trait NFTUsage{
    /// @notice Transfer the usage right to someone else, 
    /// the `msg.sender` must have the usage right of the token. 
    /// If you are the owner of the token, use `lend_usage_to` or you may be not able to get the usage back!
    /// @dev 
    /// @param usage_id The address of the next user
    /// @param token_id The ID of the token
    fn transferUsageFrom(&mut self, from: AccountId, to: AccountId, token_id: String);

    fn approveUsage(&mut self, approved: AccountId, token_id: String);

    fn getUsageApproved(&self, token_id: String) ->AccountId;

    fn transfer_usage_without_check(&mut self, from: AccountId, to: AccountId, token_id: String);

    /// @notice The owner lend his token to someone else, the `predecessor_account_id` must be the token owner.
    /// The `usage_rights` must belong to the owner.
    /// @dev 
    /// @param to The address of the next user
    /// @param token_id The ID of the token
    /// @param period The rental period of the leasing. The owner can `get_usage_back` after `env::block_height() + period`
    fn lend_usage_to(&mut self, to: AccountId, token_id: String, period: u64);

    /// @notice The borrower returns the token to the owner, the `predecessor_account_id` must be the token user.
    /// @dev 
    /// @param token_id The ID of the token
    fn usage_return(&mut self, token_id: String);

    /// @notice Get the leasing period of token_id.
    /// @dev 
    /// @param token_id The ID of the token
    /// @returns time up block_height
    fn get_leasing_period(&self, token_id: String) -> u64;
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
            usage_approvals: LookupMap::new(StorageRecord::UsageApprovals),
            leasing_period: LookupMap::new(StorageRecord::LeasingPeriod),
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

        assert_eq!(asset_rights.ownership , asset_rights.usage_rights, "the `ownership` must be the same as `usage_rights` in `mint`");

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

        let art = self.owner_ship.get(&token_id).expect("`token_id` not exist!");
        // token without approves

        // the owner of the token must be the same as `from`
        assert_eq!(from, art.ownership, "Ownership Unauthorized");

        // if in ownership, the `from` must be the same as the `predecessor_account_id`
        // if not, the `predecessor_account_id` must be in the `self.approvals` of the token
        let pre_account = env::predecessor_account_id();

        if pre_account != from{
            if self.approvals.get(&token_id).expect("Caller Ownership Unauthorized: 1!") != pre_account{
                env::panic_str("Caller Ownership Unauthorized: 2");
            }
        }

        // process leasing first
        if !self.leasing_period.contains_key(&token_id){
            // if there's no leasing, the `ownership` must be the same as `usage_rights`
            assert_eq!(art.ownership, art.usage_rights, "Usage Unauthorized: if there's no leasing, the `ownership` must be the same as `usage_rights`");

            // transfer the usage from `from` to `to`
            // log!("Before usage transfer!");
            self.transfer_usage_without_check(from.clone(), to.clone(), token_id.clone());
        }

        self.transfer_ownership_without_check(from, to, token_id);
    }

    /// @notice Transfers the ownership of an NFT from one address to another address
    /// @dev Throws unless `msg.sender` is the current owner, an authorized
    ///  operator, or the approved address for this NFT. Throws if `_from` is
    ///  not the current owner. Throws if `_to` is the zero address. Throws if
    ///  `_tokenId` is not a valid NFT. When transfer is complete, this function
    ///  checks if `_to` is a smart contract (code size > 0). If so, it calls
    ///  `onERC721Received` on `_to` and throws if the return value is not
    ///  `bytes4(keccak256("onERC721Received(address,address,uint256,bytes)"))`.
    /// @param _from The current owner of the NFT
    /// @param _to The new owner
    /// @param _tokenId The NFT to transfer
    /// @param data Additional data with no specified format, sent in call to `_to`
    pub fn safeTransferFrom(&mut self, from: AccountId, to: AccountId, token_id: String, data: String){
        self.transferFrom(from.clone(), to.clone(), token_id.clone());

        // note: if you intended to print `{` or `}`, you can escape it using `{{` or `}}`
        // let arguments = format!("{{\"operator\": \"{}\", \"from\": \"{}\", \"token_id\": \"{}\", \"data\": \"{}\"}}", env::predecessor_account_id().as_str(), from.as_str(), token_id, data);

        let arguments = near_sdk::serde_json::json!({
            "operator": env::predecessor_account_id(),
            "from": from.as_str(),
            "token_id": token_id,
            "data": data,
        });

        let arguments = Base64VecU8::from(arguments.to_string().into_bytes());

        Promise::new(to.clone())
        .function_call("onERC721Received".to_string(),
            arguments.into(),
            0,
            GAS_FOR_FUNCTION_CALL);
    }

    /// @notice Transfers the ownership of an NFT from one address to another address
    /// @dev This works identically to the other function with an extra data parameter,
    ///  except this function just sets data to "".
    /// @param _from The current owner of the NFT
    /// @param _to The new owner
    /// @param _tokenId The NFT to transfer
    fn safeTransferFromNone(&mut self, from: AccountId, to: AccountId, token_id: String){
        self.safeTransferFrom(from, to, token_id, "".to_string());
    }

    pub fn approve(&mut self, approved: AccountId, token_id: String){
        let owner = self.owner_ship.get(&token_id).expect("Token does not exist!");

        assert_eq!(env::predecessor_account_id(), owner.ownership, "Ownership Unauthorized");

        self.approvals.insert(&token_id, &approved);
    }

    pub fn getApproved(&self, token_id: String) -> AccountId{
        self.approvals.get(&token_id).expect("token is not approved!")
    }

    /// @notice Enable or disable approval for a third party ("operator") to manage
    ///  all of `msg.sender`'s assets
    /// @dev Emits the ApprovalForAll event. The contract MUST allow
    ///  multiple operators per owner.
    /// @param _operator Address to add to the set of authorized operators
    /// @param _approved True if the operator is approved, false to revoke approval
    pub fn setApprovalForAll(&mut self, operator: AccountId, approved: bool){
        let pre_account = env::predecessor_account_id();

        if pre_account == operator{
            return;
        }

        let assets_owned = self.assets_own_info.get(&pre_account).expect("None of assets!");

        if approved{
            for token in assets_owned.iter(){
                self.approvals.insert(&token, &operator);
            }
        }else{
            for token in assets_owned.iter(){
                self.approvals.remove(&token);
            }
        }
    }

    /// @notice Query if an address is an authorized operator for another address
    /// @param _owner The address that owns the NFTs
    /// @param _operator The address that acts on behalf of the owner
    /// @return True if `_operator` is an approved operator for `_owner`, false otherwise
    pub fn isApprovedForAll(&self, owner: AccountId, operator: AccountId) ->bool{
        let assets_owned = self.assets_own_info.get(&owner).expect("None of assets!");

        for token in assets_owned.iter(){
            let approved_opt = self.approvals.get(&token);
            if let Some(approved) = approved_opt {
                if approved != operator{
                    return false;
                }
            }else{
                return false;
            }
        }

        true
    }

    // for test interfaces
    pub fn get_contract_meta_data(&self) -> metadata::ContractMetaData{
        self.contract_meta.clone()
    }

    // private 
    #[private]
    fn transfer_ownership_without_check(&mut self, from: AccountId, to: AccountId, token_id: String){        
        let mut art = self.owner_ship.get(&token_id).expect("token dose not exist!");

        // update `self.assets_own_info`
        let cur_owned_tokens = self.assets_own_info.get(&from);
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

            // update `self.owner_ship`
            art.ownership = to;
            self.owner_ship.insert(&token_id, &art);

            // delete from approved
            self.approvals.remove(&token_id);

        }else{
            env::panic_str("There's a bug, because someone has the ownership, but the asset dose not existed in the asset_own_info table!");
        }
    }
    // call back

}

#[near_bindgen]
impl NFTUsage for Contract{
    #[private]
    #[deny(useless_deprecated)]
    fn transferUsageFrom(&mut self, from: AccountId, to: AccountId, token_id: String){
        if !env::is_valid_account_id(to.as_bytes()){
            env::panic_str("Invalid usage account id!");
        }
        
        let mut asset_right = self.owner_ship.get(&token_id).expect("token dose not exist!");

        assert_eq!(from, asset_right.usage_rights, "Unauthorized");

        let pre_account = env::predecessor_account_id();
        
        if pre_account != from{
            if self.usage_approvals.get(&token_id).expect("") != pre_account{
                env::panic_str("Caller Unauthorized");
            }
        }

        self.transfer_usage_without_check(from, to, token_id);
    }

    #[private]
    #[deny(useless_deprecated)]
    fn approveUsage(&mut self, approved: AccountId, token_id: String){
        let owner = self.owner_ship.get(&token_id).expect("token does not exist!");

        assert_eq!(env::predecessor_account_id(), owner.usage_rights, "Usage Unauthorized");

        self.usage_approvals.insert(&token_id, &approved);
    }

    #[private]
    #[deny(useless_deprecated)]
    fn getUsageApproved(&self, token_id: String) ->AccountId{
        self.usage_approvals.get(&token_id).expect("token does not exist!")
    }

    #[private]    
    fn transfer_usage_without_check(&mut self, from: AccountId, to: AccountId, token_id: String){
        let mut asset_right = self.owner_ship.get(&token_id).expect("token dose not exist!");

        let mut cur_usage_tokens = self.assets_usage_info.get(&from).expect("There's a bug, because someone has the usage_right, but the asset does not existed in the asset_usage_info table!");

        // log!("In usage transfer!");

        // delete from current usage
        cur_usage_tokens.remove(&token_id);
        self.assets_usage_info.insert(&from, &cur_usage_tokens);

        // add into new usage
        let mut new_usage_tokens = self.assets_usage_info.get(&to).unwrap_or_else(||{
            UnorderedSet::new(StorageRecord::AssetsUsageTable{
                account_hash: env::sha256(to.as_bytes()),
            })
        });
        new_usage_tokens.insert(&token_id);
        self.assets_usage_info.insert(&to, &new_usage_tokens);

        // change usage_rights
        asset_right.usage_rights = to;
        self.owner_ship.insert(&token_id, &asset_right);

        // delete from approved
        self.usage_approvals.remove(&token_id);

        // log!("The usage is: {}", self.owner_ship.get(&token_id).expect("no token").usage_rights.as_str());
    }

    fn lend_usage_to(&mut self, to: AccountId, token_id: String, period: u64){
        if !env::is_valid_account_id(to.as_bytes()){
            env::panic_str("Invalid `to` account!");
        }
        
        let pre_account = env::predecessor_account_id();

        let mut art = self.owner_ship.get(&token_id).expect("token does not exist!");

        if !((art.ownership == pre_account) && (art.usage_rights == pre_account))
        {
            env::panic_str("Lend Unauthorized!");
        }

        let mut cur_usage_tokens = self.assets_usage_info.get(&art.usage_rights).expect("There's a bug, because someone has the usage_right, but the asset does not existed in the asset_usage_info table!");

        // delete from current usage
        cur_usage_tokens.remove(&token_id);
        self.assets_usage_info.insert(&art.usage_rights, &cur_usage_tokens);

        // add into new usage
        let mut new_usage_tokens = self.assets_usage_info.get(&to).unwrap_or_else(||{
            UnorderedSet::new(StorageRecord::AssetsUsageTable{
                account_hash: env::sha256(to.as_bytes()),
            })
        });
        new_usage_tokens.insert(&token_id);
        self.assets_usage_info.insert(&to, &new_usage_tokens);

        // change usage_rights
        art.usage_rights = to;
        self.owner_ship.insert(&token_id, &art);

        // delete from approved
        self.usage_approvals.remove(&token_id);

        // add leasing record
        self.leasing_period.insert(&token_id, &(env::block_height() + period));
    }

    fn usage_return(&mut self, token_id: String){
        let pre_account = env::predecessor_account_id();

        let mut art = self.owner_ship.get(&token_id).expect("token does not exist!");

        if pre_account != art.usage_rights{
            if pre_account != art.ownership{
                env::panic_str("Return Unauthorized!");
            }else{
                let token_lease_period = self.leasing_period.get(&token_id).expect("No leasing record!");
                if env::block_height() <= token_lease_period{
                    env::panic_str("Return Unauthorized. Not time up!");
                }
            }
        }

        let mut cur_usage_tokens = self.assets_usage_info.get(&art.usage_rights).expect("There's a bug, because someone has the usage_right, but the asset does not existed in the asset_usage_info table!");

        // delete from current usage
        cur_usage_tokens.remove(&token_id);
        self.assets_usage_info.insert(&art.usage_rights, &cur_usage_tokens);

        // add into new usage
        let mut new_usage_tokens = self.assets_usage_info.get(&art.ownership).unwrap_or_else(||{
            UnorderedSet::new(StorageRecord::AssetsUsageTable{
                account_hash: env::sha256(art.ownership.as_bytes()),
            })
        });
        new_usage_tokens.insert(&token_id);
        self.assets_usage_info.insert(&art.ownership, &new_usage_tokens);

        // change usage_rights
        art.usage_rights = art.ownership.clone();
        self.owner_ship.insert(&token_id, &art);

        // delete from approved
        self.usage_approvals.remove(&token_id);

        // add leasing record
        self.leasing_period.remove(&token_id);

    }

    fn get_leasing_period(&self, token_id: String)->u64{
        self.leasing_period.get(&token_id).expect("token is not lent")
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
    #[test]
    fn test_json(){
        let s1 = "1".to_string();
        let s2 = "2".to_string();
        let s3 = "3".to_string();
        let s4 = "4".to_string();

        let arguments = format!("{{\"operator\":\"{}\",\"from\":\"{}\",\"token_id\":\"{}\",\"data\":\"{}\"}}", s1, s2, s3, s4);
        let args = near_sdk::serde_json::json!({
            "operator": s1,
            "from": s2,
            "token_id": s3,
            "data": s4,
        });

        assert_eq!(arguments.to_string(), args.to_string());
    }
    
}
