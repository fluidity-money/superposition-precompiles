#![no_std]
#![no_main]

use array_concat::concat_arrays;

use bobcat_sdk::{
    cd::leftpad_bool,
    entry::{write_result_slice, read_args_safe, read_words},
    maths::ruint_mul_div,
};

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = read_args_safe!(args_len, { 32 * 3 });
    let (x, y, z) = read_words!(&args[4..], 3);
    let (a, was_overflow) = ruint_mul_div(x, y, *z).unwrap();
    let b: [u8; 32 * 2] = concat_arrays!(leftpad_bool(was_overflow), a.0);
    write_result_slice(&b);
    0
}

#[allow(unused)]
fn main() {}
