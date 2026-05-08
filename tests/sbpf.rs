use solana_nostd_keccak::{hash, hash_ref, hashv};
use svm_unit_test::svm_test;

const TEST: [u8; 4] = *b"test";

#[svm_test]
fn bench_hashv() {
    hashv(&[&TEST]);
}

#[svm_test]
fn bench_hash() {
    hash(&TEST);
}

#[svm_test]
fn bench_hash_ref() {
    hash_ref(TEST);
}
