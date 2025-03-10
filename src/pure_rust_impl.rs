use core::cmp::Ordering;

use const_for::const_for;

use crate::{BITS_U32, BYTES, u206265};

/// Creates [`u206265`] from provided little-endian bytes. Can be used in constant context.
///
/// To clarify things for dummies like me: this means that least significant byte comes **first**.
///
/// ### Panics
///
/// 1. If input array is too big. In this case, bigger than ``25_784`` bytes.
///
/// ```rust,should_panic
/// not_too_many_arcseconds::create_bytes([0; 25_785]);
/// ```
///
/// 2. If array is exactly ``25_784`` bytes, and the last byte is greater than 1
///
/// ```rust,should_panic
/// let mut array = [0; 25_784];
/// array[25_784 - 1] = 0x10; // setting last byte to ``2``
/// not_too_many_arcseconds::create_bytes(array);
/// ```
///
/// ### Example
///
/// Manually create from [`u32`]:
///
/// ```rust
/// let some_u32: u32 = 0xAB_CD_DE_F1;
/// let some_u206265 = not_too_many_arcseconds::create_bytes(some_u32.to_le_bytes());
/// # assert_eq!(format!("{some_u32}"), format!("{some_u206265}"));
/// ```
///
/// Though in this case, you probably better off using [`u206265::from_u32`].
pub const fn create_bytes<const N: usize>(bytes: [u8; N]) -> u206265 {
    assert!(N <= BYTES, "Input array is too big!");
    if N == BYTES {
        assert!(
            bytes[BYTES - 1] <= 1,
            "Upper-most byte should contain at most 1!"
        );
    }
    let mut result = [0u8; BYTES];
    const_for!(i in 0..N => result[i] = bytes[i]);
    u206265(result)
}

/// Compares two [`u206265`]s. Same as [`Ord::cmp`], but can be used in constant context.
pub const fn const_cmp(lhs: &u206265, rhs: &u206265) -> Ordering {
    let lhs_bytes = lhs.significant_bytes();
    let rhs_bytes = rhs.significant_bytes();
    if lhs_bytes < rhs_bytes {
        return Ordering::Less;
    }
    if lhs_bytes > rhs_bytes {
        return Ordering::Greater;
    }
    assert!(lhs_bytes == rhs_bytes);
    const_for!(b in (0..lhs_bytes).rev() => {
        let lhs = lhs.0[b];
        let rhs = rhs.0[b];
        if lhs < rhs {
            return Ordering::Less;
        }
        if lhs > rhs {
            return Ordering::Greater;
        }
        assert!(lhs == rhs);
    });
    Ordering::Equal
}

/// Shifts [`u206265`]s by ``rhs`` bits to the left. Can be used in constant context.
///
/// Operation is performed in-place, so prefer using this function (or ``<<=`` operator), if you wish to avoid copying [`u206265`]s around.
///
/// ### Returns
/// If overflow had occurred.
///
/// This implementation attempts to be consistent with the standard library one, so please look closely if ``std``'s overflow in this case means what you expect. I sure was surprised.
pub const fn const_shl_assign(lhs: &mut u206265, mut rhs: u32) -> bool {
    // first, do the same thing std does, for consistency
    let overflow;
    if rhs >= BITS_U32 {
        overflow = true;
        rhs %= BITS_U32;
    } else {
        overflow = false;
    }

    // first, apply the whole-byte shift
    let byte_shift = (rhs >> 3) as usize;
    if byte_shift > 0 {
        const_for!(i in (byte_shift..BYTES).rev() => lhs.0[i] = lhs.0[i-byte_shift]);
        const_for!(i in 0..byte_shift => lhs.0[i] = 0);
    }

    // then, the subbyte shift
    let subbyte_shift = rhs & 0b111;
    let mut carry = 0u16;
    const_for!(i in byte_shift..BYTES => {
        carry += (lhs.0[i] as u16) << subbyte_shift;
        lhs.0[i] = (carry & 0x00FF) as u8;
        carry >>= 8;
    });
    overflow
}

