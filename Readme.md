# Solana NoStd Keccak256

[![CI](https://github.com/blueshift-gg/solana-nostd-keccak/actions/workflows/ci.yml/badge.svg)](https://github.com/blueshift-gg/solana-nostd-keccak/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/solana-nostd-keccak.svg)](https://crates.io/crates/solana-nostd-keccak)
[![docs.rs](https://docs.rs/solana-nostd-keccak/badge.svg)](https://docs.rs/solana-nostd-keccak)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/blueshift-gg/solana-nostd-keccak/blob/master/LICENSE)

A more efficient, `no_std` Keccak-256 for the Solana SVM. Routes through the `sol_keccak256` syscall on-chain (~100 CUs for `hashv(&[b"test"])` vs ~121 CUs for `solana_program::keccak::hashv`) and falls through to the `sha3` crate off-chain so the same APIs work in host code.

## Quick start

```toml
[dependencies]
solana-nostd-keccak = "0.2.0"
```

```rust
use solana_nostd_keccak::{hash, hash_ref, hashv};

let a = hash(b"test");
let b = hashv(&[b"hello", b" ", b"world"]);
let c = hash_ref("any AsRef<[u8]>");
```

The library is `#![no_std]`-clean for SBPF; no allocator setup required.

## Features

- Adds `hash_ref` which takes any type that implements `AsRef<[u8]>`
- No `Hash` struct — returns `[u8; 32]` directly
- Uses `MaybeUninit` to skip zero-initializing the output buffer
- `hash_into` lets you hash directly into a pre-allocated buffer

## Static syscalls

If your target supports the Upstream BPF / sBPFv3 static-syscall ABI, enable the `static-syscalls` feature:

```toml
[dependencies]
solana-nostd-keccak = { version = "0.2.0", features = ["static-syscalls"] }
```

The syscall name is murmur3-hashed at compile time and transmuted to a fn pointer, so the SBPF program calls the syscall directly instead of going through an `extern "C"` PLT relocation. No measurable CU difference in our benchmarks — the win is a smaller `.so` and one less relocation for the loader to resolve.

## Benchmarks

On-chain compute unit cost per operation:

| function            | CU cost |
|---------------------|--------:|
| `hashv(&[b"test"])` |     100 |
| `hash(b"test")`     |     105 |
| `hash_ref("test")`  |     105 |

To reproduce, install `cargo build-sbf` (Solana CLI) and run:

```sh
cargo test --test sbpf --jobs 1
```

Sample output (includes the SBPF entrypoint wrapper, ~2 CUs above a raw call):

```
svm_test `bench_hashv`    => 102 CUs
svm_test `bench_hash`     => 106 CUs
svm_test `bench_hash_ref` => 108 CUs
```

The benchmarks compile each function into its own SBPF program and run it through [Mollusk](https://github.com/anza-xyz/mollusk) via [`svm-unit-test`](https://crates.io/crates/svm-unit-test).

## License

Licensed under the [MIT License](https://github.com/blueshift-gg/solana-nostd-keccak/blob/master/LICENSE). The license includes the standard "as-is" warranty disclaimer — use at your own risk.
