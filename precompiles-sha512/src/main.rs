#![no_main]
#![no_std]

use bobcat_sdk::prelude::*;

use sha2::{digest::Update, Digest, Sha512};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(len: usize) -> usize {
    let mut d = Sha512::new();
    Update::update(&mut d, &read_args_vec(len));
    let x: [u8; 64] = d.finalize().into();
    write_result_slice(&x);
    0
}
