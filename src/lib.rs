//! A more efficient, no_std Keccak-256 for the Solana SVM.
//!
//! On `target_os = "solana"`, hashing routes through the `sol_keccak256`
//! syscall. Off-Solana, it falls through to the `sha3` crate so the same
//! APIs work in host code (tests, off-chain tooling). The Solana
//! implementation costs ~100 CUs for `hashv(&[b"test"])`, vs ~121 CUs for
//! `solana_program::keccak::hashv`.
#![no_std]

use core::mem::MaybeUninit;

#[cfg(not(any(target_arch = "bpf", target_os = "solana")))]
use sha3::{Digest, Keccak256};

/// Length of a Keccak-256 digest, in bytes.
pub const HASH_LENGTH: usize = 32;

#[cfg(all(
    any(target_arch = "bpf", target_os = "solana"),
    not(feature = "static-syscalls")
))]
unsafe extern "C" {
    fn sol_keccak256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;
}

#[cfg(all(
    any(target_arch = "bpf", target_os = "solana"),
    feature = "static-syscalls"
))]
#[inline(always)]
unsafe fn sol_keccak256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64 {
    // murmur3_32(b"sol_keccak256", 0) — precomputed
    const SOL_KECCAK256_ID: usize = 0xd7793abb;
    let syscall: extern "C" fn(*const u8, u64, *mut u8) -> u64 =
        unsafe { core::mem::transmute(SOL_KECCAK256_ID) };
    syscall(vals, val_len, hash_result)
}

/// Hash a single byte slice and return the digest by value.
#[cfg_attr(target_os = "solana", inline(always))]
pub fn hash(data: &[u8]) -> [u8; HASH_LENGTH] {
    hashv(&[data])
}

/// Hash any `T: AsRef<[u8]>` (e.g. `&str`, `&[u8; N]`) and return the digest.
#[inline(always)]
pub fn hash_ref<T: AsRef<[u8]>>(data: T) -> [u8; HASH_LENGTH] {
    hashv(&[data.as_ref()])
}

/// Hash a sequence of byte slices as if they were concatenated, and return
/// the digest. Cheaper than concatenating the inputs yourself.
#[cfg_attr(target_os = "solana", inline(always))]
pub fn hashv(data: &[&[u8]]) -> [u8; HASH_LENGTH] {
    let mut out = MaybeUninit::<[u8; HASH_LENGTH]>::uninit();
    unsafe {
        hash_into(data, out.assume_init_mut());
        out.assume_init()
    }
}

/// Hash `data` directly into the provided 32-byte buffer.
///
/// Use this when you want the digest written into pre-existing storage
/// (e.g. a struct field) without an intermediate move.
#[cfg(not(target_os = "solana"))]
pub fn hash_into(data: &[&[u8]], out: &mut [u8; HASH_LENGTH]) {
    let mut hasher = Keccak256::new();
    for item in data {
        hasher.update(item);
    }
    hasher.finalize_into(out.into());
}

/// Hash `data` directly into the provided 32-byte buffer.
///
/// Use this when you want the digest written into pre-existing storage
/// (e.g. a struct field) without an intermediate move.
#[cfg(target_os = "solana")]
#[inline(always)]
pub fn hash_into(data: &[&[u8]], out: &mut [u8; HASH_LENGTH]) {
    unsafe {
        sol_keccak256(
            data as *const _ as *const u8,
            data.len() as u64,
            out.as_mut_ptr(),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_hash() {
        let h = hash_ref("test");
        let h2 = hashv(&[b"test".as_ref()]);
        assert_eq!(h, h2);
        assert_eq!(
            h2,
            [
                0x9c, 0x22, 0xff, 0x5f, 0x21, 0xf0, 0xb8, 0x1b, 0x11, 0x3e, 0x63, 0xf7, 0xdb, 0x6d,
                0xa9, 0x4f, 0xed, 0xef, 0x11, 0xb2, 0x11, 0x9b, 0x40, 0x88, 0xb8, 0x96, 0x64, 0xfb,
                0x9a, 0x3c, 0xb6, 0x58
            ]
        );
    }
}
