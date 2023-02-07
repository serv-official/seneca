Please make sure you set up your rust and substrate environment following these instructions https://docs.substrate.io/install/

## Please install the following packages after setup

For linux

```
sudo apt install binaryen
```
On macos
 ```
 brew install binaryen
 ```

 After you've installed the WebAssembly binaryen package, you can install the cargo-contract package. The cargo-contract package provides a command-line interface for working with smart contracts using the ink! language.

 ```
 cargo install cargo-dylint dylint-link
 ```

 Install cargo-contract by running the following command:

 ```
 cargo install cargo-contract --force
 ```

 Verify the installation and explore the commands available by running the following command:

 ```
 cargo contract --help
 ```

 Now you can go ahead and create a new contract

 ```
 cargo contract new <contract name>
 ```

 After developing you contract, you can go ahead and build it

 ```
 cargo +nightly contract build
 ```
 This command builds a WebAssembly binary for the <name> project, a metadata file that contains the contract Application Binary Interface (ABI), and a .contract file that you use to deploy the contract. For example, you should see output similar to the following:

 ```
 Original wasm size: 47.8K, Optimized: 22.4K

The contract was built in DEBUG mode.

Your contract artifacts are ready. You can find them in:
/Users/dev-doc/<contract-name>/target/ink

- <contract-name>.contract (code + metadata)
- <contract-name>.wasm (the contract's code)
- metadata.json (the contract's metadata)
```

If you downloaded the precompiled binary on a macOS computer, you can run the following command after building the node:

```
./target/release/serv-node --dev
```

After a few seconds, you should see blocks being finalized.

To interact with the blockchain, you need to connect to this node. You can connect to the node through a browser by opening the [Contracts UI](https://contracts-ui.substrate.io/).

Select Local Node.

Deploying a smart contract on Substrate is a little different than deploying on traditional smart contract platforms. For most smart contract platforms, you must deploy a completely new blob of the smart contract source code each time you make a change. For example, the standard ERC20 token has been deployed to Ethereum thousands of times. Even if a change is minimal or only affects some initial configuration setting, each change requires a full redeployment of the code. Each smart contract instance consumes blockchain resources equivalent to the full contract source code, even if no code was actually changed.

### In Substrate, the contract deployment process is split into two steps:

* Upload the contract code to the blockchain.
* Create an instance of the contract.

With this pattern, you can store the code for a smart contract like the ERC20 standard on the blockchain once, then instantiate it any number of times. You don't need to reload the same source code repeatedly, so your smart contract doesn't consume unnecessary resources on the blockchain.

## Deploy contract

To upload the smart contract source code:

* Open the Contracts UI in a web browser.
* Verify that you are connected to the Local Node.
* Click Add New Contract.
* Click Upload New Contract Code.
* Select an Account to use to create a contract instance. You can select any existing account, including a predefined account such as alice.

* Type a descriptive Name for the smart contract, for example, <contract-name> Contract.
* Browse and select or drag and drop the <contract-name>.contract file that contains the bundled Wasm  blob and metadata into the upload section.
* Click Next to continue.

### Create a contract instance

To create the instance:

* Review and accept the default Deployment Constructor options for the initial version of the smart contract.
* Review and accept the default Max Gas Allowed.
* Click Next,the transaction is now queued. If you needed to make changes, you could click Go Back to modify the input.
* Click Upload and Instantiate, depending on the account you used, you might be prompted for the account password. If you used a predefined account, you won't need to provide a password.

Now that your contract has been deployed on the blockchain, you can interact with it.