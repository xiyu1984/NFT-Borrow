#!/bin/bash

near call testzero.monkeyluffyd.testnet say_hello --account-id monkeyluffyd.testnet

near view testzero.monkeyluffyd.testnet c_say_hello

#test user defined input and output params
near call other.monkeyluffyd.testnet set_and_get_md '{"md" : {"s": "hello", "i": -73, "v": [ 1, 2, 3 ] }}' --account-id monkeyluffyd.testnet

near call other.monkeyluffyd.testnet new --account-id monkeyluffyd.testnet

near call other.monkeyluffyd.testnet inster_val '{"k": 73, "s": "hello"}' --account-id monkeyluffyd.testnet

near view other.monkeyluffyd.testnet get_val '{"k": 73}'

near delete other.monkeyluffyd.testnet monkeyluffyd.testnet

near create-account other.monkeyluffyd.testnet --masterAccount monkeyluffyd.testnet --initialBalance 10

near call other.monkeyluffyd.testnet new '{"contract_meta": {"spec": "hello666", "name": "myNFT-666", "symbol": "PLANCK", "icon": "", "base_uri": "", "reference": "", "reference_hash": "12345678"}}' --account-id other.monkeyluffyd.testnet

near view other.monkeyluffyd.testnet get_contract_meta_data

near view other.monkeyluffyd.testnet usageOf '{"token_id" : "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}'

near view other.monkeyluffyd.testnet ownerOf '{"token_id" : "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}'

5d93d2a82d608c16cbf7215f92fd47e46417e7782b9887b4afc7ebe3e57240d3

near view other.monkeyluffyd.testnet balanceOf '{"account_id" : "other.monkeyluffyd.testnet"}'

near call other.monkeyluffyd.testnet mint \
 '{"asset_rights": {"ownership": "other.monkeyluffyd.testnet", "usage_rights" : "monkeyluffyd.testnet"}, "token_metadata": {"title": "hello", "description": "world", "media": "May", "media_hash": "12345678", "copies": 1, "issued_at": "", "expires_at": "", "starts_at": "", "updated_at": "", "extra": "", "reference": "", "reference_hash": "12345678"}}' \
 --account-id other.monkeyluffyd.testnet

 near view other.monkeyluffyd.testnet tokenURI '{"token_id": "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}'

 near call other.monkeyluffyd.testnet transferFrom '{"from": "monkeyluffyd.testnet", "to": "other.monkeyluffyd.testnet", "token_id": "4c33352ee21a5f3bf79e24993946f1382f3dd0d665d6533a7dfe931c83c74129"}' --account-id monkeyluffyd.testnet