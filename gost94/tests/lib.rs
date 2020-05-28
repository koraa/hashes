#![no_std]
#[macro_use]
extern crate digest;
use gost94;

use digest::dev::{digest_test, one_million_a};

new_test!(gost94_test_main, "test", gost94::Gost94Test, digest_test);
new_test!(
    gost94_cryptopro_main,
    "cryptopro",
    gost94::Gost94CryptoPro,
    digest_test
);

#[test]
fn gost94_test_1million_a() {
    let output = include_bytes!("data/test_one_million_a.bin");
    one_million_a::<gost94::Gost94Test>(output);
}

#[test]
fn gost94_cryptopro_1million_a() {
    let output = include_bytes!("data/cryptopro_one_million_a.bin");
    one_million_a::<gost94::Gost94CryptoPro>(output);
}
