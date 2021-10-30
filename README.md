# Dfinity Fungible Token Standard

## Tutorial

[https://dft.delandlabs.com/](https://dft.delandlabs.com/)

## How to use rust to create a fungible token with [1 line of code](https://github.com/Deland-Labs/dfinity-fungible-token-standard/blob/86a87b7631c9c075bf02399d75e74de319b8d99d/rust/dft_basic/src/lib.rs#L7)?

```RUST
dft_derive::standard_basic!();
```

## Tools

[DFT issuance tool](https://github.com/Deland-Labs/dft-issuance-tool)


## Compile dependencies

### dfx

```bash
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

### rust

Linux & Mac

1. Install Rust & cmake & optimizer

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install cmake
cargo install ic-cdk-optimizer
```

2. Add wasm32-unknown-unknown target

```bash
rustup target add wasm32-unknown-unknown
```

## How to test?
### Rust
```bash
   cd rust
   make test
```

### Motoko
```bash
   cd motoko
   make test
```

## About us

We are from Deland-Labs team.

We are building a decentralized exchange based on Dfinity with Open Order Protocol.

Offcial Website : [https://delandlabs.com](https://delandlabs.com)