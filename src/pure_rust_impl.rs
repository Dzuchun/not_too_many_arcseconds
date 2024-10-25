use const_for::const_for;

use crate::{u206265, BYTES};

pub const fn create_bytes<const N: usize>(bytes: [u8; N]) -> u206265 {
    let Some(diff) = BYTES.checked_sub(N) else {
        panic!("Input array is too big!");
    };
    if diff == 0 {
        assert!(
            bytes[0] <= 1,
            "Can't create u206265: upper-most byte contains more than a single bit!"
        );
    }
    let mut result = [0u8; BYTES];
    const_for!(i in diff..BYTES => result[i] = bytes[i-diff]);
    u206265(result)
}

pub const fn add_new(&(mut lhs): &u206265, rhs: &u206265) -> (u206265, bool) {
    let mut carry = 0u8;
    let significant_length = {
        let lhs = lhs.significant_bytes();
        let rhs = rhs.significant_bytes();
        if lhs > rhs {
            lhs
        } else {
            rhs
        }
    };
    let low_index = BYTES.saturating_sub(significant_length + 2);
    const_for!(i in (low_index..BYTES).rev() => {
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
        match lhs.0[0] {
            0 | 1 => overflow = false,
            2 => {
                overflow = true;
                lhs.0[0] = 0;
            }
            3 => {
                overflow = true;
                lhs.0[0] = 1;
            }
            4.. => panic!("Most-significant bit cannot be 4 or more"),
        }
    } else {
        overflow = false;
    }
    (lhs, overflow)
}
