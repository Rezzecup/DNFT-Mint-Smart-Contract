# ⚡ Quick Start Guide

Get up and running with dNFT contracts in 5 minutes!

## 🎯 Prerequisites

- Rust installed: https://rustup.rs/
- Docker (for optimization): https://docs.docker.com/get-docker/
- CLI tool for your chain (terrad/junod/starsd)

## 📥 Installation

```bash
# Clone the repository
git clone https://github.com/topsecretagent007/dNFT-contracts.git
cd dNFT-contracts

# Build the contract
cargo wasm

# Run tests
cargo test
```

## 🔨 Build for Deployment

```bash
# Optimize the WASM
cargo run-script optimize

# Check the output
ls -lh artifacts/
```

## 🚀 Deploy to Testnet

```bash
# Set your variables
export WALLET="your_wallet_name"
export CONTRACT_ADDRESS=""

# Store the contract
terrad tx wasm store artifacts/dnft_contracts.wasm \
  --from $WALLET \
  --chain-id pisco-1 \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 100000uluna

# Note the CODE_ID from output, then instantiate
terrad tx wasm instantiate <CODE_ID> \
  '{"name":"My dNFT Collection","symbol":"DNFT","minter":"terra1..."}' \
  --from $WALLET \
  --label "dNFT-test" \
  --chain-id pisco-1 \
  --gas auto \
  --fees 50000uluna \
  --admin terra1...
```

## 🎨 Mint Your First NFT

```bash
# Mint an original NFT (as minter)
terrad tx wasm execute $CONTRACT_ADDRESS \
  '{
    "mint": {
      "token_id": "nft_1",
      "owner": "terra1...",
      "extension": {
        "name": "My First dNFT",
        "image": "ipfs://QmHash..."
      }
    }
  }' \
  --from $WALLET \
  --gas auto \
  --fees 20000uluna
```

## 🎯 Create a Derivative NFT

```bash
# Mint a derivative (must own source NFTs)
terrad tx wasm execute $CONTRACT_ADDRESS \
  '{
    "mint": {
      "token_id": "derivative_1",
      "owner": "terra1...",
      "extension": {
        "name": "My Derivative",
        "image": "ipfs://QmNewHash...",
        "derivative": {
          "method": "style_transfer",
          "params": "model:vgg19",
          "source_ids": ["nft_1"]
        }
      }
    }
  }' \
  --from $WALLET \
  --gas auto \
  --fees 25000uluna
```

## 🔍 Query NFT Info

```bash
# Get NFT metadata
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"nft_info":{"token_id":"nft_1"}}'

# Get owner
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"owner_of":{"token_id":"nft_1"}}'

# List all tokens
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"all_tokens":{"limit":30}}'
```

## 📚 Next Steps

- Read the full [README.md](README.md) for detailed information
- Check [EXAMPLES.md](EXAMPLES.md) for more use cases
- See [DEPLOYMENT.md](DEPLOYMENT.md) for mainnet deployment
- Review [CONTRIBUTING.md](CONTRIBUTING.md) to contribute

## 🆘 Need Help?

- 📧 GitHub: [@topsecretagent007](https://github.com/topsecretagent007)
- 💬 Telegram: [@topsecretagent_007](https://t.me/topsecretagent_007)

---

**Happy building! 🚀**

