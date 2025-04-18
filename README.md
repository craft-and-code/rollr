# 🎲 Rust Dice Roller

[![Crates.io](https://img.shields.io/crates/v/rollr.svg)](https://crates.io/crates/rollr)
[![Docs.rs](https://docs.rs/rollr/badge.svg)](https://docs.rs/rollr)
![CI](https://github.com/craft-and-code/rollr/actions/workflows/rust.yml/badge.svg)

A small CLI tool to roll dice or flip a coin using Rust.

## Features

- Roll dice in RPG format (e.g., `2D20`, `1D6`)
- Flip a coin with commands like `flipcoin`, `flip`, or `f`
- Only allows valid dice types: D3, D4, D5, D6, D7, D8, D10, D12, D14, D16, D20, D24, D30, D50, D60, D100
- Graceful fallback to default `1D6` if input is missing or malformed

## 🧠 Supported Input Formats

| Input       | Behavior                              |
|-------------|---------------------------------------|
| (none)      | Defaults to rolling `1D6`             |
| `D6`        | Interpreted as `1D6`                  |
| `2D20`      | Rolls two `D20` dice                  |
| `3d100`     | Case-insensitive → `3D100`            |
| `1`         | Interpreted as `1D6`                  |
| `1D`        | Fallback → `1D6`                      |
| `2D99`      | Invalid die type → fallback to `2D6`  |
| `fezfe`     | Malformed → fallback to `1D6`         |

> Dice types not included in the supported `enum` are automatically rejected and fallback to `D6`.

## 📚 Library & CLI

This project is structured as a hybrid **library** + **binary** crate:

It provides:

- A CLI entry point in `src/main.rs`,
- Reusable modules:
  - `dices.rs`: handles parsing and validating dice expressions,
  - `throw.rs`: handles random dice rolls and coin flips.

### 📦 As a library dependency

You can install it with:

```bash
cargo add rollr
```

Or manually add it to your `Cargo.toml`:

```toml
[dependencies]
rollr = "0.1.0"
```

If you’re using the crate locally (e.g. during development), reference it via a local path:

```toml
[dependencies]
rollr = { path = "../path/to/rollr" }
```

### 🧰 As a binary

You can also use `rollr` as a command-line tool without embedding it into another project.

#### 🚀 Running from source:

```bash
cargo run -- 3D20
cargo run -- flip
```

#### 📦 Install locally (for global CLI use):

Install it as a binary:

```bash
cargo install rollr
```

Then use it from anywhere:

```bash
rollr 2D6
rollr flip
```

> Don’t forget to add $HOME/.cargo/bin to your PATH if it’s not already.
