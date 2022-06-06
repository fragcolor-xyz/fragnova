# Clamor

Clamor is a custom blockchain built on the [Substrate](https://substrate.io/) framework.

It is a protocol and networking stack that enables complete on-chain storage and full synchronization of asset data (protos, fragments, chainblocks code etc.) across the blockchain nodes.

## Installation

### Requirements

Before you start, ensure you've [set up your development environment](https://docs.fragcolor.xyz/contribute/code/getting-started/) and [installed Rust](https://docs.fragcolor.xyz/contribute/code/getting-started/#install-setup-rust).

*NOTE - The following instructions are for developing Clamor on Linux/Mac since Substrate doesn't yet have reliable [native Windows support](https://docs.substrate.io/v3/getting-started/windows-users/). On Windows, [WSL](https://docs.microsoft.com/en-us/windows/wsl/) might work with these instructions, but results are not guaranteed.*

### Update system packages

 We use `rust nightly` so run `rustup update` every week when building Clamor. This will update your Rust installation, tools, and dependencies.

### Build & run the project

Clone the Clamor repository and checkout the default branch.

Navigate to the root folder and run the following script to build and run a local instance of Clamor:

```
RUST_LOG=bitswap=trace,pallet_protos::pallet=trace,pallet_frag::pallet=trace cargo run -- --dev --tmp --rpc-external --rpc-port 9933 --rpc-cors all --ws-external --enable-offchain-indexing 1 --rpc-methods=Unsafe --ipfs-server
```

If you want to run Clamor with a [chain specification](https://docs.substrate.io/v3/runtime/chain-specs/) instead, use this script:

```
cargo run -- --chain=spec_raw.json --validator --rpc-external --rpc-port 9933 --rpc-cors all --ws-external --enable-offchain-indexing 1 --rpc-methods=Unsafe --ipfs-server -d <DATA PATH>
```

## Usage

### Uploading test data programatically

 Once you have a Clamor node running locally you can programatically upload test data to it with the following script (using Docker and Chainblocks):

```
docker run --rm --user root --network host -v `pwd`:/data chainblocks/shards shards /data/shards/add-test-assets.edn
```

### Connecting to Polkadot's App Explorer

[Polkadot.js](https://github.com/polkadot-js/) provides a browser based application, [App Explorer](https://polkadot.js.org/apps/#/explorer) (also available as hosted IPFS and IPNS versions). This application allows you to interact with your locally running Substrate node, with minimal setup.

To do this:

1. Run you Clamor node locally
2. Head over to the [App Explorer](https://polkadot.js.org/apps/#/explorer)
3. Click the top-left Pokadot icon on the header of the page
4. Expand the **Development** sub-menu (at the bottom of the list)
5. Click **Local Node** to enable it
6. Click **Switch** at the top of the panel

The App Explorer will now connect with your local node and will show the blocks being produced by your node in real-time.

## License
Clamor is licensed under the terms of the [BUSL-1.1](https://spdx.org/licenses/BUSL-1.1.html) license.
