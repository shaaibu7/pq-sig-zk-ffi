# Post-Quantum Signature Project

A hybrid Rust/Go implementation for post-quantum cryptographic signatures using XMSS (eXtended Merkle Signature Scheme) with support for signature aggregation and verification via zkVM.

## Overview

This project integrates [leanSig](https://github.com/leanEthereum/leanSig.git) for XMSS-based post-quantum key generation and signing, with plans to incorporate leanMultisig for signature aggregation and verification using a minimal zkVM targeting XMSS operations.

### Architecture

- **Rust Component**: Core cryptographic operations including XMSS key pair generation and signing, exposed via FFI (Foreign Function Interface) as C-compatible functions
- **Go Component**: High-level application logic that interfaces with the Rust library through CGo
- **Future Integration**: leanMultisig zkVM for signature aggregation and recursive verification

## Features

- Post-quantum secure XMSS key pair generation
- Message signing with XMSS signatures
- FFI bridge between Rust and Go
- (Coming Soon) Signature aggregation with leanMultisig zkVM
- (Coming Soon) Recursive signature verification

## Prerequisites

- **Rust**: Latest stable version (install via [rustup](https://rustup.rs/))
- **Go**: Version 1.16 or higher
- **Cargo**: Rust's package manager (comes with Rust installation)
- **GCC/Clang**: C compiler for CGo

## Project Structure

```
.
├── rust/               # Rust library for XMSS operations
│   ├── src/
│   ├── Cargo.toml
│   └── target/
│       └── release/   # Compiled shared library
└── go/                 # Go application
    └── main.go
```

## Building

### Rust Library

Navigate to the Rust project directory and build the release version:

```bash
cd rust
cargo build --release
```

This will generate the shared library in `rust/target/release/`.

## Running

### Go Application

From the Go project directory, run the application with the library path specified:

```bash
cd go
LD_LIBRARY_PATH=../rust/target/release go run main.go
```

The `LD_LIBRARY_PATH` environment variable tells the dynamic linker where to find the Rust-compiled shared library at runtime.

## How It Works

1. The Rust code integrates with leanSig to provide XMSS cryptographic primitives
2. Key pair generation and signing functions are exposed as C-compatible functions using Rust's FFI
3. The Go application uses CGo to call these functions, bridging the gap between the two languages
4. Currently, the system signs an empty message as a proof of concept

## Upcoming Features

### leanMultisig Integration

We are actively adding support for signature aggregation and verification using leanMultisig, a minimal zkVM specifically designed for:

- XMSS signature aggregation
- Recursive proof generation and verification
- Efficient multi-signature schemes with post-quantum security

This will enable multiple XMSS signatures to be aggregated into a compact proof, reducing on-chain storage and verification costs while maintaining post-quantum security guarantees.

## Dependencies

### Rust
- leanSig: XMSS signature library
- (Coming) leanMultisig: zkVM for signature aggregation

### Go
- CGo: For FFI with Rust shared library

## Security Considerations

XMSS is a hash-based signature scheme that provides post-quantum security. However, proper key management is critical:

- **Stateful signatures**: XMSS requires careful state management to avoid key reuse
- **One-time signing**: Each XMSS key has a limited number of signatures
- **Secure storage**: Private keys must be stored securely

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

