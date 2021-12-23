#!/bin/bash

near call nft.monkeyluffyd.testnet say_hello --account-id monkeyluffyd.testnet

near view nft.monkeyluffyd.testnet c_say_hello

#test user defined input and output params
near call nft.monkeyluffyd.testnet set_and_get_md '{"md" : {"s": "hello", "i": -73, "v": [ 1, 2, 3 ] }}' --account-id monkeyluffyd.testnet

near call nft.monkeyluffyd.testnet new --account-id monkeyluffyd.testnet

near call nft.monkeyluffyd.testnet inster_val '{"k": 73, "s": "hello"}' --account-id monkeyluffyd.testnet

near view nft.monkeyluffyd.testnet get_val '{"k": 73}'

near delete nft.monkeyluffyd.testnet monkeyluffyd.testnet

near create-account nft.monkeyluffyd.testnet --masterAccount monkeyluffyd.testnet --initialBalance 10

near call nft.monkeyluffyd.testnet new '{"contract_meta": {"spec": "hello666", "name": "myNFT-666", "symbol": "PLANCK", "icon": "", "base_uri": "", "reference": "", "reference_hash": "12345678"}}' --account-id nft.monkeyluffyd.testnet

near view nft.monkeyluffyd.testnet get_contract_meta_data

near view nft.monkeyluffyd.testnet usageOf '{"token_id" : "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}'

near view nft.monkeyluffyd.testnet ownerOf '{"token_id" : "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}'

near view nft.monkeyluffyd.testnet balanceOf '{"account_id" : "nft.monkeyluffyd.testnet"}'

# mint
near call nft.monkeyluffyd.testnet mint \
 '{"asset_rights": {"ownership": "nft.monkeyluffyd.testnet", "usage_rights" : "nft.monkeyluffyd.testnet"}, "token_metadata": {"title": "hello", "description": "world", "media": "May", "media_hash": "12345678", "copies": 1, "issued_at": "", "expires_at": "", "starts_at": "", "updated_at": "", "extra": "", "reference": "", "reference_hash": "12345678"}}' \
 --account-id nft.monkeyluffyd.testnet

 near call nft.monkeyluffyd.testnet mint \
 '{"asset_rights": {"ownership": "zation.testnet", "usage_rights" : "zation.testnet"}, "token_metadata": {"title": "Hi", "description": "zation", "media": "zation1", "media_hash": "12345678", "copies": 1, "issued_at": "", "expires_at": "", "starts_at": "", "updated_at": "", "extra": "", "reference": "", "reference_hash": "12345678"}}' \
 --account-id nft.monkeyluffyd.testnet

near call nft.monkeyluffyd.testnet mint \
 '{"asset_rights": {"ownership": "zation.testnet", "usage_rights" : "zation.testnet"}, "token_metadata": {"title": "Hi", "description": "zation2", "media": "zation2", "media_hash": "12345678", "copies": 1, "issued_at": "", "expires_at": "", "starts_at": "", "updated_at": "", "extra": "", "reference": "", "reference_hash": "12345678"}}' \
 --account-id nft.monkeyluffyd.testnet

near view nft.monkeyluffyd.testnet tokenURI '{"token_id": "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}'

#transfer
near call nft.monkeyluffyd.testnet transferFrom \
 '{"from": "nft.monkeyluffyd.testnet", "to": "monkeyluffyd.testnet", "token_id": "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}' \
  --account-id monkeyluffyd.testnet

#approve
near call nft.monkeyluffyd.testnet approve \
 '{"approved" : "monkeyluffyd.testnet", "token_id" : "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}' \
 --account_id nft.monkeyluffyd.testnet

 near view nft.monkeyluffyd.testnet getApproved '{"token_id" : "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}'

 near call nft.monkeyluffyd.testnet setApprovalForAll '{"operator": "monkeyluffyd.testnet", "approved": true}' --account_id nft.monkeyluffyd.testnet

 near view nft.monkeyluffyd.testnet isApprovedForAll '{"owner": "nft.monkeyluffyd.testnet", "operator": "monkeyluffyd.testnet"}'

#Usage Transfer. private currently#######################################################

#transfer
near call nft.monkeyluffyd.testnet transferUsageFrom \
 '{"from": "monkeyluffyd.testnet", "to": "nft.monkeyluffyd.testnet", "token_id": "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}' \
  --account-id nft.monkeyluffyd.testnet

#approve
near call nft.monkeyluffyd.testnet approveUsage \
 '{"approved" : "nft.monkeyluffyd.testnet", "token_id" : "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}' \
 --account_id monkeyluffyd.testnet

 near view nft.monkeyluffyd.testnet getUsageApproved '{"token_id" : "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}'

#Usage leasing###############################################################
near view nft.monkeyluffyd.testnet get_leasing_period '{"token_id" : "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}'

near call nft.monkeyluffyd.testnet lend_usage_to \
 '{"to" : "nft.monkeyluffyd.testnet", "token_id" : "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca", "period" : 100}' \
 --account_id monkeyluffyd.testnet

near call nft.monkeyluffyd.testnet usage_return '{"token_id": "79fa45feb72a9cd7ed453a0d20e83dca40c62482fe6929fb84cc0a56b5449fca"}' \
 --account_id nft.monkeyluffyd.testnet
