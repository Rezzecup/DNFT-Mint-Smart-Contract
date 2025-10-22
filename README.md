# 🎨 dNFT: Derivative NFT Smart Contract

A CosmWasm smart contract implementation for creating and managing derivative NFTs on Cosmos-based blockchains (Terra, Juno, etc.). This contract extends the CW721 standard to support NFTs that are derived from one or more source NFTs, maintaining full provenance and derivative chain information on-chain.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CosmWasm](https://img.shields.io/badge/CosmWasm-v0.16-blue)](https://cosmwasm.com/)
[![CW721](https://img.shields.io/badge/CW721-Compatible-green)](https://github.com/CosmWasm/cw-nfts)

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [How It Works](#how-it-works)
- [Architecture](#architecture)
- [Getting Started](#getting-started)
- [Building](#building)
- [Deployment](#deployment)
- [Usage Examples](#usage-examples)
- [Smart Contract Interface](#smart-contract-interface)
- [Testing](#testing)
- [Contributing](#contributing)
- [Contact](#contact)
- [License](#license)

## 🌟 Overview

**dNFT** (Derivative NFT) is a revolutionary smart contract that enables the creation of NFTs derived from existing NFTs. This creates a provable chain of derivative works stored entirely on-chain, perfect for:

- 🎨 **AI-Generated Art**: Style transfer, image morphing, and AI art generation
- 🖼️ **NFT Remixes**: Create derivative works from existing NFT collections
- 🧬 **Breeding/Fusion**: Combine multiple NFTs to create new ones
- 📊 **Composite Assets**: Merge multiple NFTs into a single derivative
- 🔗 **Provenance Tracking**: Maintain complete on-chain history of NFT lineage

## ✨ Features

### Core Functionality

- **✅ CW721 Compatible**: Fully compatible with the CW721 NFT standard
- **🔄 Derivative Minting**: Create new NFTs from existing ones
- **🔐 Ownership Verification**: Automatic verification that minter owns source NFTs
- **📝 On-chain Metadata**: Store complete derivation information on-chain
- **🔍 Provenance Tracking**: Track the complete lineage of derivative NFTs
- **🛡️ Access Control**: Role-based minting permissions

### Metadata Features

Supports OpenSea metadata standards plus derivative-specific fields:

```rust
{
  "image": "ipfs://...",
  "description": "A derivative NFT created by combining two original pieces",
  "name": "Derivative Art #1",
  "attributes": [...],
  "derivative": {
    "method": "styletransfer",           // Algorithm used
    "params": "model:vgg19;intensity:0.8", // Generation parameters
    "source_ids": ["nft1", "nft2"]        // Source NFT IDs
  }
}
```

## 🔧 How It Works

### Original NFT Minting

Only the designated **minter** (set during contract instantiation) can mint original NFTs:

```
1. Minter calls Mint() with metadata
2. Contract verifies sender is authorized minter
3. NFT is minted to specified owner
```

### Derivative NFT Minting

**Any NFT owner** can mint derivative NFTs if they own all source NFTs:

```
1. Owner calls Mint() with derivative metadata
2. Contract checks if derivative field is present
3. Contract verifies sender owns ALL source NFTs listed
4. Derivative NFT is minted to specified owner
```

This enables permissionless derivative creation while ensuring proper ownership rights!

## 🏗️ Architecture

```
dNFT-contracts/
├── src/
│   ├── lib.rs              # Entry points (instantiate, execute, query)
│   ├── contract_tests.rs   # Unit tests
│   ├── error.rs            # Error definitions
│   ├── execute.rs          # Execute logic (mint, transfer, approve)
│   ├── msg.rs              # Message types and metadata structures
│   ├── query.rs            # Query handlers
│   └── state.rs            # State management and storage
├── packages/
│   └── cw721/              # CW721 base implementation
├── schema/                 # JSON schemas for messages
├── artifacts/              # Compiled WASM binaries
├── Cargo.toml              # Rust dependencies
└── README.md               # This file
```

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.55+ - [Install Rust](https://www.rust-lang.org/tools/install)
- **wasm32 target**: `rustup target add wasm32-unknown-unknown`
- **cargo-generate**: `cargo install cargo-generate`
- **cargo-run-script**: `cargo install cargo-run-script --features vendored-openssl`
- **Docker** (for optimization) - [Install Docker](https://docs.docker.com/get-docker/)

### Installation

```bash
# Clone the repository
git clone https://github.com/topsecretagent007/dNFT-contracts.git
cd dNFT-contracts

cargo build
```

## 🔨 Building

### Development Build

```bash
cargo wasm

```

### Production Build (Optimized)

```bash
cargo wasm
cargo run-script optimize

```

### Build Schemas

```bash
cargo schema

```

## 📦 Deployment

### 1. Store the Contract

```bash
terrad tx wasm store artifacts/cw721terra.wasm \
  --from wallet \
  --chain-id phoenix-1 \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 100000uluna

```

### 2. Instantiate the Contract

```bash
INIT_MSG='{
  "name": "Derivative NFT Collection",
  "symbol": "DNFT",
  "minter": "terra1your_address_here"
}'

terrad tx wasm instantiate $CODE_ID "$INIT_MSG" \
  --from wallet \
  --label "dNFT-v1" \
  --chain-id phoenix-1 \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 50000uluna \
  --admin terra1your_address_here

```

### 3. Verify Deployment

```bash
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"contract_info":{}}'
```

## 📖 Usage Examples

### Mint an Original NFT (Minter Only)

```bash
MINT_MSG='{
  "mint": {
    "token_id": "original_1",
    "owner": "terra1owner_address",
    "token_uri": "ipfs://QmHash...",
    "extension": {
      "name": "Original Artwork #1",
      "description": "An original piece of digital art",
      "image": "ipfs://QmImageHash...",
      "attributes": [
        {
          "trait_type": "Artist",
          "value": "Creator Name"
        }
      ]
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$MINT_MSG" \
  --from minter_wallet \
  --chain-id phoenix-1 \
  --gas auto \
  --fees 20000uluna
```

### Mint a Derivative NFT (Any Owner)

```bash
# User must own both "original_1" and "original_2"
DERIVATIVE_MINT_MSG='{
  "mint": {
    "token_id": "derivative_1",
    "owner": "terra1owner_address",
    "token_uri": "ipfs://QmDerivativeHash...",
    "extension": {
      "name": "Derivative Art #1",
      "description": "A fusion of two original pieces",
      "image": "ipfs://QmDerivedImageHash...",
      "derivative": {
        "method": "style_transfer",
        "params": "model:vgg19;intensity:0.8;resolution:1024",
        "source_ids": ["original_1", "original_2"]
      },
      "attributes": [
        {
          "trait_type": "Generation Method",
          "value": "AI Style Transfer"
        }
      ]
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$DERIVATIVE_MINT_MSG" \
  --from owner_wallet \
  --chain-id phoenix-1 \
  --gas auto \
  --fees 25000uluna
```

### Query NFT Information

```bash
# Get NFT info with metadata
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"nft_info":{"token_id":"derivative_1"}}'

# Get owner and approvals
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"owner_of":{"token_id":"derivative_1"}}'

# Get all NFT info at once
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"all_nft_info":{"token_id":"derivative_1"}}'

# List all tokens owned by address
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{
    "tokens": {
      "owner": "terra1owner_address",
      "limit": 30
    }
  }'

# Get total number of minted tokens
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"num_tokens":{}}'
```

### Transfer NFT

```bash
TRANSFER_MSG='{
  "transfer_nft": {
    "recipient": "terra1recipient_address",
    "token_id": "derivative_1"
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$TRANSFER_MSG" \
  --from owner_wallet \
  --chain-id phoenix-1 \
  --gas auto \
  --fees 15000uluna
```

## 📝 Smart Contract Interface

### Execute Messages

| Message | Description | Authorized |
|---------|-------------|------------|
| `Mint` | Mint new NFT (original or derivative) | Minter (original) or Source Owner (derivative) |
| `TransferNft` | Transfer NFT to another address | Owner or Approved |
| `SendNft` | Transfer NFT and trigger receive hook | Owner or Approved |
| `Approve` | Grant approval for specific token | Owner or Operator |
| `Revoke` | Revoke specific token approval | Owner or Operator |
| `ApproveAll` | Grant approval for all tokens | Owner |
| `RevokeAll` | Revoke all token approvals | Owner |

### Query Messages

| Query | Description | Returns |
|-------|-------------|---------|
| `ContractInfo` | Get contract name and symbol | `ContractInfoResponse` |
| `NumTokens` | Get total token count | `NumTokensResponse` |
| `NftInfo` | Get NFT metadata | `NftInfoResponse` |
| `OwnerOf` | Get NFT owner and approvals | `OwnerOfResponse` |
| `AllNftInfo` | Get both NFT info and ownership | `AllNftInfoResponse` |
| `Tokens` | List tokens owned by address | `TokensResponse` |
| `AllTokens` | List all tokens (paginated) | `TokensResponse` |
| `Minter` | Get authorized minter address | `MinterResponse` |

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_mint_derivative

# Check code coverage
cargo tarpaulin --out Html
```

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run linter before committing: `cargo clippy`
- Ensure all tests pass: `cargo test`

## 📞 Contact

- 📧 **GitHub**: [@topsecretagent007](https://github.com/topsecretagent007)
- 💬 **Telegram**: [@topsecretagent_007](https://t.me/topsecretagent_007)
- 🚀 **Mint Contract**: [DNFT-Mint-Smart-Contract](https://github.com/topsecretagent007/DNFT-Mint-Smart-Contract)
