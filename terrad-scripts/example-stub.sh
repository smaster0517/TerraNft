#
# A reference script to show how to send a msg to create a stub token
#

# The test1 wallet on localterra
minter=terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v

# Remember to update the contract address after instantiating or migrating!
contract=terra1y0l0ufehvw7vgavrqdml2s48vh36tthwyjspxx

stub_token_json='{"stub":{"token_id":"FOO1","token_uri":"https://tokens.test/foo1.json","owner_id":"'${minter}'","attributes":"{}"}}'

terrad tx wasm execute ${contract} "${stub_token_json}" 3000000uluna --chain-id localterra --gas auto --from ${minter} --fees 5627uluna
