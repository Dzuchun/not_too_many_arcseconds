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

pub const fn const_shl(lhs: &u206265, mut rhs: u32) -> (u206265, bool) {
    let mut result = *lhs;

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
        const_for!(i in (byte_shift..BYTES).rev() => result.0[i] = result.0[i-byte_shift]);
        const_for!(i in 0..byte_shift => result.0[i] = 0);
    }

    // then, the subbyte shift
    let subbyte_shift = rhs & 0b111;
    let mut carry = 0u16;
    const_for!(i in byte_shift..BYTES => {
        carry += (result.0[i] as u16) << subbyte_shift;
        result.0[i] = (carry & 0x00FF) as u8;
        carry >>= 8;
    });
    (result, overflow)
}

pub const fn const_shr(lhs: &u206265, mut rhs: u32) -> (u206265, bool) {
    let mut result = *lhs;

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
    (result, overflow)
}

pub const fn const_add(&(mut lhs): &u206265, rhs: &u206265) -> (u206265, bool) {
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

    let overflow;
    if significant_length == BYTES {
        match lhs.0[BYTES - 1] {
            0 | 1 => overflow = false,
            2 => {
                overflow = true;
                lhs.0[BYTES - 1] = 0;
            }
            3 => {
                overflow = true;
                lhs.0[BYTES - 1] = 1;
            }
            4.. => panic!("Most-significant bit cannot be 4 or more"),
        }
    } else {
        overflow = false;
    }
    (lhs, overflow)
}

pub const fn const_sub(&(mut lhs): &u206265, rhs: &u206265) -> (u206265, bool) {
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

    let underflow;
    match (lhs.0[BYTES - 1] & 0xFE, lhs.0[BYTES - 1] & 0x01) {
        (0, 0 | 1) => underflow = false,
        (_, last) => {
            underflow = true;
            lhs.0[BYTES - 1] = last;
        }
    }
    (lhs, underflow)
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
            let lhs = lhs.0[lhs_power];
            let rhs = rhs.0[rhs_power];
            let Some(power_mul) = (lhs as u16).checked_mul(rhs as u16) else {
                panic!("Should not overflow on 2-integer multiplication of 1-byte integers");
            };
            carry += power_mul as u32;
        });}
        result[power] = (carry & 0xFF) as u8;
        carry >>= 8;
    });

    let overflow;
    if max_power == BYTES {
        match result[BYTES - 1] {
            0 | 1 => overflow = false,
            2 => {
                overflow = true;
                result[BYTES - 1] = 0;
            }
            3 => {
                overflow = true;
                result[BYTES - 1] = 1;
            }
            4.. => panic!("Most-significant bit cannot be 4 or more"),
        }
    } else {
        overflow = false;
    }
    (u206265(result), overflow)
}

pub const fn const_div(lhs: &u206265, rhs: &u206265) -> Option<(u206265, u206265)> {
    if const_cmp(rhs, &u206265::ZERO).is_eq() {
        return None;
    }
    let mut remainder = *lhs;
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

pub const fn const_ilog(val: &u206265, base: &u206265) -> Option<u32> {
    const TWO: u206265 = create_bytes([0x02]);
    if const_cmp(base, &TWO).is_lt() || const_cmp(val, &u206265::ZERO).is_eq() {
        return None;
    }
    if const_cmp(val, &u206265::ONE).is_eq() {
        return Some(0);
    }
    let mut probe = u206265::ONE;
    let mut res = 0u32;
    loop {
        let (new_probe, overflow) = const_mul(&probe, base);
        if overflow {
            break;
        }
        probe = new_probe;
        match const_cmp(val, &probe) {
            Ordering::Less => break,
            Ordering::Equal => {
                res += 1;
                break;
            }
            Ordering::Greater => {}
        }
        res += 1;
    }
    Some(res)
}
