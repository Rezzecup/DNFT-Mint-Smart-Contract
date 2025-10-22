# Changelog

All notable changes to the dNFT Contracts project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-10-22

### Added

#### Core Features
- Initial release of dNFT (Derivative NFT) smart contract
- Full CW721 standard compatibility for NFT operations
- Derivative NFT minting with on-chain provenance tracking
- Automatic ownership verification for derivative creation
- OpenSea-compatible metadata standards

#### Data Structures
- `DerivativeNft` struct for tracking NFT lineage
  - Method field for generation algorithm
  - Parameters field for generation settings
  - Source IDs array for tracking parent NFTs
- `Metadata` struct with comprehensive NFT information
  - Standard fields (name, description, image, etc.)
  - Attributes array for traits
  - Derivative information field

#### Authorization System
- Minter-based authorization for original NFTs
- Source ownership verification for derivative NFTs
- Approval system for token transfers
- Operator permissions for managing multiple tokens

#### Query Functions
- `ContractInfo` - Get contract name and symbol
- `NumTokens` - Get total minted token count
- `NftInfo` - Get NFT metadata and extension data
- `OwnerOf` - Get NFT owner and approvals
- `AllNftInfo` - Get combined NFT info and ownership
- `Tokens` - List tokens owned by an address
- `AllTokens` - List all tokens with pagination
- `Minter` - Get authorized minter address

#### Execute Functions
- `Mint` - Mint original or derivative NFTs
- `TransferNft` - Transfer NFT ownership
- `SendNft` - Transfer with contract hook
- `Approve` - Grant token-specific approval
- `Revoke` - Revoke token-specific approval
- `ApproveAll` - Grant operator status
- `RevokeAll` - Revoke operator status

#### Documentation
- Comprehensive README with usage examples
- Detailed DEPLOYMENT.md guide for multiple chains
- CONTRIBUTING.md with development guidelines
- Inline code documentation with examples
- Deployment scripts and examples

#### Testing
- Unit tests for core functionality
- Integration tests for minting flows
- Error case testing
- Authorization testing

#### CI/CD
- GitHub Actions workflow for automated testing
- Rust formatting checks
- Clippy linting
- WASM build verification
- Code coverage reporting

### Security
- Ownership verification before derivative minting
- Proper authorization checks on all operations
- Protection against duplicate token IDs
- Expiration handling for approvals

### Developer Experience
- Clear error messages with context
- Comprehensive inline documentation
- Example usage in README
- Multiple deployment examples
- Contributing guidelines

## [Unreleased]

### Planned Features
- Multi-chain deployment support (Juno, Stargaze, Osmosis)
- Advanced derivative methods (breeding, evolution)
- Royalty distribution for derivative works
- Marketplace integration
- Frontend dApp
- Graph visualization of derivative chains
- IPFS integration

---

## Release Notes Format

### Types of Changes
- `Added` - New features
- `Changed` - Changes to existing functionality
- `Deprecated` - Soon-to-be removed features
- `Removed` - Removed features
- `Fixed` - Bug fixes
- `Security` - Security improvements

### Version Format
- Major version (X.0.0) - Breaking changes
- Minor version (0.X.0) - New features, backward compatible
- Patch version (0.0.X) - Bug fixes, backward compatible

---

**Project Links:**
- Repository: https://github.com/topsecretagent007/dNFT-contracts
- Issues: https://github.com/topsecretagent007/dNFT-contracts/issues
- Contact: [@topsecretagent_007](https://t.me/topsecretagent_007)

