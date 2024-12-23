# MEV Network

<!-- [![Crates.io][crates-badge]][crates-io] -->
<!-- [![Downloads][downloads-badge]][crates-io] -->

[![MIT License][mit-badge]][mit-url]
[![Apache-2.0 License][apache-badge]][apache-url]




### Local Development

MEV can be run locally for development and testing purposes. To do this, the binary can be run with the `--dev` flag, which will start the node with a development configuration.

First, mev should be built locally:

```bash
git clone https://github.com/MEV-Network/mev
cd mev
cargo install --path bin/mev
```

```bash
mev node --chain etc/mev-genesis.json --dev --http --http.api all
```

This will start the node with a development configuration, and expose the HTTP API on `http://localhost:8545`.

To use EOF-enabled foundry, use [forge-eof](https://github.com/paradigmxyz/forge-eof) and follow installation instructions.

### Running MEV

Running MEV will require running additional infrastructure for the archival L1 node. These instructions are a guide for
running the MEV OP-stack node only.

For instructions on running the full MEV OP stack, including the L1 node, see the [Reth book section on running the OP stack](https://paradigmxyz.github.io/reth/run/optimism.html), using the `mev` binary instead of `op-reth`.

#### Running the MEV execution node

To run MEV from source, clone the repository and run the following commands:

```bash
git clone https://github.com/MEV-Network/mev.git
cd mev
cargo install --path bin/mev
mev node \
    --chain etc/mev-genesis.json \
    --rollup.sequencer-http http://localhost:8551 \
    --http \
    --ws \
    --authrpc.port 9551 \
    --authrpc.jwtsecret /path/to/jwt.hex
```

#### Running op-node with the MEV configuration

Once `mev` is started, [`op-node`](https://github.com/ethereum-optimism/optimism/tree/develop/op-node) can be run with the
included `mev-rollup.json`:

```bash
cd mev/
op-node \
    --rollup.config ./etc/mev-rollup.json \
    --l1=<your-sepolia-L1-rpc> \
    --l2=http://localhost:9551 \
    --l2.jwt-secret=/path/to/jwt.hex \
    --rpc.addr=0.0.0.0 \
    --rpc.port=7000 \
    --l1.trustrpc
```

### Running MEV with Kurtosis

Running a local network with a full MEV OP stack with Kurtosis requires some extra setup, since MEV uses a forked version of `op-node`.

To get started, follow [these instructions](https://docs.kurtosis.com/install/) to install Kurtosis.

Next, start a Kurtosis enclave:

```bash
kurtosis run --enclave op-devnet github.com/ethpandaops/optimism-package \
  --args-file https://raw.githubusercontent.com/MEV-Network/mev/main/etc/kurtosis.yaml
```

This will start an enclave named `op-devnet`. You can tear down the enclave with `kurtosis enclave rm --force op-devnet`, or tear down all enclaves using `kurtosis clean -a`.

> [!NOTE]
>
> If you want to use a custom build of MEV, simply build an MEV image with `docker build . -t ghcr.io/MEV-Network/mev:latest`.

Consult the [Kurtosis OP package](https://github.com/ethpandaops/optimism-package) repository for instructions on how to adjust the args file to spin up additional services, like a block explorer.

### Wallet extension

MEV has a custom `wallet_` namespace, that allows users to delegate their EOAs to a contract using EIP-7702, and perform transactions on those accounts, all funded by the sequencer.

To enable this namespace, set the environment variable `EXP1_SK` to a private key that will sign the transactions. The new RPC method, `wallet_sendTransaction`, will only sign transactions that either:

1. Designates a contract address to an EOA via EIP-7702, or
1. Send transactions to an EIP-7702 EOA that is already delegated to an address

The `mev_sendTransaction` endpoint accepts the same fields as `eth_sendTransaction`, with these notable exceptions:

1. `nonce` must not be set, as this is managed by the node
1. `value` must be unset or 0
1. `from` must not be specified

The following fields are ignored, as they are overwritten internally:

1. `gasPrice` (and EIP-1559 gas related pricing fields)
1. `gasLimit`
1. `chainId`

### Security

See [SECURITY.md](SECURITY.md).

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>

<!-- [crates-badge]: https://img.shields.io/crates/v/mev.svg -->
<!-- [crates-io]: https://crates.io/crates/mev -->
<!-- [downloads-badge]: https://img.shields.io/crates/d/mev -->

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[apache-badge]: https://img.shields.io/badge/license-Apache--2.0-blue.svg
[mit-url]: LICENSE-MIT
[apache-url]: LICENSE-APACHE
[actions-badge]: https://github.com/MEV-Network/mev/workflows/unit/badge.svg
[actions-url]: https://github.com/MEV-Network/mev/actions?query=workflow%3ACI+branch%3Amain
