#
# A reference script to upload the optimized WASM file using the test1 account on localterra
#

admin=terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v

# uploading is the store command
terrad tx wasm store artifacts/terra_nft_stub.wasm --chain-id localterra --from ${admin} --fees 391868uusd --gas auto -y -b sync --gas-adjustment 1.2
