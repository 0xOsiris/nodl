# A simple CLI utility to interface with Ethereum Execution Clients

## Installation

Run the following to install the binary:

```bash
$ cargo install --path .
```

### Usage

```bash
$ nodl --help 
```
```txt
A CLI for interacting with Ethereum nodes.

Usage: nodl [OPTIONS] <COMMAND>

Commands:
  tx-pool  Inspect the TxPool of a node [aliases: tp]
  net      Interface with the Net API of a node [aliases: n]
  admin    Interface with the Admin API of a node [aliases: a]
  web3     Interface with the Web3 API of a node [aliases: w]
  help     Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>  Path to dump the json output to. If not provided, the output will be printed to stdout
  -h, --help         Print help
  -V, --version      Print version
```

#### [TxPool](https://reth.rs/jsonrpc/txpool.html)
```bash
$ nodl tx-pool --help
```
```txt
Inspect the TxPool of a node

Usage: nodl tx-pool <COMMAND>

Commands:
  content       Fetches the content of the transaction pool
  content-from  Fetches the content of the transaction pool filtered by a specific address
  inspect       Fetches a textual summary of each transaction in the pool
  status        Fetches the current status of the transaction pool
  help          Print this message or the help of the given subcommand(s)
```

#### [Net](https://reth.rs/jsonrpc/net.html)
```bash
$ nodl net --help
```
```txt
Interface with the Net API of a node

Usage: nodl net <COMMAND>

Commands:
  listening   Returns a bool indicating whether or not the node is listening for network connections
  peer-count  Returns the number of peers connected to the node
  version     Returns the network ID (e.g. 1 for mainnet)
  help        Print this message or the help of the given subcommand(s)
```

#### [Admin](https://reth.rs/jsonrpc/admin.html) 
```bash
$ nodl admin --help
```
```txt
Interface with the Admin API of a node

Usage: nodl admin <COMMAND>

Commands:
  add-peer             Add the given peer to the current peer set of the node
  remove-peer          Disconnects from a peer if the connection exists. Returns a bool indicating whether the peer was successfully removed or not
  add-trusted-peer     Adds the given peer to a list of trusted peers, which allows the peer to always connect, even if there would be no room for it otherwise
  remove-trusted-peer  Removes a remote node from the trusted peer set, but it does not disconnect it automatically
  node-info            Returns all information known about the running node
  help                 Print this message or the help of the given subcommand(s)
```

#### [Web3](https://reth.rs/jsonrpc/web3.html)
```bash
$ nodl web3 --help
```
```txt
Interface with the Web3 API of a node

Usage: nodl web3 <COMMAND>

Commands:
  client-version  Returns the current client version
  help            Print this message or the help of the given subcommand(s)
```

