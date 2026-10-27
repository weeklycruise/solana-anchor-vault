# Solana Anchor Vault Protocol

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Solana](https://img.shields.io/badge/Solana-Anchor%200.30-14F195?logo=solana)](https://solana.com)
[![Rust](https://img.shields.io/badge/Rust-1.79+-orange.svg?logo=rust)](https://www.rust-lang.org)

An audited, high-throughput non-custodial multi-token vault and programmatic escrow protocol deployed on the Solana blockchain. Engineered using the Anchor framework, featuring program-derived address (PDA) authority derivation, zero-copy deserialization, and strict re-entrancy prevention.

---

## 🏛 Architecture Overview

```mermaid
graph TD
    User([User / Depositor]) -->|Deposit SPL Token / SOL| VaultPDA[Vault PDA Account]
    VaultPDA -->|State Tracking| VaultState[Vault State & Merkle Root]
    User -->|Initiate Time-Locked Claim| EscrowGate[Escrow Settlement Gate]
    EscrowGate -->|Verify Proof & Timelock| VaultPDA
    VaultPDA -->|Disburse Assets| Beneficiary([Beneficiary / Claimer])
```

## ⚡ Key Features

- **Program-Derived Escrow (PDA)**: Automated escrow management with deterministic seeds ensuring multi-sig or timelocked custody without private key reliance.
- **SPL Token 2022 Compatibility**: Full support for native SOL, standard SPL Tokens, and Token-2022 extensions (transfer hooks, confidential transfers).
- **Time-Weighted Vesting**: Configurable linear and cliff vesting curves for programmatic team and community token distributions.
- **Re-entrancy Guard**: Cryptographic single-execution state locks preventing cross-CPI state manipulation.

## 📦 Directory Structure

```text
├── Anchor.toml
├── Cargo.toml
├── package.json
├── programs/
│   └── solana-anchor-vault/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
└── tests/
    └── vault.ts
```

## 🛠 Local Development & Testing

```bash
# 1. Install Anchor CLI & Solana Toolsuite
avm install 0.30.0
avm use 0.30.0

# 2. Build smart contract binary
anchor build

# 3. Execute comprehensive integration test suite
anchor test
```

## 📄 License

Licensed under the MIT License.
