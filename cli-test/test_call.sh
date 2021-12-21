#!/bin/bash

near call other.monkeyluffyd.testnet say_hello --account-id monkeyluffyd.testnet

near view other.monkeyluffyd.testnet c_say_hello

#test user defined input and output params
near call other.monkeyluffyd.testnet set_and_get_md '{"md" : {"s": "hello", "i": -73, "v": [ 1, 2, 3 ] }}' --account-id monkeyluffyd.testnet

near call other.monkeyluffyd.testnet new --account-id monkeyluffyd.testnet

near call other.monkeyluffyd.testnet inster_val '{"k": 73, "s": "hello"}' --account-id monkeyluffyd.testnet

near view other.monkeyluffyd.testnet get_val '{"k": 73}'

near delete other.monkeyluffyd.testnet monkeyluffyd.testnet

near create-account other.monkeyluffyd.testnet --masterAccount monkeyluffyd.testnet --initialBalance 10

near deploy other.monkeyluffyd.testnet --wasmFile ./res/*.wasm

near call other.monkeyluffyd.testnet new '{"contract_meta": {"spec": "hello666", "name": "myother-666", "symbol": "PLANCK", "icon": "", "base_uri": "", "reference": "", "reference_hash": "12345678"}}' --account-id other.monkeyluffyd.testnet

near view other.monkeyluffyd.testnet get_contract_meta_data

near view other.monkeyluffyd.testnet usageOf '{"token_id" : "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}'

near view other.monkeyluffyd.testnet ownerOf '{"token_id" : "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}'

near view other.monkeyluffyd.testnet balanceOf '{"account_id" : "other.monkeyluffyd.testnet"}'

# mint
near call other.monkeyluffyd.testnet mint \
 '{"asset_rights": {"ownership": "other.monkeyluffyd.testnet", "usage_rights" : "other.monkeyluffyd.testnet"}, "token_metadata": {"title": "hello", "description": "world", "media": "May", "media_hash": "12345678", "copies": 1, "issued_at": "", "expires_at": "", "starts_at": "", "updated_at": "", "extra": "", "reference": "", "reference_hash": "12345678"}}' \
 --account-id other.monkeyluffyd.testnet

near view other.monkeyluffyd.testnet tokenURI '{"token_id": "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}'

#transfer
near call other.monkeyluffyd.testnet transferFrom \
 '{"from": "other.monkeyluffyd.testnet", "to": "monkeyluffyd.testnet", "token_id": "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}' \
  --account-id other.monkeyluffyd.testnet

#approve
near call other.monkeyluffyd.testnet approve \
 '{"approved" : "monkeyluffyd.testnet", "token_id" : "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}' \
 --account_id other.monkeyluffyd.testnet

 near view other.monkeyluffyd.testnet getApproved '{"token_id" : "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}'

#Usage Transfer. private currently#######################################################

#transfer
near call other.monkeyluffyd.testnet transferUsageFrom \
 '{"from": "monkeyluffyd.testnet", "to": "other.monkeyluffyd.testnet", "token_id": "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}' \
  --account-id other.monkeyluffyd.testnet

#approve
near call other.monkeyluffyd.testnet approveUsage \
 '{"approved" : "other.monkeyluffyd.testnet", "token_id" : "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}' \
 --account_id monkeyluffyd.testnet

 near view other.monkeyluffyd.testnet getUsageApproved '{"token_id" : "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}'

#Usage leasing###############################################################
near view other.monkeyluffyd.testnet get_leasing_period '{"token_id" : "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}'

near call other.monkeyluffyd.testnet lend_usage_to \
 '{"to" : "other.monkeyluffyd.testnet", "token_id" : "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129", "period" : 100}' \
 --account_id monkeyluffyd.testnet

near call other.monkeyluffyd.testnet usage_return '{"token_id": "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}' \
 --account_id other.monkeyluffyd.testnet
