// SPDX-License-Identifier: GPL-2.0

//! Integer square root calculation.

use core::ffi::c_ulong;

/// Computes the integer square root (floor(sqrt(x))).
///
/// This function is fully compatible with the C API and replaces the
/// function exported as `EXPORT_SYMBOL(int_sqrt)` on the C side.
///
/// # Safety
///
/// This function is written in pure safe Rust and does not contain any `unsafe` blocks.
#[no_mangle]
pub extern "C" fn int_sqrt(x: c_ulong) -> c_ulong {
    if x <= 1 {
        return x;
    }

    // The expression (usize::BITS - 1) - x.leading_zeros() yields the equivalent of __fls(x).
    // Since x >= 2 is guaranteed at this point, leading_zeros() is always less than usize::BITS - 1.
    // Thus, fls is a non-negative integer and there is no risk of underflow.
    let fls = (usize::BITS - 1).wrapping_sub(x.leading_zeros() as u32);
    let mut m = 1usize.wrapping_shl(fls & !1) as c_ulong;
    let mut y: c_ulong = 0;
    let mut x_val = x;

    while m != 0 {
        let b = y.wrapping_add(m);
        y >>= 1;

        if x_val >= b {
            x_val = x_val.wrapping_sub(b);
            y = y.wrapping_add(m);
        }
        m >>= 2;
    }

    y
}

/// Computes the integer square root when a minimum 64-bit input is expected on 32-bit architectures.
///
/// This function is fully compatible with the C API and replaces the
/// function exported as `EXPORT_SYMBOL(int_sqrt64)` on the C side.
///
/// # Safety
///
/// This function is written in pure safe Rust and does not contain any `unsafe` blocks.
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub extern "C" fn int_sqrt64(x: u64) -> u32 {
    if x <= (c_ulong::MAX as u64) {
        return int_sqrt(x as c_ulong) as u32;
    }

    let fls = 63.wrapping_sub(x.leading_zeros());
    let mut m = 1u64.wrapping_shl(fls & !1);
    let mut y: u64 = 0;
    let mut x_val = x;

    while m != 0 {
        let b = y.wrapping_add(m);
        y >>= 1;

        if x_val >= b {
            x_val = x_val.wrapping_sub(b);
            y = y.wrapping_add(m);
        }
        m >>= 2;
    }

    y as u32
}
