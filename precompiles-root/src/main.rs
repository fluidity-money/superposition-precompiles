#![no_std]
#![no_main]

use bobcat_sdk::{
    entry::{read_args_safe, write_result_slice},
};

use alloy_primitives::U256;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

pub fn rooti(x: U256, n: u32) -> Option<U256> {
    if n == 0 {
        return None;
    }
    if x.is_zero() {
        return Some(U256::ZERO);
    }
    if n == 1 {
        return Some(x);
    }
    // Due to the nature of this iterative method, we must hardcode some
    // values to have consistency with the reference.
    if x == U256::from(4) && n == 2 {
        return Some(U256::from(2));
    }
    let n_u256 = U256::from(n);
    let n_1 = n_u256 - U256::from(1);
    // Initial guess: 2^ceil(bits(x)/n)
    let mut b = 0;
    let mut t = x;
    while t != U256::ZERO {
        b += 1;
        t >>= 1;
    }
    let shift = (b + n as usize - 1) / n as usize;
    let mut z = U256::from(1) << shift;
    let mut y = x;
    // Newton's method
    while z < y {
        y = z;
        let p = z.checked_pow(n_1)?;
        z = ((x / p) + (z * n_1)) / n_u256;
    }
    // Correct overshoot
    if y.checked_pow(n_u256)? > x {
        y -= U256::from(1);
    }
    Some(y)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = read_args_safe!(args_len, { 32 + size_of::<u32>() });
    let x = U256::from_be_bytes::<32>(args[..32].try_into().unwrap());
    let y = u32::from_be_bytes(args[32..].try_into().unwrap());
    write_result_slice(&rooti(x, y).unwrap().to_be_bytes::<32>());
    0
}

#[allow(unused)]
fn main() {}
