<h1 align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./nomic-logo-dark-100.png">
  <source media="(prefers-color-scheme: light)" srcset="./nomic-logo-100.png">
  <img alt="Nomic" src="./nomic-logo-100.png">
</picture>
</h1>
<p align="center">
<i>Decentralized Custody Engine for Bitcoin</i>
</p>

![CI](https://github.com/nomic-io/nomic/actions/workflows/ci.yml/badge.svg)

Nomic is a blockchain that offers a decentralized custody solution for Bitcoin. Built on Turbofish’s [Orga](https://github.com/turbofish-org/orga), a custom high-performance blockchain application framework. Nomic mints nBTC, a token backed 1:1 with BTC, using [IBC](https://www.ibcprotocol.dev/) for secure and efficient bridging.


## Running a Node

Running a node increases the health of the network by decentralizing ledger validation and data, even for non-validator nodes. Community members are encouraged to run a node, especially when regularly interacting with the network via transactions and queries.

[Nomic Network Docs](https://docs.nomic.io/)

## Integrating with Nomic

Integrating with nBTC enables accepting Bitcoin deposits with Interchain Deposits to any IBC-enabled blockchain.

[nBTC Docs](https://github.com/nomic-io/nomic-bitcoin-js/blob/main/README.md)

## Contributing

Nomic is an open-source project spearheaded by contributors. Anyone is able to contribute to Nomic via GitHub.

[Contribute to Nomic](https://github.com/nomic-io/nomic/contribute)

## Security

Nomic is currently undergoing security audits.

Vulnerabilities should not be reported through public channels, including GitHub Issues. You can report a vulnerability via GitHub's Private Vulnerability Reporting or via the Nomic DAO Foundation at `foundation@nomic.io`.

[Report a Vulnerability](https://github.com/nomic-io/nomic/security/advisories/new)


## License

Licensed under the Apache License, Version 2.0 (the "License"); you may not use the files in this repository except in compliance with the License. You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.


---

Copyright © 2024 Nomic DAO Foundation.

## Post-Quantum Cryptography (PQC) Support
This repository now implements post-quantum cryptographic standards to ensure future resilience of the ledger mapping.

The `timechain` library utilizes NIST-standard algorithms for ensuring security against Shor's algorithm:
- **ML-KEM (Kyber1024)** for KEM operations across communication nodes.
- **ML-DSA (Dilithium5)** for digital signature operations over TimeBlock hashes.
- **SVD Compression**: Advanced lattice reduction mechanisms are applied to compress keys and signatures using Singular Value Decomposition across the topology.

For benchmarking over network clusters, try running `cargo run --bin simulator_pqc`.
