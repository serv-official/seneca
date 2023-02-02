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

## 🚴 Features

* WASM Smart contract support
* NPoS
* Onchain governance
* DID support

**Notes:** The code is un-audited and still under active development, use it at your own risk.

## 📦 Getting Started

Follow the steps below to get started.

### 🛠️ Rust Setup

First, complete the [Dev Docs Installation](https://docs.substrate.io/install/).

### 🏃 Build and Run

## Run dev node

Use the following command to build the node and run it after build successfully:

```sh
cargo build --release
./target/release/serv --dev
```

### 🕸️ Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet with two validator nodes for Alice and Bob, who are the initial authorities of the genesis chain that have been endowed with testnet units.

Optionally, give each node a name and expose them so they are listed on the Polkadot [telemetry site](https://telemetry.polkadot.io/#/Local%20Testnet).

You'll need two terminal windows open.

We'll start Alice's substrate node first on default TCP port 30333 with her chain database stored locally at `/tmp/alice`. The bootnode ID of her node is `12D3KooWCkmvmzEYwdxS7c6zkXT9K8u2PUxfPRogDShH9CrcecB4`, which is generated from the `--node-key` value that we specify below:

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator
```

In the second terminal, we'll start Bob's substrate node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/bob`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
cargo run -- \
  --base-path /tmp/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWCkmvmzEYwdxS7c6zkXT9K8u2PUxfPRogDShH9CrcecB4 \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator
```

Additional CLI usage options are available and may be shown by running `cargo run -- --help`.

### 🐳 Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can also replace the default command (`cargo build --release && ./target/release/serv --dev --ws-external`) by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/serv --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/serv purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

## ⚡ Run public testnet
Connect as a validator
* Start your validator node, make sure your node has discovered other peers
  ```shell
    ./target/release/serv-node \
    --base-path  /tmp/<validator name> \
    --chain  serv-spec-raw.json \
    --bootnodes  /ip4/172.31.23.55/tcp/30333/p2p/12D3KooWCkmvmzEYwdxS7c6zkXT9K8u2PUxfPRogDShH9CrcecB4 \
    --port 30334 \
    --ws-port 9945 \
    --rpc-port 9934 \
    --name  <validator name> \
    --validator
  ```
* Submit your keys to the network
  | Key Type  | Scheme  |
  |-----------|---------|
  | gran      | Ed25519 |
  | babe      | Sr25519 |
  | imon      | Sr25519 |
  ```
    ./target/release/serv-node key insert \
    --base-path <path for this validator> \
    --chain <raw relay chainSpec> \
    --scheme <see table> \
    --suri "<mnemonic of this validator>" \
    --key-type <see table>
  ```