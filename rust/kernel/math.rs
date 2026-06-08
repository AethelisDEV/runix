// SPDX-License-Identifier: GPL-2.0

//! Mathematical helper functions.

pub mod gcd;
pub mod lcm;
pub mod int_pow;
pub mod int_sqrt;
pub mod reciprocal;
pub mod rational;
pub mod int_log;
pub mod cordic;
pub mod polynomial;

// Re-export symbols at the module root to maintain API & ABI compatibility.
pub use gcd::gcd;
pub use lcm::{lcm, lcm_not_zero};
pub use int_pow::int_pow;
pub use int_sqrt::int_sqrt;
#[cfg(target_pointer_width = "32")]
pub use int_sqrt::int_sqrt64;
pub use reciprocal::{reciprocal_value, reciprocal_value_adv};
pub use rational::rational_best_approximation;
pub use int_log::{intlog2, intlog10};
pub use cordic::cordic_calc_iq;
pub use polynomial::polynomial_calc;
