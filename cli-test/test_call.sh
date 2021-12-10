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