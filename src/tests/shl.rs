use core::ops::{Shl, Shr};

use rand::{thread_rng, Rng};

use crate::u206265;

use super::ITERATIONS;

#[test]
fn shl() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let sh = rng.gen_range(0..128);
        let lhs: u128 = rng.r#gen::<u128>().shr(sh);
        let rhs: u32 = rng.gen_range(0..=sh);
        let shift = lhs.shl(rhs);

        let the_lhs = u206265::from(lhs);

        // act
        let (the_shift, the_ov) = crate::const_shl(&the_lhs, rhs);

        // assert
        let shift2 = u128::try_from(the_shift);
        assert!(
            !the_ov,
            "Shifting by 32 bits cannot result in u206265 overflow"
        );
        assert_eq!(shift2, Ok(shift));
    }
}
