// SPDX-License-Identifier: GPL-2.0

//! Least common multiple (LCM) calculation.

use core::ffi::c_ulong;

/// Calculates the least common multiple (LCM) of two unsigned long values.
///
/// This function is fully compatible with the C API and replaces the
/// function exported as `EXPORT_SYMBOL_GPL(lcm)` on the C side.
///
/// # Safety
///
/// This function is written in pure safe Rust and does not contain any `unsafe` blocks.
#[no_mangle]
pub extern "C" fn lcm(a: c_ulong, b: c_ulong) -> c_ulong {
    if a != 0 && b != 0 {
        a.wrapping_div(super::gcd::gcd(a, b)).wrapping_mul(b)
    } else {
        0
    }
}

/// Calculates the least common multiple (LCM) of two unsigned long values,
/// returning a non-zero value if possible.
///
/// If both values are non-zero, it returns the standard LCM. If one of them is
/// zero, it returns the non-zero value. If both are zero, it returns zero.
///
/// This function is fully compatible with the C API and replaces the
/// function exported as `EXPORT_SYMBOL_GPL(lcm_not_zero)` on the C side.
///
/// # Safety
///
/// This function is written in pure safe Rust and does not contain any `unsafe` blocks.
#[no_mangle]
pub extern "C" fn lcm_not_zero(a: c_ulong, b: c_ulong) -> c_ulong {
    let l = lcm(a, b);
    if l != 0 {
        l
    } else if b != 0 {
        b
    } else {
        a
    }
}
