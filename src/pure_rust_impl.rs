use core::cmp::Ordering;

use const_for::const_for;

use crate::{u206265, BITS_U32, BYTES};

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

/// ### Returns
/// If overflow had occurred
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

pub const fn const_shl(lhs: &u206265, rhs: u32) -> (u206265, bool) {
    let mut result = lhs.const_clone();
    let overflow = const_shl_assign(&mut result, rhs);
    (result, overflow)
}

/// ### Returns
/// If overflow had occurred
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

pub const fn const_shr(lhs: &u206265, rhs: u32) -> (u206265, bool) {
    let mut result = lhs.const_clone();
    let overflow = const_shr_assign(&mut result, rhs);
    (result, overflow)
}

/// ### Returns
/// If overflow had occurred
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

pub const fn const_add(lhs: &u206265, rhs: &u206265) -> (u206265, bool) {
    let mut result = lhs.const_clone();
    let overflow = const_add_assign(&mut result, rhs);
    (result, overflow)
}

/// ### Returns
/// If underflow had occurred
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

pub const fn const_sub(lhs: &u206265, rhs: &u206265) -> (u206265, bool) {
    let mut result = lhs.const_clone();
    let underflow = const_sub_assign(&mut result, rhs);
    (result, underflow)
}

pub const fn const_mul_assign(lhs: &mut u206265, rhs: &u206265) -> bool {
    let (result, overflow) = const_mul(lhs, rhs);
    *lhs = result;
    overflow
}

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

#[inline]
pub const fn const_div(lhs: &u206265, rhs: &u206265) -> Option<u206265> {
    if let Some((result, _)) = const_div_rem(lhs, rhs) {
        Some(result)
    } else {
        None
    }
}

#[inline]
pub const fn const_rem(lhs: &u206265, rhs: &u206265) -> Option<u206265> {
    if let Some((_, result)) = const_div_rem(lhs, rhs) {
        Some(result)
    } else {
        None
    }
}

#[inline]
pub const fn const_div_assign(lhs: &mut u206265, rhs: &u206265) {
    *lhs = const_div(lhs, rhs).expect("Division by zero");
}

#[inline]
pub const fn const_rem_assign(lhs: &mut u206265, rhs: &u206265) {
    *lhs = const_rem(lhs, rhs).expect("Division by zero");
}

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

pub const fn const_ilog10(val: &u206265) -> Option<u32> {
    const TEN: u206265 = create_bytes([10u8]);
    const_ilog(val, &TEN)
}

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
            pub const fn [<const_ $op_name _assign>](lhs: &mut u206265, rhs: &u206265) {
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

            #[inline]
            pub const fn [<const_ $op_name>](lhs: &u206265, rhs: &u206265) -> u206265 {
                let mut lhs = lhs.const_clone();
                [<const_ $op_name _assign>](&mut lhs, rhs);
                lhs
            }
        }
    };
}

bit_op! {bitand, &=}
bit_op! {bitor, |=}
bit_op! {bitxor, ^=}

pub const fn const_not_assign(val: &mut u206265) {
    const_for!(i in 0..BYTES => {
        val.0[i] = !val.0[i];
    });
    // last byte should only use one bit
    val.0[BYTES - 1] &= 0x01;
}