/// Shifts [`u206265`]s by ``rhs`` bits to the left. Can be used in constant context.
///
/// This implementation attempts to be consistent with the standard library one, so please look closely if ``std``'s overflow in this case means what you expect. I sure was surprised.
#[inline]
pub const fn const_shl(lhs: &u206265, rhs: u32) -> (u206265, bool) {
    let mut result = lhs.const_clone();
    let overflow = const_shl_assign(&mut result, rhs);
    (result, overflow)
}

/// Shifts [`u206265`]s by ``rhs`` bits to the right. Can be used in constant context.
///
/// Operation is performed in-place, so prefer using this function (or ``>>=`` operator), if you wish to avoid copying [`u206265`]s around.
///
/// ### Returns
/// If overflow had occurred.
///
/// This implementation attempts to be consistent with the standard library one, so please look closely if ``std``'s overflow in this case means what you expect. I sure was surprised.
pub const fn const_shr_assign(result: &mut u206265, mut rhs: u32) -> bool {
    // first, do the same thing std does, for consistency
    let overflow;
    if rhs >= BITS_U32 {
        overflow = true;
        rhs %= BITS_U32;
    } else {
        overflow = false;
    }

    // first, apply the whole-byte shift
    let mut byte_shift = (rhs >> 3) as usize;
    if byte_shift > 0 {
        const_for!(i in 0..(BYTES - byte_shift) => result.0[i] = result.0[i + byte_shift]);
        const_for!(i in (BYTES - byte_shift)..BYTES => result.0[i] = 0);
        byte_shift -= 1;
    }

    // then, the subbyte shift
    let subbyte_shift = rhs & 0b111;
    let mut carry = 0u16;
    const_for!(i in (0..(BYTES - byte_shift)).rev() => {
        carry += ((result.0[i] as u16) << 8) >> subbyte_shift;
        result.0[i] = (carry >> 8) as u8;
        carry <<= 8;
    });
    overflow
}

/// Shifts [`u206265`]s by ``rhs`` bits to the right. Can be used in constant context.
///
/// This implementation attempts to be consistent with the standard library one, so please look closely if ``std``'s overflow in this case means what you expect. I sure was surprised.
#[inline]
pub const fn const_shr(lhs: &u206265, rhs: u32) -> (u206265, bool) {
    let mut result = lhs.const_clone();
    let overflow = const_shr_assign(&mut result, rhs);
    (result, overflow)
}

/// Adds ``rhs`` to ``lhs``. Same as [`core::ops::AddAssign::add_assign`], but can be used in constant context.
///
/// Operation is performed in-place, so prefer using this function (or ``+=`` operator), if you wish to avoid copying [`u206265`]s around.
///
/// ### Returns
/// If arithmetic overflow had occurred.
pub const fn const_add_assign(lhs: &mut u206265, rhs: &u206265) -> bool {
    let significant_length = {
        let mut sl;
        let lhs = lhs.significant_bytes();
        let rhs = rhs.significant_bytes();
        if lhs > rhs {
            sl = lhs;
        } else {
            sl = rhs;
        }
        if sl < BYTES {
            sl += 1;
        }
        sl
    };

    let mut carry = 0u8;
    const_for!(i in 0..significant_length => {
        let sum = carry;
        carry = 0;

        let (sum, overflow) = sum.overflowing_add(lhs.0[i]);
        if overflow {
            carry += 1;
        }

        let (sum, overflow) = sum.overflowing_add(rhs.0[i]);
        if overflow {
            carry += 1;
        }

        lhs.0[i] = sum;
    });

    if significant_length == BYTES {
        match lhs.0[BYTES - 1] {
            0 | 1 => false,
            2 => {
                lhs.0[BYTES - 1] = 0;
                true
            }
            3 => {
                lhs.0[BYTES - 1] = 1;
                true
            }
            4.. => panic!("Most-significant bit cannot be 4 or more"),
        }
    } else {
        false
    }
}

