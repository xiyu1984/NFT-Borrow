# Rust Smart Contract For NFT666

## Description
Something interesting will be happened in metaverse, but the current standard for non-fungible tokens may not be able to adapt to some interesting situations. For example, if someone wants to lend an exceptionally valuable digital asset in the virtual world, to a friend, the existing standards do not guarantee that his friend will return it.

I believe that in the future metaverse world, the rental of digital assets is a very common situation, so I am ready to study how to provide a better service based on the existing NFT standard.

First, I'll try make NFT666 from ERC721(https://eips.ethereum.org/EIPS/eip-721).

The NFT666 has beed completed!

## Getting started

To get started with this template:

1. Click the "Use this template" button to create a new repo based on this template
2. Update line 2 of `Cargo.toml` with your project name
3. Update line 4 of `Cargo.toml` with your project author names
4. Set up the [prerequisites](https://github.com/near/near-sdk-rs#pre-requisites)
5. Begin writing your smart contract in `src/lib.rs`
6. Test the contract 

    `cargo test -- --nocapture`

8. Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)
