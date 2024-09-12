# Solana NoStd Keccak256

A more efficient implementation of Keccak256 for SVM.

# Installation

```cargo add solana-nostd-keccak```

# Features

- Adds `hash_ref` which takes in any type that implements `<AsRef<[u8]>>`
- No `Hash` struct. Returns `[u8;32]` directly.
- Makes use of MaybeUninit to skip zero allocations
- Adds `hash_into` to let you hash directly into a mutable buffer.

# Performance

| library        | function          | CU cost |
|----------------|-------------------|---------|
| nostd-keccak   | hashv(&[b"test"]) | 100     |
| nostd-keccak   | hash(b"test")     | 105     |
| nostd-keccak   | hash_ref("test")  | 105     |
| solana-program | hashv(&[b"test"]) | 121     |
| solana-program | hash(b"test")     | 123     |