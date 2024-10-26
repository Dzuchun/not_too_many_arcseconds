use const_for::const_for;

use crate::{u206265, BYTES};

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
