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
