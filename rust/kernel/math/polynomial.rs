// SPDX-License-Identifier: GPL-2.0

//! Polynomial calculation helper functions.

#[inline]
fn mult_frac(x: isize, n: isize, d: isize) -> isize {
    if d == 0 {
        return 0;
    }
    let q = x.wrapping_div(d);
    let r = x.wrapping_rem(d);
    q.wrapping_mul(n).wrapping_add(r.wrapping_mul(n).wrapping_div(d))
}

/// Calculates a polynomial using integer arithmetic.
///
/// # Safety
///
/// The caller must ensure that `poly` is a valid, non-null pointer to a
/// `struct polynomial` with a valid terms array terminated by a term with deg == 0.
///
/// # Examples
///
/// ```
/// # use kernel::math::polynomial::polynomial_calc;
/// # use core::mem::size_of;
/// # #[repr(C)]
/// # struct PolyWithTerms {
/// #     total_divider: isize,
/// #     terms: [kernel::bindings::polynomial_term; 3],
/// # }
/// # let my_poly = PolyWithTerms {
/// #     total_divider: 1,
/// #     terms: [
/// #         kernel::bindings::polynomial_term { deg: 2, coef: 2, divider: 1, divider_leftover: 1 },
/// #         kernel::bindings::polynomial_term { deg: 1, coef: 3, divider: 1, divider_leftover: 1 },
/// #         kernel::bindings::polynomial_term { deg: 0, coef: 5, divider: 1, divider_leftover: 1 },
/// #     ],
/// # };
/// # let poly_ptr = &my_poly as *const PolyWithTerms as *const kernel::bindings::polynomial;
/// // y = 2*x^2 + 3*x + 5. For x = 2, y = 8 + 6 + 5 = 19
/// let res = polynomial_calc(poly_ptr, 2);
/// assert_eq!(res, 19);
/// ```
#[no_mangle]
pub extern "C" fn polynomial_calc(poly: *const crate::bindings::polynomial, data: isize) -> isize {
    if poly.is_null() {
        return 0;
    }

    // SAFETY: We verify that the pointer is not null. The caller guarantees that `poly` points to
    // a valid `struct polynomial` with a terms array terminated by a term with deg == 0.
    unsafe {
        let total_divider = (*poly).total_divider;
        let total_divider = if total_divider == 0 { 1 } else { total_divider };

        let mut term_ptr = core::ptr::addr_of!((*poly).terms) as *const crate::bindings::polynomial_term;
        let mut ret: isize = 0;

        loop {
            let term = &*term_ptr;
            let mut tmp = term.coef;

            for _ in 0..term.deg {
                tmp = mult_frac(tmp, data, term.divider);
            }

            let leftover = if term.divider_leftover == 0 { 1 } else { term.divider_leftover };
            ret = ret.wrapping_add(tmp.wrapping_div(leftover));

            if term.deg == 0 {
                break;
            }

            term_ptr = term_ptr.add(1);
        }

        ret.wrapping_div(total_divider)
    }
}
