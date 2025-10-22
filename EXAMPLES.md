# 📚 dNFT Usage Examples

This document provides comprehensive examples for using the dNFT smart contract.

## Table of Contents

- [Basic Minting](#basic-minting)
- [Derivative NFT Examples](#derivative-nft-examples)
- [Transfer Operations](#transfer-operations)
- [Approval Management](#approval-management)
- [Query Examples](#query-examples)
- [Advanced Use Cases](#advanced-use-cases)

## Basic Minting

### Mint a Simple Original NFT

```bash
# Minimal metadata
MINT_MSG='{
  "mint": {
    "token_id": "simple_1",
    "owner": "terra1owner_address",
    "extension": {
      "name": "Simple NFT",
      "image": "ipfs://QmHash123..."
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$MINT_MSG" \
  --from minter_wallet --gas auto --fees 20000uluna
```

### Mint with Full Metadata

```bash
MINT_MSG='{
  "mint": {
    "token_id": "detailed_1",
    "owner": "terra1owner_address",
    "token_uri": "ipfs://QmMetadataHash...",
    "extension": {
      "name": "Detailed Artwork #1",
      "description": "A beautiful piece of digital art with rich metadata",
      "image": "ipfs://QmImageHash...",
      "external_url": "https://myproject.com/nft/1",
      "background_color": "000000",
      "attributes": [
        {
          "trait_type": "Artist",
          "value": "Creator Name"
        },
        {
          "trait_type": "Rarity",
          "value": "Legendary"
        },
        {
          "display_type": "boost_number",
          "trait_type": "Power",
          "value": "100"
        },
        {
          "display_type": "date",
          "trait_type": "Created",
          "value": "1672531200"
        }
      ]
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$MINT_MSG" \
  --from minter_wallet --gas auto --fees 25000uluna
```

### Mint with Animation

```bash
MINT_MSG='{
  "mint": {
    "token_id": "animated_1",
    "owner": "terra1owner_address",
    "extension": {
      "name": "Animated NFT",
      "description": "An NFT with video content",
      "image": "ipfs://QmThumbnail...",
      "animation_url": "ipfs://QmVideo..."
    }
  }
}'
```

## Derivative NFT Examples

### Style Transfer Derivative

Create an NFT by applying style transfer between two original NFTs:

```bash
# User must own both "original_1" and "original_2"
DERIVATIVE_MINT_MSG='{
  "mint": {
    "token_id": "style_transfer_1",
    "owner": "terra1owner_address",
    "extension": {
      "name": "Style Transfer Art #1",
      "description": "Created by applying the style of NFT #2 to NFT #1",
      "image": "ipfs://QmResultHash...",
      "attributes": [
        {
          "trait_type": "Generation Method",
          "value": "Style Transfer"
        },
        {
          "trait_type": "AI Model",
          "value": "VGG19"
        }
      ],
      "derivative": {
        "method": "style_transfer",
        "params": "model:vgg19;intensity:0.8;resolution:1024",
        "source_ids": ["original_1", "original_2"]
      }
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$DERIVATIVE_MINT_MSG" \
  --from owner_wallet --gas auto --fees 30000uluna
```

### Breeding/Fusion Derivative

Combine multiple NFTs to create a new "bred" NFT:

```bash
BREEDING_MSG='{
  "mint": {
    "token_id": "bred_1",
    "owner": "terra1owner_address",
    "extension": {
      "name": "Generation 2 - Bred NFT",
      "description": "Offspring of Parent #1 and Parent #2",
      "image": "ipfs://QmBredHash...",
      "attributes": [
        {
          "trait_type": "Generation",
          "value": "2"
        },
        {
          "trait_type": "Parent 1",
          "value": "parent_nft_1"
        },
        {
          "trait_type": "Parent 2",
          "value": "parent_nft_2"
        },
        {
          "trait_type": "Rarity",
          "value": "Rare"
        }
      ],
      "derivative": {
        "method": "breeding",
        "params": "algorithm:genetic;mutation_rate:0.05",
        "source_ids": ["parent_nft_1", "parent_nft_2"]
      }
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$BREEDING_MSG" \
  --from owner_wallet --gas auto --fees 30000uluna
```

### Multi-Source Composite

Create a composite NFT from multiple sources:

```bash
COMPOSITE_MSG='{
  "mint": {
    "token_id": "composite_1",
    "owner": "terra1owner_address",
    "extension": {
      "name": "Composite Artwork",
      "description": "A composition created from 5 different NFTs",
      "image": "ipfs://QmCompositeHash...",
      "attributes": [
        {
          "trait_type": "Type",
          "value": "Composite"
        },
        {
          "trait_type": "Source Count",
          "value": "5"
        }
      ],
      "derivative": {
        "method": "composite",
        "params": "layout:grid;blend_mode:multiply",
        "source_ids": ["nft_1", "nft_2", "nft_3", "nft_4", "nft_5"]
      }
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$COMPOSITE_MSG" \
  --from owner_wallet --gas auto --fees 35000uluna
```

### AI Generation Derivative

Create an AI-generated derivative:

```bash
AI_DERIVATIVE_MSG='{
  "mint": {
    "token_id": "ai_gen_1",
    "owner": "terra1owner_address",
    "extension": {
      "name": "AI Generated Art #1",
      "description": "Generated using DALL-E with prompts from source NFTs",
      "image": "ipfs://QmAIGenHash...",
      "attributes": [
        {
          "trait_type": "AI Model",
          "value": "DALL-E 3"
        },
        {
          "trait_type": "Style",
          "value": "Surrealism"
        }
      ],
      "derivative": {
        "method": "ai_generation",
        "params": "model:dalle3;style:surrealism;steps:50",
        "source_ids": ["prompt_source_1"]
      }
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$AI_DERIVATIVE_MSG" \
  --from owner_wallet --gas auto --fees 30000uluna
```

## Transfer Operations

### Simple Transfer

```bash
TRANSFER_MSG='{
  "transfer_nft": {
    "recipient": "terra1recipient_address",
    "token_id": "nft_1"
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$TRANSFER_MSG" \
  --from owner_wallet --gas auto --fees 15000uluna
```

### Send to Contract

Send NFT to another contract and trigger a receive hook:

```bash
# The receiving contract must implement Cw721ReceiveMsg
SEND_MSG='{
  "send_nft": {
    "contract": "terra1receiving_contract",
    "token_id": "nft_1",
    "msg": "eyJzb21lIjogImRhdGEifQ=="
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$SEND_MSG" \
  --from owner_wallet --gas auto --fees 20000uluna
```

## Approval Management

### Grant Token Approval

```bash
APPROVE_MSG='{
  "approve": {
    "spender": "terra1spender_address",
    "token_id": "nft_1",
    "expires": {
      "at_height": 1000000
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$APPROVE_MSG" \
  --from owner_wallet --gas auto --fees 15000uluna
```

### Grant Operator Status

```bash
APPROVE_ALL_MSG='{
  "approve_all": {
    "operator": "terra1operator_address",
    "expires": {
      "at_time": "1735689600000000000"
    }
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$APPROVE_ALL_MSG" \
  --from owner_wallet --gas auto --fees 15000uluna
```

### Revoke Approval

```bash
REVOKE_MSG='{
  "revoke": {
    "spender": "terra1spender_address",
    "token_id": "nft_1"
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$REVOKE_MSG" \
  --from owner_wallet --gas auto --fees 15000uluna
```

### Revoke All

```bash
REVOKE_ALL_MSG='{
  "revoke_all": {
    "operator": "terra1operator_address"
  }
}'

terrad tx wasm execute $CONTRACT_ADDRESS "$REVOKE_ALL_MSG" \
  --from owner_wallet --gas auto --fees 15000uluna
```

## Query Examples

### Get Contract Info

```bash
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"contract_info":{}}' --output json | jq
```

Response:
```json
{
  "data": {
    "name": "Derivative NFT Collection",
    "symbol": "DNFT"
  }
}
```

### Get NFT Information

```bash
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"nft_info":{"token_id":"nft_1"}}' --output json | jq
```

Response:
```json
{
  "data": {
    "token_uri": "ipfs://QmHash...",
    "extension": {
      "name": "My NFT",
      "description": "A beautiful NFT",
      "image": "ipfs://QmImageHash...",
      "attributes": [...],
      "derivative": {
        "method": "style_transfer",
        "params": "model:vgg19",
        "source_ids": ["source_1", "source_2"]
      }
    }
  }
}
```

### Get Owner

```bash
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"owner_of":{"token_id":"nft_1"}}' --output json | jq
```

Response:
```json
{
  "data": {
    "owner": "terra1owner_address",
    "approvals": [
      {
        "spender": "terra1spender_address",
        "expires": {"at_height": 1000000}
      }
    ]
  }
}
```

### List Owner's Tokens

```bash
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{
    "tokens": {
      "owner": "terra1owner_address",
      "limit": 30
    }
  }' --output json | jq
```

Response:
```json
{
  "data": {
    "tokens": ["nft_1", "nft_2", "derivative_1"]
  }
}
```

### Get Total Token Count

```bash
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"num_tokens":{}}' --output json | jq
```

Response:
```json
{
  "data": {
    "count": 42
  }
}
```

### List All Tokens

```bash
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{
    "all_tokens": {
      "start_after": "nft_10",
      "limit": 10
    }
  }' --output json | jq
```

### Get Complete NFT Info

```bash
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  '{"all_nft_info":{"token_id":"nft_1"}}' --output json | jq
```

## Advanced Use Cases

### Derivative Chain Query

Query an entire derivative chain:

```bash
# Get initial NFT
NFT_ID="final_derivative"

# Query its info
terrad query wasm contract-state smart $CONTRACT_ADDRESS \
  "{\"nft_info\":{\"token_id\":\"$NFT_ID\"}}" \
  --output json | jq '.data.extension.derivative.source_ids[]' -r | \
while read source_id; do
  echo "Source: $source_id"
  terrad query wasm contract-state smart $CONTRACT_ADDRESS \
    "{\"nft_info\":{\"token_id\":\"$source_id\"}}" \
    --output json | jq '.data.extension.name'
done
```

### Batch Minting

Mint multiple NFTs in sequence:

```bash
for i in {1..10}; do
  MINT_MSG="{
    \"mint\": {
      \"token_id\": \"batch_$i\",
      \"owner\": \"terra1owner_address\",
      \"extension\": {
        \"name\": \"Batch NFT #$i\",
        \"image\": \"ipfs://QmHash$i...\"
      }
    }
  }"
  
  terrad tx wasm execute $CONTRACT_ADDRESS "$MINT_MSG" \
    --from minter_wallet --gas auto --fees 20000uluna -y
    
  sleep 6  # Wait for block confirmation
done
```

### Verify Derivative Ownership

Script to verify user owns all source NFTs before attempting derivative mint:

```bash
#!/bin/bash

USER_ADDRESS="terra1user_address"
SOURCE_IDS=("source_1" "source_2" "source_3")

for token_id in "${SOURCE_IDS[@]}"; do
  OWNER=$(terrad query wasm contract-state smart $CONTRACT_ADDRESS \
    "{\"owner_of\":{\"token_id\":\"$token_id\"}}" \
    --output json | jq -r '.data.owner')
  
  if [ "$OWNER" != "$USER_ADDRESS" ]; then
    echo "Error: User doesn't own $token_id"
    exit 1
  fi
done

echo "Ownership verified! User owns all source NFTs"
```

## Tips and Best Practices

### Gas Estimation

- Simple mint: ~150,000 gas
- Derivative mint: ~200,000 gas (due to ownership checks)
- Transfer: ~100,000 gas
- Query: Free (no gas needed)

### IPFS Best Practices

```bash
# Upload to IPFS first
ipfs add image.png
# Output: QmHash123...

# Use in metadata
"image": "ipfs://QmHash123..."
# or
"image": "https://ipfs.io/ipfs/QmHash123..."
```

### Error Handling

```bash
# Capture and handle errors
RESULT=$(terrad tx wasm execute $CONTRACT_ADDRESS "$MINT_MSG" \
  --from wallet --gas auto --fees 20000uluna --output json 2>&1)

if echo "$RESULT" | grep -q "error"; then
  echo "Transaction failed: $RESULT"
  exit 1
else
  echo "Success! TX: $(echo $RESULT | jq -r '.txhash')"
fi
```

---

**Need more examples?**

Contact [@topsecretagent_007](https://t.me/topsecretagent_007) on Telegram!

