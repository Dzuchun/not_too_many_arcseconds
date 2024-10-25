use crate::{u206265, BYTES};

pub fn create_bytes<const N: usize>(bytes: [u8; N]) -> u206265 {
    let diff: usize = BYTES.checked_sub(N).expect("Input array is too big!");
    if diff == 0 {
        assert!(
            bytes[0] <= 1,
            "Can't create u206265: upper-most byte contains more than a single bit!"
        );
    }
    let mut result = [0u8; BYTES];
    result[diff..].copy_from_slice(bytes.as_slice());
    u206265(result)
}
