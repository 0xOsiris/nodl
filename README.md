# A simple CLI utility to interface with Ethereum Execution Clients

## Installation

```sh

cargo install --path .

```

## Suported API's

- [Admin](https://reth.rs/jsonrpc/admin.html) 
- [Net](https://reth.rs/jsonrpc/net.html)
- [TxPool](https://reth.rs/jsonrpc/txpool.html)
- [Web3](https://reth.rs/jsonrpc/web3.html)


### Example Usage

```sh

# Inspect the Transaction pool of your node
nodl txpool content --rpc-url <URI> --jwt <TOKEN> --path <PATH_TO_EXPORT>

# Return all known info about your node
nodl admin node-info --rpc-url <URI> --jwt <TOKEN>

# Fetch the peer count of your node
nodl net peer-count -rpc-url <URI> --jwt <TOKEN>

```