# Avalanche-json-rpc-specification

[View the Documentation](https://lucasespinosa28.github.io/Avalanche-json-rpc-specification/api-documentation/.)

## Overview

This repository is the home of the Avalanche [OpenRPC](https://spec.open-rpc.org) document that describes the JSON-RPC interface with every  methods supportedby public node ``https://api.avax.network/``

⚠ How some methods have different URL paths you need to combine the tag of method with serve URL, the eth methods have tags ``/bc/C/rpc`` the path is https://api.avax.network/ext/bc/P


## Specification

You can view the specification in documentation form [here](https://lucasespinosa28.github.io/Avalanche-json-rpc-specification/api-documentation) or the raw **OpenRPC Document** [here](openrpc.json).

#### Preview

![eth_rpc_playground_docs_demo_eth_rpc](https://user-images.githubusercontent.com/364566/71375336-ba47f980-2572-11ea-9cd5-38c5149c485a.gif)


## Clients

The clients are generated from the **OpenRPC Document** `openrpc.json` in this repository, and can be used as an alternative to web3.js or ethers.js but for various languages:


## Documentation

[View the Documentation](https://lucasespinosa28.github.io/Avalanche-json-rpc-specification/api-documentation).

### Contributing

Proposals to make method changes should be [made as an issue](https://help.github.com/en/articles/creating-an-issue).

How to contribute, build and release are outlined in [CONTRIBUTING.md](CONTRIBUTING.md), [BUILDING.md](BUILDING.md) and [RELEASING.md](RELEASING.md) respectively. Commits in this repository follow the [CONVENTIONAL_COMMITS.md](CONVENTIONAL_COMMITS.md) specification.

## Resources
- https://docs.avax.network/build/avalanchego-apis/platform-chain-p-chain-api
- https://docs.avax.network/build/avalanchego-apis/contract-chain-c-chain-api
- https://docs.avax.network/build/avalanchego-apis/exchange-chain-x-chain-api
