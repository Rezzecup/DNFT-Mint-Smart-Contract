# 🤝 Contributing to dNFT Contracts

Thank you for your interest in contributing to the dNFT (Derivative NFT) smart contract project! This document provides guidelines and information for contributors.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Enhancements](#suggesting-enhancements)
- [Community](#community)

## 📜 Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all. We expect all participants to:

- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment, trolling, or discriminatory comments
- Publishing others' private information
- Other conduct which could reasonably be considered inappropriate

## 🚀 Getting Started

### Prerequisites

1. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup default stable
   rustup target add wasm32-unknown-unknown
   ```

2. **Install Development Tools**
   ```bash
   cargo install cargo-generate cargo-run-script --features vendored-openssl
   ```

3. **Fork and Clone**
   ```bash
   # Fork the repository on GitHub
   git clone https://github.com/YOUR_USERNAME/dNFT-contracts.git
   cd dNFT-contracts
   ```

4. **Set Up Remote**
   ```bash
   git remote add upstream https://github.com/topsecretagent007/dNFT-contracts.git
   ```

### Development Setup

```bash
# Install dependencies and build
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings
```

## 🔄 Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

Branch naming conventions:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions or updates

### 2. Make Changes

- Write clear, concise commit messages
- Keep commits focused and atomic
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Check code formatting
cargo fmt

# Run linter
cargo clippy
```

### 4. Commit Changes

```bash
git add .
git commit -m "feat: add derivative NFT validation

- Add validation for source NFT existence
- Improve error messages for ownership checks
- Add tests for new validation logic"
```

#### Commit Message Format

```
<type>: <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### 5. Keep Your Branch Updated

```bash
git fetch upstream
git rebase upstream/main
```

### 6. Push Changes

```bash
git push origin feature/your-feature-name
```

## 📝 Coding Standards

### Rust Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

```rust
// Good: Clear, documented, idiomatic Rust
/// Validates that the sender owns all source NFTs for a derivative
/// 
/// # Arguments
/// * `deps` - Dependencies for storage access
/// * `sender` - Address attempting to mint
/// * `source_ids` - List of source NFT token IDs
/// 
/// # Returns
/// * `Ok(())` - If sender owns all source NFTs
/// * `Err(ContractError::Unauthorized)` - If sender doesn't own a source NFT
fn validate_source_ownership(
    deps: Deps,
    sender: &Addr,
    source_ids: &[String],
) -> Result<(), ContractError> {
    for token_id in source_ids {
        let token = self.tokens.load(deps.storage, token_id)?;
        if token.owner != *sender {
            return Err(ContractError::Unauthorized {});
        }
    }
    Ok(())
}
```

### Documentation

- Add doc comments (`///`) for all public items
- Include examples in documentation when helpful
- Document error conditions
- Keep documentation up-to-date with code changes

### Error Handling

```rust
// Good: Descriptive error with context
if !is_authorized {
    return Err(ContractError::Unauthorized {
        reason: "Only minter can mint original NFTs".to_string()
    });
}

// Bad: Generic error
if !is_authorized {
    return Err(ContractError::Unauthorized {});
}
```

### Formatting

```bash
# Format code before committing
cargo fmt

# Check formatting
cargo fmt --check
```

## 🧪 Testing Guidelines

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn test_mint_derivative_success() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("owner", &[]);

        // Setup: Create source NFTs
        // ...

        // Execute: Mint derivative
        // ...

        // Assert: Check results
        // ...
    }

    #[test]
    fn test_mint_derivative_unauthorized() {
        // Test error case
    }
}
```

### Test Coverage

- Write tests for all new features
- Include positive and negative test cases
- Test edge cases and boundary conditions
- Maintain or improve code coverage

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_mint_derivative

# With output
cargo test -- --nocapture --test-threads=1

# Coverage report
cargo tarpaulin --out Html
```

## 🔍 Pull Request Process

### Before Submitting

- [ ] Code follows style guidelines
- [ ] All tests pass locally
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] No linting errors (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Commit messages are clear

### Submitting PR

1. **Push your branch**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create Pull Request** on GitHub

3. **Fill out PR template** with:
   - Description of changes
   - Related issues
   - Testing performed
   - Screenshots (if UI changes)

4. **Request review** from maintainers

### PR Template

```markdown
## Description
Brief description of what this PR does

## Related Issues
Closes #123

## Changes
- Added X feature
- Fixed Y bug
- Updated Z documentation

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All tests pass
- [ ] No linting errors
```

### Review Process

1. At least one maintainer review required
2. All CI checks must pass
3. Address feedback in new commits
4. Once approved, maintainer will merge

## 🐛 Reporting Bugs

### Before Reporting

- Check existing issues for duplicates
- Verify bug exists in latest version
- Collect relevant information

### Bug Report Template

```markdown
**Describe the bug**
Clear description of the bug

**To Reproduce**
Steps to reproduce:
1. Execute message '...'
2. With parameters '...'
3. See error

**Expected behavior**
What you expected to happen

**Actual behavior**
What actually happened

**Environment**
- Chain: Terra/Juno/etc.
- Contract version: v1.0.0
- Rust version: 1.70.0

**Additional context**
Any other relevant information
```

## 💡 Suggesting Enhancements

### Enhancement Template

```markdown
**Feature description**
Clear description of the proposed feature

**Use case**
Why is this feature needed?

**Proposed solution**
How could this be implemented?

**Alternatives considered**
Other approaches you've thought about

**Additional context**
Any other relevant information
```

## 🌐 Community

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and discussions
- **Telegram**: [@topsecretagent_007](https://t.me/topsecretagent_007)

### Getting Help

If you need help:
1. Check existing documentation
2. Search closed issues
3. Ask in GitHub Discussions
4. Contact maintainers on Telegram

## 🏆 Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Credited in relevant documentation

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🙏 Thank You!

Thank you for contributing to dNFT Contracts! Your efforts help build a better ecosystem for derivative NFTs.

---

**Questions?** Contact [@topsecretagent007](https://github.com/topsecretagent007)

