// SPDX-License-Identifier: GPL-2.0

//! Integer exponentiation.

/// Computes the exponentiation of the given base and exponent.
///
/// This function is fully compatible with the C API and replaces the
/// function exported as `EXPORT_SYMBOL_GPL(int_pow)` on the C side.
///
/// # Safety
///
/// This function is written in pure safe Rust and does not contain any `unsafe` blocks.
#[no_mangle]
pub extern "C" fn int_pow(mut base: u64, mut exp: core::ffi::c_uint) -> u64 {
    let mut result: u64 = 1;

    while exp != 0 {
        if (exp & 1) != 0 {
            result = result.wrapping_mul(base);
        }
        exp >>= 1;
        base = base.wrapping_mul(base);
    }

    result
}
