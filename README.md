# Serv Network &middot; [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](docs/CONTRIBUTING.adoc) [![Substrate](https://img.shields.io/badge/v.4-Substrate-blue)](https://joinserv.com/)
<div align="center">
  <p align="center">
    <img src="/docs/media/serv.png">
  </p>
  <strong>Implementation of the serv network node <a href="https://joinserv.com">joinserv.com</a>. Built using <a href="https://github.com/paritytech/substrate">Substrate Framework</a>.</strong> 🚀

  <h3>
    <a href="https://docs.substrate.io">Docs</a> 📚
    <span> | </span>
    <a href="https://discord.gg/H8AZxzh7sx">Chat</a> 💬
  </h3>

</div>

## Features

* WASM Smart contract support
* NPoS
* Onchain governance
* DID support

**Notes:** The code is un-audited and still under active development, use it at your own risk.

## Getting Started

Follow the steps below to get started.

### Rust Setup

First, complete the [Dev Docs Installation](https://docs.substrate.io/install/).

### Build and Run

## Run dev node

Use the following command to build the node and run it after build successfully:

```sh
cargo build --release
./target/release/serv-node --dev
```

## Run public testnet

Start your bootnodes, node key can be generate with command `./target/release/serv-node key generate-node-key`. The network supports 4 initial validators.
  ```shell
    ./target/release/serv-node \
    --node-key  0x0278c3d699e1020d6117cbb6217d6c71e6e502a7460223eb274bed0a6cca2369 \
    --base-path /tmp/bootnode1 \
    --chain serv-network-staging-raw.json \
    --name bootnode1
  ```
* Start your initial validators,
  ```shell
    ./target/release/serv-node \
    --base-path  /tmp/validator1 \
    --chain   serv-network-staging-raw.json \
    --bootnodes  /ip4/<your-bootnode-ip>/tcp/30333/p2p/<your-bootnode-peerid> \
    --port 30336 \
    --ws-port 9947 \
    --rpc-port 9936 \
    --name  validator1 \
    --validator
  ```
* Start second validator,
  ```shell
    ./target/release/serv-node \
    --base-path  /tmp/validator2 \
    --chain   serv-network-staging-raw.json \
    --bootnodes  /ip4/<your-bootnode-ip>/tcp/30333/p2p/<your-bootnode-peerid> \
    --port 30337 \
    --ws-port 9948 \
    --rpc-port 9937 \
    --name  validator2 \
    --validator
  ```
* Start third validator,
  ```shell
    ./target/release/serv-node \
    --base-path  /tmp/validator3 \
    --chain   serv-network-staging-raw.json \
    --bootnodes  /ip4/<your-bootnode-ip>/tcp/30333/p2p/<your-bootnode-peerid> \
    --port 30338 \
    --ws-port 9949 \
    --rpc-port 9938 \
    --name  validator3 \
    --validator
  ```