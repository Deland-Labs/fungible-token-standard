# Fungible Token Standard
![build](https://github.com/Deland-Labs/fungible-token-standard/actions/workflows/build.yml/badge.svg)
[![codecov](https://codecov.io/gh/Deland-Labs/fungible-token-standard/branch/main/graph/badge.svg?token=C0594RE316)](https://codecov.io/gh/Deland-Labs/fungible-token-standard)
![integration test](https://github.com/Deland-Labs/fungible-token-standard/actions/workflows/run_integration_test.yml/badge.svg)
[![license](https://img.shields.io/github/license/Deland-Labs/fungible-token-standard)](https://img.shields.io/github/license/Deland-Labs/fungible-token-standard)
[![twitter](https://img.shields.io/twitter/url?label=twitter&style=social&url=https%3A%2F%2Ftwitter.com%2FDelandLabs)](https://twitter.com/DelandLabs)

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/Deland-Labs/fungible-token-standard)
## Tutorial

[https://dft.delandlabs.com/](https://dft.delandlabs.com/)

## Tools

[DFT issuance tool](https://github.com/Deland-Labs/dft-issuance-tool)

## Development

### Dev with Visual Studio Code and Docker

Open `src` folder with Visual Studio Code with `Remote Dev Tools` extension, and load the source code in the container.

everything should be install when container started.

### Dev without container

#### Setup Dev Environment

Prepare the environment as below:

- dfx
- nodejs 16
- python 3
- Rust

run shell script below in `src` dir.

```bash
./sh_post_container_created.sh
```

#### Enable pre-commit hook

```bash
pip install pre-commit
pre-commit install
```

## About us

We are from Deland Labs team.

We are building a decentralized exchange based on Dfinity with Open Order Protocol.

Offcial Website : https://delandlabs.com
