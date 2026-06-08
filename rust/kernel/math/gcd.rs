// SPDX-License-Identifier: GPL-2.0

//! Greatest common divisor (GCD) calculation.

use core::ffi::c_ulong;

/// Calculates the greatest common divisor (GCD) of two unsigned long values.
///
/// This function is fully compatible with the C API and replaces the
/// function exported as `EXPORT_SYMBOL_GPL(gcd)` on the C side.
///
/// # Safety
///
/// This function is written in pure safe Rust and does not contain any `unsafe`
/// blocks, pointer dereferences, or undefined behavior risks.
#[no_mangle]
pub extern "C" fn gcd(mut a: c_ulong, mut b: c_ulong) -> c_ulong {
    let r = a | b;

    if a == 0 || b == 0 {
        return r;
    }

    // Get the number of trailing zeros to find common factors of 2
    let shift = r.trailing_zeros();

    // Divide b by 2 until it is odd
    b >>= b.trailing_zeros();

    loop {
        // Divide a by 2 until it is odd
        a >>= a.trailing_zeros();

        if a == b {
            return a << shift;
        }

        if a < b {
            core::mem::swap(&mut a, &mut b);
        }

        a = a.wrapping_sub(b);
    }
}
