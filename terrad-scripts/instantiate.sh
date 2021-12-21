#
# A reference script to instantiate the stub smart contract using the test1 account on localterra
#

# Wallet address of test1 in localterra
minter=terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v

# Change according to code_id of latest store
code_id=8

# This sample does not include a static token, so uses the default static token defined in the
# contract.
stub_json='{"name":"stub1","symbol":"STUB1","minter":"'${minter}'","always_owner":"'${minter}'"}'

# Tries to include verbose logging but this seems to do nothing to actually change output.
terrad --log_level trace --trace tx wasm instantiate ${code_id} "${stub_json}" --chain-id localterra --gas auto --from ${minter} --fees  5627uluna --admin ${minter}
