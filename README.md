# Runix: Migrating Linux Kernel Mathematical Utilities to Rust

Runix is a project dedicated to porting the mathematical helper functions of the Linux Kernel (`lib/math/` directory) from C to memory-safe Rust, while strictly maintaining API/ABI compatibility and preserving the original kernel interfaces for driver modules.

---

## 🚀 Objectives & Principles

The migration adheres to the following core guidelines:

1. **Linus Torvalds Style Geliştirme (Simple & Clean):** Write simple, straightforward, C-like Rust logic. Avoid heavy abstractions, complex generic types, or nested trait systems. Use CPU-optimized native methods (e.g., `leading_zeros()` and `trailing_zeros()`) that compile directly into hardware assembly instructions.
2. **Strictly Safe Rust:** Maintain 100% memory safety. Unsafe blocks are strictly prohibited, except at mandatory FFI pointer boundaries where we check parameters for null pointer values before dereferencing.
3. **ABI & API Preservation:** All ported functions are exported using `#[no_mangle] pub extern "C"` to guarantee the exact C signature and calling conventions are preserved. C drivers and test suites call the Rust functions transparently.
4. **Panic-Free Math:** Since the kernel compiles with `CONFIG_RUST_OVERFLOW_CHECKS=y`, any standard integer overflow will panic and crash the kernel. Runix uses wrapping arithmetic (`wrapping_add`, `wrapping_sub`, `wrapping_mul`, `wrapping_div`, etc.) to match C's overflow wrapping behavior safely.
5. **Rigorous Verification:** Every ported module is validated against existing C-based KUnit test suites and newly added Rust doctests.

---

## 🛠️ Ported Functions & Modules

Under `rust/kernel/math/`, the codebase has been structured modularly:

| Module | Source (C) | Ported (Rust) | Description |
| :--- | :--- | :--- | :--- |
| **`gcd`** | `lib/math/gcd.c` | `math/gcd.rs` | Stein's binary Greatest Common Divisor |
| **`lcm`** | `lib/math/lcm.c` | `math/lcm.rs` | Least Common Multiple (includes `lcm_not_zero`) |
| **`int_pow`** | `lib/math/int_pow.c` | `math/int_pow.rs` | Exponentiation by squaring |
| **`int_sqrt`** | `lib/math/int_sqrt.c` | `math/int_sqrt.rs` | Integer square root (includes `int_sqrt64` on 32-bit targets) |
| **`reciprocal`** | `lib/math/reciprocal_div.c` | `math/reciprocal.rs` | Reciprocal division optimization |
| **`rational`** | `lib/math/rational.c` | `math/rational.rs` | Continued fraction expansion for rational approximation |
| **`int_log`** | `lib/math/int_log.c` | `math/int_log.rs` | Fixed-point base-2 and base-10 logarithms |
| **`cordic`** | `lib/math/cordic.c` | `math/cordic.rs` | CORDIC algorithm for angle/coordinate calculation |

---

## 🧪 Testing and Verification

Validation is performed using User Mode Linux (UML) and KUnit. 

### 1. Enable Kconfig options
Ensure `.kunitconfig` contains the mathematical test suites:
```text
CONFIG_KUNIT=y
CONFIG_RUST=y
CONFIG_RUST_KERNEL_DOCTESTS=y
CONFIG_GCD_KUNIT_TEST=y
CONFIG_INT_POW_KUNIT_TEST=y
CONFIG_INT_SQRT_KUNIT_TEST=y
CONFIG_INT_LOG_KUNIT_TEST=y
CONFIG_RATIONAL_KUNIT_TEST=y
CONFIG_RATIONAL=y
CONFIG_CORDIC=y
```

### 2. Run KUnit Suite
Run the test command:
```bash
./tools/testing/kunit/kunit.py run --make_options LLVM=1 --kunitconfig=.kunitconfig
```
Currently, all 400+ KUnit tests and Rust library doctests pass successfully.
