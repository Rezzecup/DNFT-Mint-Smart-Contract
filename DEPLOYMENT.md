# 📦 Deployment Guide

This guide provides step-by-step instructions for deploying the dNFT smart contract on various CosmWasm-compatible blockchains.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Building for Production](#building-for-production)
- [Deployment on Terra](#deployment-on-terra)
- [Deployment on Juno](#deployment-on-juno)
- [Deployment on Stargaze](#deployment-on-stargaze)
- [Testing the Deployment](#testing-the-deployment)
- [Troubleshooting](#troubleshooting)

## Prerequisites

Before deploying, ensure you have:

1. **Built and Optimized Contract**
   ```bash
   cargo wasm
   cargo run-script optimize
   ```

2. **Wallet with Funds**
   - For Terra: LUNA tokens
   - For Juno: JUNO tokens
   - For Stargaze: STARS tokens

3. **CLI Tools Installed**
   - Terra: `terrad`
   - Juno: `junod`
   - Stargaze: `starsd`

## Building for Production

### 1. Clean Build

```bash
# Clean previous builds
cargo clean

# Build the contract
cargo wasm

# Verify the build
ls -lh target/wasm32-unknown-unknown/release/*.wasm
```

### 2. Optimize the Contract

```bash
# Using Docker optimizer (recommended)
cargo run-script optimize

# The optimized WASM will be in artifacts/
ls -lh artifacts/*.wasm
```

**Expected Size**: ~150-200 KB (optimized)

### 3. Verify the Build

```bash
# Check WASM validity
cosmwasm-check artifacts/dnft_contracts.wasm

# Output should show: "All checks passed!"
```

## Deployment on Terra

### Testnet (Pisco-1)

```bash
# Set network variables
export CHAIN_ID="pisco-1"
export NODE="https://pisco-lcd.terra.dev"
export DENOM="uluna"
export WALLET="your_wallet_name"

# 1. Store the contract
terrad tx wasm store artifacts/dnft_contracts.wasm \
  --from $WALLET \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 100000$DENOM \
  --broadcast-mode block

# Save the CODE_ID from the output (e.g., 12345)
export CODE_ID=12345

# 2. Instantiate the contract
INIT_MSG='{
  "name": "Derivative NFT Collection",
  "symbol": "DNFT",
  "minter": "terra1your_address_here"
}'

terrad tx wasm instantiate $CODE_ID "$INIT_MSG" \
  --from $WALLET \
  --label "dNFT-v1-testnet" \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 50000$DENOM \
  --admin terra1your_address_here \
  --broadcast-mode block

# Save the CONTRACT_ADDRESS from the output
export CONTRACT_ADDRESS="terra1..."
```

### Mainnet (Phoenix-1)

```bash
# Set network variables
export CHAIN_ID="phoenix-1"
export NODE="https://lcd.terra.dev"
export DENOM="uluna"
export WALLET="your_wallet_name"

# 1. Store the contract
terrad tx wasm store artifacts/dnft_contracts.wasm \
  --from $WALLET \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas 3000000 \
  --gas-adjustment 1.3 \
  --fees 300000$DENOM \
  --broadcast-mode block

# Get CODE_ID from transaction
export CODE_ID=<code_id>

# 2. Instantiate
INIT_MSG='{
  "name": "Your NFT Collection Name",
  "symbol": "YNFT",
  "minter": "terra1your_mainnet_address"
}'

terrad tx wasm instantiate $CODE_ID "$INIT_MSG" \
  --from $WALLET \
  --label "dNFT-v1-mainnet" \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas 200000 \
  --gas-adjustment 1.3 \
  --fees 100000$DENOM \
  --admin terra1your_mainnet_address \
  --broadcast-mode block
```

## Deployment on Juno

### Testnet (Uni-6)

```bash
export CHAIN_ID="uni-6"
export NODE="https://rpc.uni.junonetwork.io:443"
export DENOM="ujunox"
export WALLET="your_wallet_name"

# Store contract
junod tx wasm store artifacts/dnft_contracts.wasm \
  --from $WALLET \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 50000$DENOM \
  --broadcast-mode block

export CODE_ID=<code_id>

# Instantiate
INIT_MSG='{
  "name": "Derivative NFT Collection",
  "symbol": "DNFT",
  "minter": "juno1your_address"
}'

junod tx wasm instantiate $CODE_ID "$INIT_MSG" \
  --from $WALLET \
  --label "dNFT-testnet" \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 30000$DENOM \
  --admin juno1your_address \
  --broadcast-mode block
```

### Mainnet (Juno-1)

```bash
export CHAIN_ID="juno-1"
export NODE="https://rpc-juno.itastakers.com:443"
export DENOM="ujuno"
export WALLET="your_wallet_name"

# Similar to testnet but with mainnet values
junod tx wasm store artifacts/dnft_contracts.wasm \
  --from $WALLET \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas 3000000 \
  --fees 75000$DENOM \
  --broadcast-mode block
```

## Deployment on Stargaze

### Mainnet (Stargaze-1)

```bash
export CHAIN_ID="stargaze-1"
export NODE="https://rpc.stargaze-apis.com:443"
export DENOM="ustars"
export WALLET="your_wallet_name"

# Store contract
starsd tx wasm store artifacts/dnft_contracts.wasm \
  --from $WALLET \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 100000$DENOM \
  --broadcast-mode block

export CODE_ID=<code_id>

# Instantiate
INIT_MSG='{
  "name": "Derivative NFT Collection",
  "symbol": "DNFT",
  "minter": "stars1your_address"
}'

starsd tx wasm instantiate $CODE_ID "$INIT_MSG" \
  --from $WALLET \
  --label "dNFT-v1" \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 50000$DENOM \
  --admin stars1your_address \
  --broadcast-mode block
```

## Testing the Deployment

### 1. Query Contract Info

```bash
# Replace with your CLI tool (terrad/junod/starsd)
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"contract_info":{}}' \
  --chain-id $CHAIN_ID \
  --node $NODE
```

Expected output:
```json
{
  "name": "Derivative NFT Collection",
  "symbol": "DNFT"
}
```

### 2. Query Minter

```bash
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"minter":{}}' \
  --chain-id $CHAIN_ID \
  --node $NODE
```

### 3. Test Mint (as Minter)

```bash
MINT_MSG='{
  "mint": {
    "token_id": "test_1",
    "owner": "terra1your_address",
    "extension": {
      "name": "Test NFT",
      "description": "First test NFT",
      "image": "ipfs://QmTest..."
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$MINT_MSG" \
  --from $WALLET \
  --chain-id $CHAIN_ID \
  --node $NODE \
  --gas auto \
  --fees 20000$DENOM \
  --broadcast-mode block
```

### 4. Query NFT

```bash
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"nft_info":{"token_id":"test_1"}}' \
  --chain-id $CHAIN_ID \
  --node $NODE
```

## Post-Deployment Checklist

- [ ] Saved CODE_ID in secure location
- [ ] Saved CONTRACT_ADDRESS in secure location
- [ ] Verified contract info query works
- [ ] Tested minting functionality
- [ ] Documented deployment addresses
- [ ] Updated frontend configuration (if applicable)
- [ ] Announced deployment to community
- [ ] Set up contract monitoring/alerts

## Troubleshooting

### "out of gas" Error

Increase the gas limit:
```bash
--gas 3000000
```

### "insufficient fees" Error

Increase the fee amount:
```bash
--fees 200000uluna
```

### "contract verification failed"

Ensure you're using the optimized WASM:
```bash
ls -lh artifacts/
cosmwasm-check artifacts/dnft_contracts.wasm
```

### "account sequence mismatch"

Wait a few blocks and retry, or query your account:
```bash
terrad query auth account $(terrad keys show -a $WALLET)
```

### Transaction Not Found

Check if using correct node and chain-id:
```bash
echo $NODE
echo $CHAIN_ID
```

## Security Best Practices

1. **Use Hardware Wallet**: For mainnet deployments
2. **Verify Contract Code**: Before storing, verify the WASM matches your source
3. **Test Thoroughly**: Deploy and test on testnet first
4. **Set Admin Carefully**: Admin can migrate the contract
5. **Monitor Contract**: Set up alerts for unexpected activity
6. **Backup Keys**: Securely backup all deployment information

## Gas Estimates

| Operation | Estimated Gas | Estimated Fee (Terra) |
|-----------|---------------|----------------------|
| Store Contract | 2,500,000 - 3,000,000 | 250,000 - 300,000 uluna |
| Instantiate | 150,000 - 200,000 | 50,000 - 75,000 uluna |
| Mint NFT | 150,000 - 200,000 | 20,000 - 30,000 uluna |
| Transfer NFT | 100,000 - 150,000 | 15,000 - 20,000 uluna |

*Fees may vary by network congestion*

## Support

Need help with deployment?

- 📧 **GitHub**: [@topsecretagent007](https://github.com/topsecretagent007)
- 💬 **Telegram**: [@topsecretagent_007](https://t.me/topsecretagent_007)

---

**Good luck with your deployment! 🚀**

