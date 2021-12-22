# Terra NFT Stub contract

Testing interactions with other NFT projects locally is difficult.
Terra NFT Stub makes things a bit easier by providing a contract that allows you to easily(?) stub
out tokens with known characteristics and attributes.

Any token that follows the CW721 Metadata structure can be stubbed.

## Usage

This project is still early stage so has not been published on crates.io yet.

The following commands will build and optimize a WASM binary.

```
cargo build
cargo optimize
```

The scripts in `terrad-scripts/` provide examples for storing and instantiating the contract
against a `localterra` instance, and a basic example of sending a `Stub` msg to stub out a new
token in the contract.

The scripts need to be manually updated to include specific details for `code_id`, contract
address, minter address, etc.
After running each script, you can use the TX hash returned by the script to look up the
transaction in finder and then copy the required details to the next script that you need to run.

## Ideas and suggestions

Please submit ideas for extending and improving this testing contract as an issue.
The initial motivation was to build something that would allow deterministic results to be returned
from `nft_info` and `all_nft_info` messages to have predictable results when using either query
within another smart contract.

There are plenty more ways that this contract could be extended to support local testing though, and
we'd like to know what others want from a stub NFT contract for testing and developing Terra smart
contracts.

## Test contract only

This contract is explicitly for testing purposes only.
Don't use this repo as a basis for building a production smart contract, and don't deploy this
contract on `mainnet`.

## Inspiration

This project leans heavily on work done by [TerraPeeps](https://github.com/PFC-Validator/terra-peep721) and [R.E.S.T](https://github.com/R-E-S-T/rest-nft).

Digging through those projects has help make this project a reality.
TerraPeeps helped understand handling custom messages, querying and storage.
R.E.S.T. helped thinking about structuring smart contract code and how to reuse the reference
implementation in CW721 base and CW721 Metadata Onchain.

This is still a project created to help learn about smart contracts on Terra, and also learn and
pratctice Rust, though.
Any suggestions on how to improve the structure of the code or how to write more effective Rust are
welcome; just open an issue!
