# Terra NFT Stub contract

Testing interactions with other NFT projects locally is difficult.
Terra NFT Stub makes things a bit easier by providing a contract that allows you to easily(?) stub
out tokens with known characteristics and attributes.

Any token that follows the CW721 Metadata structure can be stubbed.

## Usage

The following commands will build and optimize a WASM binary.

```
cargo build
cargo optimize
```

Look through the scripts in `terrad-scripts/` for usage instructions.
The scripts work but need to be manually updated to include specific details for `code_id`,
contract address, minter address, etc.
After running each script, you can use the TX hash returned by the script to look up the
transaction in finder and then copy the required details to the next script that you need to run.

## Ideas and suggestions

Please submit ideas for extending and improving this testing contract as an issue.
The initial idea was to build something that would allow deterministic `nft_info` and
`all_nft_info` messages but there are plenty more ways that this contract could be extended to
support local testing.

## Test contract only

This contract is explicitly for testing purposes only.
Don't use this repo as a basis for building a production smart contract, and don't deploy this
contract on `mainnet`.

## Inspiration

This project leans heavily on work done by TerraPeeps and R.E.S.T.
Digging through those projects has help make this project a reality.
TerraPeeps helped understand hadling custom messages and querying.
R.E.S.T. helped thinking about structuring smart contract code and how to reuse the reference
implementation in CW721 base and CW721 Metadata Onchain.

This is still a project created to help learn about smart contracts on Terra, and also learn and
pratctice Rust though.
Any suggestions on how to improve the structure of the code or how to write more effective Rust are
welcome; just open an issue!
