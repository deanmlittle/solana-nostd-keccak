use core::mem::MaybeUninit;

#[cfg(not(target_os = "solana"))]
use sha3::{Digest, Keccak256};

pub const HASH_LENGTH: usize = 32;

#[cfg(target_os = "solana")]
extern "C" {
    fn sol_keccak256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;
}

#[cfg_attr(target_os = "solana", inline(always))]
pub fn hash(data: &[u8]) -> [u8;HASH_LENGTH] {
    hashv(&[data])
}

#[inline(always)]
pub fn hash_ref<T: AsRef<[u8]>>(data: T) -> [u8;HASH_LENGTH] {
    hashv(&[data.as_ref()])
}

#[cfg(not(target_os = "solana"))]
pub fn hashv(data: &[&[u8]]) -> [u8; HASH_LENGTH] {
    let mut out = MaybeUninit::<[u8; HASH_LENGTH]>::uninit();
    unsafe {
        hash_into(data, out.assume_init_mut());
        out.assume_init()
    }
}

#[cfg(target_os = "solana")]
#[inline(always)]
pub fn hashv(data: &[&[u8]]) -> [u8; HASH_LENGTH] {
    let mut out = MaybeUninit::<[u8; HASH_LENGTH]>::uninit();
    unsafe {
        hash_into(data, out.as_mut_ptr());
        out.assume_init()
    }
}

#[cfg(not(target_os = "solana"))]
pub fn hash_into(data: &[&[u8]], out: &mut [u8; HASH_LENGTH]) {
    let mut hasher = Keccak256::new();
    for item in data {
        hasher.update(item);
    }
    hasher.finalize_into(out.into());
}

#[cfg(target_os = "solana")]
#[inline(always)]
pub fn hash_into(data: &[&[u8]], out: *mut [u8; 32]) {
    unsafe {
        sol_keccak256(
            data as *const _ as *const u8,
            data.len() as u64,
            out as *mut u8,
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
        assert_eq!(h2, [0x9c, 0x22, 0xff, 0x5f, 0x21, 0xf0, 0xb8, 0x1b, 0x11, 0x3e, 0x63, 0xf7, 0xdb, 0x6d, 0xa9, 0x4f, 0xed, 0xef, 0x11, 0xb2, 0x11, 0x9b, 0x40, 0x88, 0xb8, 0x96, 0x64, 0xfb, 0x9a, 0x3c, 0xb6, 0x58]);
    }
}