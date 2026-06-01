# Code Style and Linting Standards

This document describes the code style expectations and linting rules enforced in StarForge.

## Table of Contents

1. [Formatting Standards](#formatting-standards)
2. [Clippy Lint Rules](#clippy-lint-rules)
3. [Code Conventions](#code-conventions)
4. [Pre-Commit Checklist](#pre-commit-checklist)
5. [IDE Integration](#ide-integration)
6. [Fixing Violations](#fixing-violations)

---

## Formatting Standards

### Overview

StarForge uses **standard Rust formatting** via `cargo fmt`. This is non-negotiable and automated.

**Command:**
```bash
cargo fmt --all
```

### Key Rules Enforced

#### Indentation

- **4 spaces** per indentation level (no tabs)
- Applies to: blocks, items, expressions

```rust
// ✅ Correct
fn main() {
    if condition {
        do_something();
    }
}

// ❌ Wrong - uses 2 spaces
fn main() {
  if condition {
    do_something();
  }
}
```

#### Line Length

- **Maximum 100 characters** (enforced by rustfmt)
- Longer lines are automatically wrapped

```rust
// ✅ Correct - wrapped at 100 chars
let result = some_really_long_function_name(arg1, arg2)
    .chain_method()
    .another_method();

// ❌ Wrong - exceeds 100 chars
let result = some_really_long_function_name(arg1, arg2).chain_method().another_method();
```

#### Spacing

**Around operators:**
```rust
// ✅ Correct
let x = a + b;
let y = x * 2;
if x > 0 { }

// ❌ Wrong
let x=a+b;
let y=x*2;
if x>0{}
```

**Around delimiters:**
```rust
// ✅ Correct
fn func(arg1, arg2) { }
let tuple = (a, b, c);
let array = [1, 2, 3];

// ❌ Wrong
fn func( arg1,arg2 ){}
let tuple = (a,b,c);
let array = [1,2,3];
```

#### Imports

Organized by standard library, external crates, then internal modules:

```rust
// ✅ Correct
use std::collections::HashMap;
use std::path::Path;

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::utils::config;
use crate::utils::print as p;

// ❌ Wrong - no organization
use serde::Serialize;
use std::path::Path;
use crate::utils::config;
use clap::Parser;
use std::collections::HashMap;
```

#### Function Signatures

Keep short, but wrap if needed:

```rust
// ✅ Correct - fits on one line
pub fn validate(input: &str) -> Result<bool> {
    // ...
}

// ✅ Correct - wrapped for readability
pub fn long_function_name(
    first_parameter: String,
    second_parameter: u32,
    third_parameter: &Path,
) -> Result<Vec<String>> {
    // ...
}
```

---

## Clippy Lint Rules

### Overview

Clippy is Rust's official linter. StarForge enforces all clippy warnings as errors via:

```bash
cargo clippy --locked -- -D warnings
```

### Categories of Rules

#### 1. Correctness (Never ignore these)

**Unhandled error types:**
```rust
// ❌ Wrong - ignores error
let _ = risky_operation();

// ✅ Correct - handles error
let result = risky_operation()?;
```

**Dereferencing without checks:**
```rust
// ❌ Wrong - may panic
let value = &some_option.unwrap().field;

// ✅ Correct - safe
if let Some(val) = some_option {
    let value = &val.field;
}
```

#### 2. Performance

**Unnecessary cloning:**
```rust
// ❌ Wrong - unnecessary clone
let owned = value.clone();
let reference = &owned;

// ✅ Correct - use reference directly
let reference = &value;
```

**Inefficient data structures:**
```rust
// ❌ Wrong - linear search
for item in vec.iter() {
    if item == target { return true; }
}

// ✅ Correct - use HashSet for lookups
if set.contains(target) { return true; }
```

#### 3. Readability

**Simplifiable conditions:**
```rust
// ❌ Wrong - unnecessary complexity
if x == true {
    do_something();
}

// ✅ Correct - simple and clear
if x {
    do_something();
}
```

**Too many nested conditionals:**
```rust
// ❌ Wrong - hard to read
if a {
    if b {
        if c {
            do_something();
        }
    }
}

// ✅ Correct - early return pattern
if !a { return; }
if !b { return; }
if !c { return; }
do_something();
```

#### 4. Maintainability

**Unused imports:**
```rust
// ❌ Wrong - unused
use std::fs;  // Never used

// ✅ Correct - remove unused
```

**Unused variables:**
```rust
// ❌ Wrong
let unused_var = calculate();
do_something_else();

// ✅ Correct - don't calculate if unused
do_something_else();
```

**Dead code:**
```rust
// ❌ Wrong - never called
#[allow(dead_code)]
fn unused_function() { }

// ✅ Correct - remove or use
fn used_function() { }
```

### Suppressing Clippy Warnings

Only suppress when the warning is genuinely incorrect. Always document why:

```rust
// ✅ Correct - documented suppression
#[allow(clippy::bool_comparison)]
fn check_enabled() -> bool {
    // Comparing to bool is intentional for clarity in this case
    config.is_enabled == true
}

// ❌ Wrong - no explanation
#[allow(clippy::all)]
fn do_something() { }
```

### Common Clippy Warnings in StarForge

| Warning | Meaning | Fix |
|---------|---------|-----|
| `needless_borrow` | Borrowing when not needed | Remove `&` |
| `needless_return` | Return keyword when not needed | Remove return, use value |
| `if_then_some_else_none` | Use `.and_then()` instead | Refactor condition |
| `match_bool` | Matching on bool is verbose | Use if/else |
| `get_first` | Use `.first()` not `.get(0)` | Replace with `.first()` |
| `manual_range_contains` | Use `.contains()` on ranges | Simplify condition |

---

## Code Conventions

### Naming

**Modules:**
```rust
// ✅ Correct - snake_case
mod wallet_manager;
mod network_config;

// ❌ Wrong - CamelCase
mod WalletManager;
mod NetworkConfig;
```

**Functions and variables:**
```rust
// ✅ Correct - snake_case
fn validate_wallet_key() { }
let user_config = Config::new();

// ❌ Wrong - camelCase or PascalCase
fn validateWalletKey() { }
let UserConfig = Config::new();
```

**Constants:**
```rust
// ✅ Correct - SCREAMING_SNAKE_CASE
const MAX_WALLET_COUNT: u32 = 100;
const DEFAULT_NETWORK: &str = "testnet";

// ❌ Wrong - lowercase or camelCase
const maxWalletCount: u32 = 100;
const default_network: &str = "testnet";
```

**Types:**
```rust
// ✅ Correct - PascalCase
struct WalletConfig { }
enum NetworkType { }
trait Validator { }

// ❌ Wrong - snake_case
struct wallet_config { }
enum network_type { }
trait validator { }
```

### Documentation

**All public items need doc comments:**

```rust
// ✅ Correct
/// Creates a new wallet with the given name.
///
/// # Arguments
/// * `name` - The wallet identifier
/// * `network` - The network to associate with
///
/// # Returns
/// A new `WalletEntry` instance
///
/// # Example
/// ```
/// let wallet = Wallet::new("alice", "testnet")?;
/// ```
pub fn new(name: &str, network: &str) -> Result<Wallet> {
    // ...
}

// ❌ Wrong - no documentation
pub fn new(name: &str, network: &str) -> Result<Wallet> {
    // ...
}
```

**Private functions** - add comments for clarity if needed:

```rust
// ✅ Correct - comment explains non-obvious logic
fn validate_key_strength(key: &str) -> bool {
    // Keys must be at least 56 characters for Stellar compatibility
    key.len() >= 56
}

// ❌ Wrong - obvious code doesn't need comments
fn validate_key_strength(key: &str) -> bool {
    // check if key length is at least 56
    key.len() >= 56
}
```

### Error Handling

**Use `Result` for fallible operations:**

```rust
// ✅ Correct - clear error type
pub fn load_config() -> Result<Config, ConfigError> {
    // ...
}

// ✅ Also correct - generic error with context
pub fn load_config() -> anyhow::Result<Config> {
    // ...
}

// ❌ Wrong - no error handling
pub fn load_config() -> Config {
    // ...
}
```

**Use context when propagating errors:**

```rust
// ✅ Correct - adds context
let config = load_config()
    .context("Failed to load wallet configuration")?;

// ❌ Wrong - loses context
let config = load_config()?;
```

### Testing

**Write tests for all public functions:**

```rust
// ✅ Correct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet() {
        let wallet = Wallet::new("alice", "testnet").unwrap();
        assert_eq!(wallet.name, "alice");
    }
}

// ❌ Wrong - untested
pub fn create_wallet(name: &str) -> Wallet {
    // ...
}
```

---

## Pre-Commit Checklist

Before committing code, run:

```bash
# 1. Format code
cargo fmt --all

# 2. Check formatting
cargo fmt --all --check

# 3. Run linter
cargo clippy --locked -- -D warnings

# 4. Run tests
cargo test --locked

# 5. Build
cargo build --locked
```

Or use this one-liner:

```bash
cargo fmt --all && \
  cargo fmt --all --check && \
  cargo clippy --locked -- -D warnings && \
  cargo test --locked && \
  cargo build --locked && \
  echo "✅ All checks passed!"
```

### Git Pre-commit Hook (Optional)

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
set -e

echo "Running pre-commit checks..."

cargo fmt --all --check || {
    echo "❌ Code is not formatted. Run: cargo fmt --all"
    exit 1
}

cargo clippy --locked -- -D warnings || {
    echo "❌ Clippy warnings found. Run: cargo clippy --fix"
    exit 1
}

cargo test --locked > /dev/null || {
    echo "❌ Tests failed"
    exit 1
}

echo "✅ All checks passed"
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

---

## IDE Integration

### VS Code (Rust Analyzer)

**Settings (settings.json):**

```json
{
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.codeActionsOnSave": {
      "source.fixAll.clippy": true
    }
  },
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
  "rust-analyzer.diagnostics.disabled": []
}
```

### IntelliJ IDEA / RustRover

**Settings → Languages & Frameworks → Rust:**
- ✅ Enable Rustfmt
- ✅ Run Rustfmt on Save
- ✅ Clippy as external linter

### Vim / Neovim with rust.vim

**vimrc configuration:**

```vim
let g:rustfmt_autosave = 1
let g:rustfmt_options = '--edition 2021'

" Clippy checks on save
let g:rust_clippy_on_save = 1
```

---

## Fixing Violations

### Formatting Issues

```bash
# Auto-fix all formatting
cargo fmt --all

# Verify it's fixed
cargo fmt --all --check
```

### Clippy Violations

**Option 1: Auto-fix (when available)**
```bash
cargo clippy --fix --allow-dirty --allow-staged
```

**Option 2: Manual fixes**
```bash
# See specific warnings
cargo clippy --locked -- -D warnings

# Read the warning message carefully and fix manually
# Re-run to verify
cargo clippy --locked -- -D warnings
```

**Option 3: Suppress if legitimate**
```rust
#[allow(clippy::rule_name)]
fn function_that_legitimately_needs_this() {
    // Document why suppression is needed
}
```

---

## Troubleshooting

### "error: code must be formatted"

```bash
cargo fmt --all
git add .
git commit -m "style: auto-format code"
```

### "warning: X could be written as Y"

Read the full Clippy message:
```bash
cargo clippy --locked -- -D warnings 2>&1 | grep -A 5 "warning:"
```

Then either:
1. Apply the suggested change
2. Or justify why it's not applicable and suppress

### "tests failed locally but CI passes"

Use same command as CI:
```bash
cargo test --locked
```

Or run sequentially:
```bash
cargo test --locked -- --test-threads=1
```

### IDE shows errors but CLI doesn't

Restart the IDE and/or Rust analyzer:
- VS Code: Command palette → Restart Rust Analyzer
- IntelliJ: File → Invalidate Caches & Restart

---

## Further Reading

- **[CI_ENFORCEMENT.md](CI_ENFORCEMENT.md)** - CI pipeline details
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guidelines
- **[Clippy Documentation](https://doc.rust-lang.org/clippy/)** - Official reference
- **[Rustfmt Documentation](https://rust-lang.github.io/rustfmt/)** - Formatting reference

---

*Last updated: 2026-06-01*  
*Issue #207: Enforce formatting and linting in CI*
