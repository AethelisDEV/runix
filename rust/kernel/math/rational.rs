// SPDX-License-Identifier: GPL-2.0

//! Rational approximation calculation.

use core::ffi::c_ulong;

/// Calculates the best rational approximation for a given fraction.
///
/// This function is fully compatible with the C API and replaces the
/// function exported as `EXPORT_SYMBOL(rational_best_approximation)` on the C side.
///
/// # Safety
///
/// The caller must ensure that `best_numerator` and `best_denominator` are
/// valid, non-null, writable pointers to `c_ulong`.
#[no_mangle]
pub extern "C" fn rational_best_approximation(
    given_numerator: c_ulong,
    given_denominator: c_ulong,
    max_numerator: c_ulong,
    max_denominator: c_ulong,
    best_numerator: *mut c_ulong,
    best_denominator: *mut c_ulong,
) {
    let mut n = given_numerator;
    let mut d = given_denominator;
    let mut n0: c_ulong = 0;
    let mut d1: c_ulong = 0;
    let mut n1: c_ulong = 1;
    let mut d0: c_ulong = 1;

    loop {
        if d == 0 {
            break;
        }

        let dp = d;
        let a = n.wrapping_div(d);
        d = n.wrapping_rem(d);
        n = dp;

        let n2 = n0.wrapping_add(a.wrapping_mul(n1));
        let d2 = d0.wrapping_add(a.wrapping_mul(d1));

        if n2 > max_numerator || d2 > max_denominator {
            let mut t = c_ulong::MAX;

            if d1 != 0 {
                t = max_denominator.wrapping_sub(d0).wrapping_div(d1);
            }
            if n1 != 0 {
                t = core::cmp::min(t, max_numerator.wrapping_sub(n0).wrapping_div(n1));
            }

            let t_mul_2 = t.wrapping_mul(2);
            if d1 == 0 || t_mul_2 > a || (t_mul_2 == a && d0.wrapping_mul(dp) > d1.wrapping_mul(d)) {
                n1 = n0.wrapping_add(t.wrapping_mul(n1));
                d1 = d0.wrapping_add(t.wrapping_mul(d1));
            }
            break;
        }

        n0 = n1;
        n1 = n2;
        d0 = d1;
        d1 = d2;
    }

    // SAFETY: We verify that the destination pointers are not null before dereferencing and writing.
    unsafe {
        if !best_numerator.is_null() {
            *best_numerator = n1;
        }
        if !best_denominator.is_null() {
            *best_denominator = d1;
        }
    }
}
