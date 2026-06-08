// SPDX-License-Identifier: GPL-2.0

//! CORDIC algorithm helper functions.

const CORDIC_ANGLE_GEN: i32 = 39797;
const CORDIC_PRECISION_SHIFT: u32 = 16;
const CORDIC_NUM_ITER: usize = 18;

const ARCTAN_TABLE: [i32; 18] = [
    2949120, 1740967, 919879, 466945, 234379, 117304, 58666, 29335,
    14668, 7334, 3667, 1833, 917, 458, 229, 115, 57, 29
];

#[inline]
fn cordic_fixed(x: i32) -> i32 {
    x.wrapping_shl(CORDIC_PRECISION_SHIFT)
}

#[inline]
fn cordic_float(x: i32) -> i32 {
    if x >= 0 {
        ((x.wrapping_shr(CORDIC_PRECISION_SHIFT - 1)).wrapping_add(1)).wrapping_shr(1)
    } else {
        let neg_x = x.wrapping_neg();
        let val = ((neg_x.wrapping_shr(CORDIC_PRECISION_SHIFT - 1)).wrapping_add(1)).wrapping_shr(1);
        val.wrapping_neg()
    }
}

/// Calculates the i/q coordinate for a given angle.
///
/// # Examples
///
/// ```
/// # use kernel::math::cordic::cordic_calc_iq;
/// // For 0 degrees, cos(0) = 1, sin(0) = 0.
/// // The returned values are scaled by 2^16 (65536).
/// let res = cordic_calc_iq(0);
/// assert!((res.i - 65536).abs() <= 10);
/// assert!((res.q - 0).abs() <= 10);
/// ```
#[no_mangle]
pub extern "C" fn cordic_calc_iq(mut theta: i32) -> crate::bindings::cordic_iq {
    let mut coord_i: i32 = CORDIC_ANGLE_GEN;
    let mut coord_q: i32 = 0;
    let mut angle: i32 = 0;
    let mut signx: i32 = 1;

    theta = cordic_fixed(theta);
    let signtheta = if theta < 0 { -1 } else { 1 };
    let term1 = cordic_fixed(180).wrapping_mul(signtheta);
    let term2 = cordic_fixed(360);
    theta = theta.wrapping_add(term1).wrapping_rem(term2).wrapping_sub(term1);

    if cordic_float(theta) > 90 {
        theta = theta.wrapping_sub(cordic_fixed(180));
        signx = -1;
    } else if cordic_float(theta) < -90 {
        theta = theta.wrapping_add(cordic_fixed(180));
        signx = -1;
    }

    for iter in 0..CORDIC_NUM_ITER {
        let valtmp;
        let shift = iter as u32;
        if theta > angle {
            valtmp = coord_i.wrapping_sub(coord_q.wrapping_shr(shift));
            coord_q = coord_q.wrapping_add(coord_i.wrapping_shr(shift));
            angle = angle.wrapping_add(ARCTAN_TABLE[iter]);
        } else {
            valtmp = coord_i.wrapping_add(coord_q.wrapping_shr(shift));
            coord_q = coord_q.wrapping_sub(coord_i.wrapping_shr(shift));
            angle = angle.wrapping_sub(ARCTAN_TABLE[iter]);
        }
        coord_i = valtmp;
    }

    coord_i = coord_i.wrapping_mul(signx);
    coord_q = coord_q.wrapping_mul(signx);

    crate::bindings::cordic_iq {
        i: coord_i,
        q: coord_q,
    }
}
