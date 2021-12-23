# Terra NFT Stub contract

Testing interactions with other NFT projects locally is difficult.
Terra NFT Stub makes things a bit easier by providing a contract that allows you to easily(?) stub out tokens with known characteristics and attributes.

Any token that follows the CW721 Metadata structure can be stubbed.

## Usage

This project is still early stage so has not been published on crates.io yet.

The following commands will build and optimize a WASM binary.

```
cargo build
cargo optimize
```

The scripts in `terrad-scripts/` provide examples for storing and instantiating the contract against a `localterra` instance, and a basic example of sending a `Stub` msg to stub out a new token in the contract.

You would typically want to:
1. Build and optimize the contract binary.
2. Store the contract code with `store.sh`.
3. Instantiate the contract with `instantiate.sh` (needs the correct `code_id`.)
4. Try creating the example stub with `example-stub.sh` (needs the address of the instantiated contract.)
5. Look up the new stub token in finder.

The scripts need to be manually updated to include specific details for `code_id`, contract address, minter address, etc.
After running each script, you can use the TX hash returned by the script to look up the transaction in finder and then copy the required details to the next script that you need to run.

This smart contract makes use of the [cw721-base](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base) and [cw721-metadata-onchain](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-metadata-onchain) reference implementations specifically so that tokens can be deserialized within other contracts using the same token structs.
A smart contract can import both of the references crates and use the structs in those packages to deserialize a token when calling `query_wasm_smart`.

## Ideas and suggestions

Please submit ideas for extending and improving this testing contract as an issue.
The initial motivation was to build something that would allow deterministic results to be returned from `nft_info` and `all_nft_info` messages to have predictable results when using either query within another smart contract.

There are plenty more ways that this contract could be extended to support local testing though, and we'd like to know what others want from a stub NFT contract for testing and developing Terra smart contracts.

## Test contract only

This contract is explicitly for testing purposes only.
Don't use this repo as a basis for building a production smart contract, and don't deploy this contract on `mainnet`.

## Inspiration

This project leans heavily on work done by [TerraPeeps](https://github.com/PFC-Validator/terra-peep721) and [R.E.S.T](https://github.com/R-E-S-T/rest-nft).

Digging through those projects has help make this project a reality.
TerraPeeps helped understand handling custom messages, querying and storage.
R.E.S.T. helped thinking about structuring smart contract code and how to reuse the reference implementation in CW721 base and CW721 Metadata Onchain.

The [astroport_fi factory project](https://github.com/astroport-fi/astroport-lbport/tree/ee24a0c532ec01a8af61ef58d5efc689bded1a16/contracts/factory) also helped understand how to test and verify how `query_wasm_smart` works, and can be tested.

This is still a project created to help learn about smart contracts on Terra, and also learn and pratctice Rust, though.
Any suggestions on how to improve the structure of the code or how to write more effective Rust are welcome; just open an issue!
