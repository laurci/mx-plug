# export SC_ADDRESS=erd1qqqqqqqqqqqqqpgqj5zftf3ef3gqm3gklcetpmxwg43rh8z2d8ss2e49aq
export SC_ADDRESS=erd1qqqqqqqqqqqqqpgqf6gygvccj4dsxr4qj6v7ssfyvuq2nkuhd8ss8sa577

deploySc() {
  mxpy --verbose contract deploy --bytecode ./output/test-smart-contract.wasm --pem=~/multiversx-sdk/testwallets/latest/users/alice.pem --proxy=http://localhost:7950 --send --gas-limit=500000000 --recall-nonce
}

upgradeSc() {
  mxpy --verbose contract upgrade $SC_ADDRESS --bytecode ./output/test-smart-contract.wasm --pem=~/multiversx-sdk/testwallets/latest/users/alice.pem --proxy=http://localhost:7950 --send --gas-limit=500000000 --recall-nonce
}

callScTest_1() {
  mxpy --verbose contract call $SC_ADDRESS --pem=~/multiversx-sdk/testwallets/latest/users/alice.pem --proxy=http://localhost:7950 --send --gas-limit=500000000 --recall-nonce --function="test_1"
}
