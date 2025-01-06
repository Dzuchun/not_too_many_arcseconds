use crate::{u206265, u206265ToUnsigned};

quickcheck! {
    fn add(lhs: u128, rhs: u128) -> bool {
        // arrange
        let (sum, ov) = lhs.overflowing_add(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let (the_sum, the_ov) = crate::const_add(&the_lhs, &the_rhs);

        // assert
        let sum2 = u128::try_from(the_sum.const_clone());
        assert!(!the_ov, "Adding two u128 cannot result in overflow");
        if ov {
            sum2 == Err(u206265ToUnsigned {
                bytes_required: 128 / 8 + 1
            })
        } else {
            sum2 == Ok(sum)
        }
    }
}

macro_rules! special_add {
    ($lhs:literal, $rhs:literal) => {
        ::paste::paste! {
            #[test]
            fn [<special_add_ $lhs _add_ $rhs>]() {
                let lhs: u128 = $lhs;
                let rhs: u128 = $rhs;

                // arrange
                let (sum, ov) = lhs.overflowing_add(rhs);

                let the_lhs = u206265::from(lhs);
                let the_rhs = u206265::from(rhs);

                // act
                let (the_sum, the_ov) = crate::const_add(&the_lhs, &the_rhs);

                // assert
                let sum2 = u128::try_from(the_sum.const_clone());
                assert!(!the_ov, "Adding two u128 cannot result in overflow");
                if ov {
                    assert_eq!(sum2, Err(u206265ToUnsigned {
                        bytes_required: 128 / 8 + 1
                    }));
                } else {
                    assert_eq!(sum2, Ok(sum));
                }
            }
        }
    };
}

special_add!(0, 0);
