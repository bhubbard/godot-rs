# Contributing to godot-rs

Thank you for your interest in contributing to `godot-rs`! We welcome contributions ranging from bug reports and documentation fixes to new Godot Variant types, math operations, SceneTree improvements, and TSCN parsing capabilities.

---

## Code of Conduct

All contributors and maintainers are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it to understand our community standards.

---

## Development Setup

`godot-rs` is written in pure Rust (2024 edition). You will need:
- Rust toolchain (`stable` 1.85+)
- `cargo`, `rustfmt`, and `clippy`

### Clone and Build

```bash
git clone https://github.com/bhubbard/godot-rs.git
cd godot-rs
cargo build
```

---

## Running Tests and CLI

Always verify that the test suite and CLI demonstrations pass before opening a pull request:

```bash
# Run unit & integration tests
cargo test --all-targets

# Run clippy with strict warnings
cargo clippy --all-targets -- -D warnings

# Run CLI commands
cargo run -- tree
cargo run -- inspect tests/fixtures/test_level.tscn
cargo run -- parse-variant "Vector2(10.5, 20.2)"
cargo run -- benchmark
```

---

## Pull Request Guidelines

1. **Keep Pull Requests Focused**: Limit changes to a single feature or bug fix.
2. **Godot Engine Parity**: Strive for exact numerical and behavioral parity with Godot 4.x engine conventions where appropriate.
3. **Format and Lint**: Run `cargo fmt` and `cargo clippy --all-targets -- -D warnings` before submitting.
4. **Preserve Compatibility**: Keep the public API clean, idiomatic Rust, and zero unsafe code.
