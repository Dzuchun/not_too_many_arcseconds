const BYTES: usize = 25_783 + 1; // 206_265 / 8 + 1

#[allow(non_camel_case_types, reason = "foolish little rust-analyser...")]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct u206265([u8; BYTES]); // last byte should only use one bit

pub const MIN: u206265 = u206265([0; BYTES]);
pub const MAX: u206265 = u206265({
    let mut all_max = [0xff; BYTES];
    all_max[0] = 0b1;
    all_max
});