/// Adds ``rhs`` to ``lhs``. Same as [`core::ops::Add::add`], but can be used in constant context.
#[inline]
pub const fn const_add(lhs: &u206265, rhs: &u206265) -> (u206265, bool) {
    let mut result = lhs.const_clone();
    let overflow = const_add_assign(&mut result, rhs);
    (result, overflow)
}

/// Subtracts ``rhs`` from ``lhs``. Same as [`core::ops::SubAssign::sub_assign`], but can be used in constant context.
///
/// Operation is performed in-place, so prefer using this function (or ``-=`` operator), if you wish to avoid copying [`u206265`]s around.
///
/// ### Returns
/// If arithmetic underflow had occurred.
pub const fn const_sub_assign(lhs: &mut u206265, rhs: &u206265) -> bool {
    let mut borrow = 0u8;
    const_for!(i in 0..BYTES => {
        let (sub, underflow) = lhs.0[i].overflowing_sub(borrow);

        borrow = 0;
        if underflow {
            borrow += 1;
        }

        let (sub, underflow) = sub.overflowing_sub(rhs.0[i]);
        if underflow {
            borrow += 1;
        }

        lhs.0[i] = sub;
    });

    match (lhs.0[BYTES - 1] & 0xFE, lhs.0[BYTES - 1] & 0x01) {
        (0, 0 | 1) => false,
        (_, last) => {
            lhs.0[BYTES - 1] = last;
            true
        }
    }
}

/// Subtracts ``rhs`` from ``lhs``. Same as [`core::ops::Sub::sub`], but can be used in constant context.
#[inline]
pub const fn const_sub(lhs: &u206265, rhs: &u206265) -> (u206265, bool) {
    let mut result = lhs.const_clone();
    let underflow = const_sub_assign(&mut result, rhs);
    (result, underflow)
}

/// Multiplies ``rhs`` by ``lhs``. Same as [`core::ops::MulAssign::mul_assign`], but can be used in constant context.
///
/// ### Returns
/// If arithmetic overflow had occurred.
#[inline]
pub const fn const_mul_assign(lhs: &mut u206265, rhs: &u206265) -> bool {
    let (result, overflow) = const_mul(lhs, rhs);
    *lhs = result;
    overflow
}

/// Multiplies ``rhs`` by ``lhs``. Same as [`core::ops::Mul::mul`], but can be used in constant context.
pub const fn const_mul(lhs: &u206265, rhs: &u206265) -> (u206265, bool) {
    let lhs_bytes = lhs.significant_bytes();
    let rhs_bytes = rhs.significant_bytes();
    let max_power = {
        let mut mp = lhs_bytes + rhs_bytes + 2;
        if mp > BYTES {
            mp = BYTES;
        }
        mp
    };

    let mut result = [0u8; BYTES];
    let mut carry = 0u32; // about 26k additions max, 256 max addition for each
    const_for!(power in 0..max_power => {
        #[allow(clippy::range_plus_one, reason = "const_for! is not compatible with ..= syntax :(")]
        {const_for!(lhs_power in 0..(power+1) => {
            if lhs_power > lhs_bytes {
                continue;
            }
            let rhs_power = power - lhs_power;
            if rhs_power > rhs_bytes {
                continue;
            }
            carry += lhs.0[lhs_power] as u32 * rhs.0[rhs_power] as u32;
        });}
        result[power] = (carry & 0xFF) as u8;
        carry >>= 8;
    });

    let overflow;
    if max_power == BYTES {
        let last_byte = &mut result[BYTES - 1];
        overflow = *last_byte > 1u8;
        *last_byte &= 0x01;
    } else {
        overflow = false;
    }
    (u206265(result), overflow)
}

