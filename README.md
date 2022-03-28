# Dfinity Fungible Token Standard

## Tutorial

[https://dft.delandlabs.com/](https://dft.delandlabs.com/)

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
**Unit Testing**
```bash
   cd src
   cargo test
```
**Automated Integration Testing**
```bash
   cd src
   sh sh_go.sh
   sh sh_setup_dev.sh
   sh itt.sh
```

## About us

We are from Deland-Labs team.

We are building a decentralized exchange based on Dfinity with Open Order Protocol.

Offcial Website : [https://delandlabs.com](https://delandlabs.com)