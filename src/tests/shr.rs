use core::ops::Shr;

use deranged::RangedU32;

use crate::u206265;

quickcheck! {
    fn shr(lhs: u128, rhs: RangedU32<0, 127>) -> bool {
        // arrange
        let rhs: u32 = rhs.into();
        let shift = lhs.shr(rhs);

        let the_lhs = u206265::from(lhs);

        // act
        let (the_shift, the_ov) = crate::const_shr(&the_lhs, rhs);

        // assert
        let shift2 = u128::try_from(the_shift);
        assert!(
            !the_ov,
            "Shifting by 128 bits cannot result in u206265 overflow"
        );
        shift2 == Ok(shift)
    }
}

macro_rules! special_shr {
    ($lhs:literal, $rhs:literal) => {
        ::paste::paste! {
            #[test]
            fn [<special_shr_ $lhs _by_ $rhs>]() {
                let lhs: u128 = $lhs;
                let rhs: u32 = $rhs;
                let shift = lhs.shr(rhs);

                let the_lhs = u206265::from(lhs);

                // act
                let (the_shift, the_ov) = crate::const_shr(&the_lhs, rhs);

                // assert
                let shift2 = u128::try_from(the_shift);
                assert!(
                    !the_ov,
                    "Shifting by 128 bits cannot result in u206265 overflow"
                );
                assert_eq!(shift2, Ok(shift));
            }
        }
    };
}

special_shr!(1, 1);
special_shr!(0, 127);