/// Divides ``lhs`` by ``rhs``.
///
/// ### Returns
/// ``Option<(quotient, remainder)>``. [`Option::None`] corresponds to ``rhs == 0``.
pub const fn const_div_rem(lhs: &u206265, rhs: &u206265) -> Option<(u206265, u206265)> {
    if const_cmp(rhs, &u206265::ZERO).is_eq() {
        return None;
    }
    let mut remainder = lhs.const_clone();
    let mut result = u206265::ZERO;
    #[allow(clippy::cast_possible_truncation)]
    let significant_bits = ((lhs
        .significant_bytes()
        .saturating_sub(rhs.significant_bytes())
        + 2)
        * 8) as u32;
    const_for!(bit in (0..significant_bits).rev() => {
        let (probe, _) = const_shl(rhs, bit);
        if const_cmp(&remainder, &probe).is_lt() {
            continue;
        }
        remainder = const_sub(&remainder, &probe).0;
        let res_add = const_shl(&u206265::ONE, bit).0;
        result = const_add(&result, &res_add).0;
    });
    Some((result, remainder))
}

/// Divides ``lhs`` by ``rhs``. Same as ``{int}::checked_div``.
#[inline]
pub const fn const_div(lhs: &u206265, rhs: &u206265) -> Option<u206265> {
    if let Some((result, _)) = const_div_rem(lhs, rhs) {
        Some(result)
    } else {
        None
    }
}

/// Finds smallest* ``lhs`` modulo ``rhs``. Same as ``{int}::checked_rem``.
///
/// * - mathematically, finding "an int modulo other int" is not a single-valued operation. What we usually want is the smallest modulo value.
#[inline]
pub const fn const_rem(lhs: &u206265, rhs: &u206265) -> Option<u206265> {
    if let Some((_, result)) = const_div_rem(lhs, rhs) {
        Some(result)
    } else {
        None
    }
}

/// Divides ``lhs`` by ``rhs``, assigning the result. Same as [`core::ops::DivAssign::div_assign`].
///
/// ### Panics
/// If ``rhs == 0``.
#[inline]
pub const fn const_div_assign(lhs: &mut u206265, rhs: &u206265) {
    *lhs = const_div(lhs, rhs).expect("Division by zero");
}

/// Finds smallest* ``lhs`` modulo ``rhs``, assigning the result. Same as [`core::ops::RemAssign::rem_assign`].
///
/// * - mathematically, finding "an int modulo other int" is not a single-valued operation. What we usually want, is the smallest modulo value.
///
/// ### Panics
/// If ``rhs == 0``.
#[inline]
pub const fn const_rem_assign(lhs: &mut u206265, rhs: &u206265) {
    *lhs = const_rem(lhs, rhs).expect("Division by zero");
}

/// Finds $\log_{\text{base}}(\test{val})$, if one exists. Same as ``{int}::checked_ilog``.
///
/// This implementation attempts to be consistent with ``core`` functions, so please check "logarithms exists" means exactly what you think it means. For instance, in ``core`` terms, ``logi(1, 1)`` does not exist:
///
/// ```rust
/// # use not_too_many_arcseconds::{const_ilog, u206265};
/// assert_eq!(const_ilog(&u206265::ONE, &u206265::ONE), None);
/// ```
#[allow(
    clippy::missing_panics_doc,
    reason = "17 steps is enough to overflow; I need manual array splitting, since there's a complex mutability pattern going on"
)]
pub const fn const_ilog(val: &u206265, base: &u206265) -> Option<u32> {
    if const_cmp(base, &u206265::ONE).is_le() || const_cmp(val, &u206265::ZERO).is_eq() {
        return None;
    }
    if const_cmp(val, &u206265::ONE).is_eq() {
        return Some(0);
    }
    let mut val = val.const_clone(); // I'm sorry for that. It's much easier to work with way
    let mut res = 0u32;
    let mut powers_of_probe: [Option<(u206265, bool)>; 17] = [
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None,
    ];
    powers_of_probe[0] = Some((base.const_clone(), false));
    while const_cmp(&val, base).is_ge() {
        let (probe, mut next_probes) = powers_of_probe.split_first_mut().unwrap();
        let mut probe = probe.as_ref().unwrap();
        let mut probe_res = 1u32;
        loop {
            let (new_probe, new_next_probes) = next_probes.split_first_mut().unwrap();
            next_probes = new_next_probes;
            let new_probe = if let Some(present) = new_probe {
                present
            } else {
                *new_probe = Some(const_mul(&probe.0, &probe.0));
                new_probe.as_ref().unwrap()
            };
            if new_probe.1 || const_cmp(&val, &new_probe.0).is_lt() {
                break;
            }
            probe = new_probe;
            probe_res <<= 1;
        }
        const_div_assign(&mut val, &probe.0);
        res += probe_res;
    }
    Some(res)
}

