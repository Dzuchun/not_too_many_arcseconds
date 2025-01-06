use core::ops::Shl;

use deranged::{RangedU128, RangedU32};

use crate::{u206265, u206265ToUnsigned};

const MAX_LHS: u128 = u128::MAX >> 30;

quickcheck! {
    fn shl(lhs: RangedU128<0, MAX_LHS>, rhs: RangedU32<0, 32>) -> bool {
        // arrange
        let lhs: u128 = lhs.into();
        let rhs: u32 = rhs.into();
        let shift = lhs.shl(rhs);

        let the_lhs = u206265::from(lhs);

        // act
        let (the_shift, the_ov) = crate::const_shl(&the_lhs, rhs);

        // assert
        let shift2 = u128::try_from(the_shift);
        assert!(
            !the_ov,
            "Shifting by 128 bits cannot result in u206265 overflow"
        );
        shift2 == Ok(shift) || shift2 == Err(u206265ToUnsigned { bytes_required: 17 })
    }
}

macro_rules! special_shl {
    ($lhs:literal, $rhs:literal) => {
        ::paste::paste! {
            #[test]
            fn [<special_shl_ $lhs _by_ $rhs>]() {
                let lhs: u128 = $lhs;
                let rhs: u32 = $rhs;
                let shift = lhs.shl(rhs);

                let the_lhs = u206265::from(lhs);

                // act
                let (the_shift, the_ov) = crate::const_shl(&the_lhs, rhs);

                // assert
                let shift2 = u128::try_from(the_shift);
                assert!(
                    !the_ov,
                    "Shifting by 128 bits cannot result in u206265 overflow"
                );
                assert!(shift2 == Ok(shift) || shift2 == Err(u206265ToUnsigned { bytes_required: 17 }))
            }
        }
    };
}

special_shl!(2_305_843_009_213_693_952, 67);
