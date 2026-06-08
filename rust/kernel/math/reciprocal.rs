// SPDX-License-Identifier: GPL-2.0

//! Reciprocal division helper functions.

/// Calculates the reciprocal multiplier and shifts for basic division speedup.
///
/// This function is fully compatible with the C API and replaces the
/// function exported as `EXPORT_SYMBOL(reciprocal_value)` on the C side.
///
/// # Safety
///
/// This function is written in pure safe Rust and does not contain any `unsafe` blocks.
#[no_mangle]
pub extern "C" fn reciprocal_value(d: u32) -> crate::bindings::reciprocal_value {
    let x = d.wrapping_sub(1);
    let l = if x == 0 {
        0
    } else {
        (32 - x.leading_zeros()) as i32
    };

    let mut m = (1u64 << 32).wrapping_mul((1u64.wrapping_shl(l as u32)).wrapping_sub(d as u64));
    m = m.wrapping_div(d as u64);
    m = m.wrapping_add(1);

    crate::bindings::reciprocal_value {
        m: m as u32,
        sh1: core::cmp::min(l, 1) as u8,
        sh2: core::cmp::max(l - 1, 0) as u8,
    }
}

/// Calculates optimized reciprocal division values for JIT code generators.
///
/// This function is fully compatible with the C API and replaces the
/// function exported as `EXPORT_SYMBOL(reciprocal_value_adv)` on the C side.
///
/// # Safety
///
/// This function is written in pure safe Rust and does not contain any `unsafe` blocks.
#[no_mangle]
pub extern "C" fn reciprocal_value_adv(d: u32, prec: u8) -> crate::bindings::reciprocal_value_adv {
    let x = d.wrapping_sub(1);
    let l = if x == 0 {
        0
    } else {
        (32 - x.leading_zeros()) as u32
    };

    if l == 32 {
        crate::pr_warn!(
            "ceil(log2(0x{:08x})) == 32, reciprocal_value_adv doesn't support such divisor\n",
            d
        );
    }

    let mut post_shift = l;
    let mut mlow = 1u64.wrapping_shl(32 + l);
    mlow = mlow.wrapping_div(d as u64);

    let mut mhigh = (1u64.wrapping_shl(32 + l)).wrapping_add(1u64.wrapping_shl(32 + l - prec as u32));
    mhigh = mhigh.wrapping_div(d as u64);

    while post_shift > 0 {
        let lo = mlow >> 1;
        let hi = mhigh >> 1;
        if lo >= hi {
            break;
        }
        mlow = lo;
        mhigh = hi;
        post_shift = post_shift.wrapping_sub(1);
    }

    crate::bindings::reciprocal_value_adv {
        m: mhigh as u32,
        sh: post_shift as u8,
        exp: l as u8,
        is_wide_m: mhigh > u32::MAX as u64,
    }
}