/// Same as [`const_ilog`], but base 10.
pub const fn const_ilog10(val: &u206265) -> Option<u32> {
    const TEN: u206265 = create_bytes([10u8]);
    const_ilog(val, &TEN)
}

/// Same as [`const_ilog`], but base 2.
///
/// MUCH faster than ``const_ilog(_, 2)``, so prefer this one, if you know your base to be 2.
pub const fn const_ilog2(val: &u206265) -> Option<u32> {
    // basically, I need to find position of the highest bit
    let high_byte_pos = val.significant_bytes();
    let high_byte = val.0[high_byte_pos - 1]; // 1 significant byte means highest byte is 0, an so on
    let Some(high_byte_bit) = high_byte.checked_ilog2() else {
        // if high byte is 0, the number itself is 0 - return none
        //
        // const context - can't use the `?` operator :(
        return None;
    };
    Some((val.significant_bytes_u32() - 1) * 8 + high_byte_bit)
}

macro_rules! bit_op {
    ($op_name:ident, $op_assign:tt) => {
        ::paste::paste! {
            #[doc = concat!("Finds ", stringify!([<$op_name:lower>]), " of ``lhs`` and ``rhs``, assigning the result to ``lhs``.")]
            #[doc = concat!("Same as [`core::ops::", stringify!([<$op_name Assign>]), "::", stringify!([<$op_name:lower _assign>]), "`], but can be used in a constant context.")]
            #[doc = concat!("Operation is performed in-place, so prefer using this function (or ``", stringify!($op_assign), "`` operator), if you wish to avoid copying [`u206265`]s around.")]
            pub const fn [<const_ $op_name:lower _assign>](lhs: &mut u206265, rhs: &u206265) {
                let lhs_bytes = lhs.significant_bytes();
                let rhs_bytes = rhs.significant_bytes();
                let bytes = if lhs_bytes >= rhs_bytes {
                    lhs_bytes
                } else {
                    rhs_bytes
                };
                const_for!(i in 0..bytes => {
                    lhs.0[i] $op_assign rhs.0[i];
                });
            }

            #[doc = concat!("Finds ", stringify!([<$op_name:lower>]), " of ``lhs`` and ``rhs``.")]
            #[doc = concat!("Same as [`core::ops::", stringify!($op_name), "::", stringify!([<$op_name:lower>]), "`], but can be used in a constant context.")]
            #[inline]
            pub const fn [<const_ $op_name:lower>](lhs: &u206265, rhs: &u206265) -> u206265 {
                let mut lhs = lhs.const_clone();
                [<const_ $op_name:lower _assign>](&mut lhs, rhs);
                lhs
            }
        }
    };
}

bit_op! {BitAnd, &=}
bit_op! {BitOr, |=}
bit_op! {BitXor, ^=}

/// Performs bitwise inversion of the integer. Same as [`core::ops::Not::not`], but can be used in a constant context.
///
/// Operation is performed in-place, so prefer using this function, if you wish to avoid copying [`u206265`]s around.
///
/// NOTE: ``!x`` COPIES the value, see [`core::ops::Not::not`] signature.
pub const fn const_not_assign(val: &mut u206265) {
    const_for!(i in 0..BYTES => {
        val.0[i] = !val.0[i];
    });
    // last byte should only use one bit
    val.0[BYTES - 1] &= 0x01;
}
