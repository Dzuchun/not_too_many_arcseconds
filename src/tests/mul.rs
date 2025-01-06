use crate::{u206265, u206265ToUnsigned};

quickcheck! {
    fn mul(lhs: u128, rhs: u128) -> bool {
        // arrange
        let (mul, ov) = lhs.overflowing_mul(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let (the_mul, the_ov) = crate::const_mul(&the_lhs, &the_rhs);

        // assert
        assert!(!the_ov, "Multiplying two u128 cannot result in overflow");
        let mul2 = u128::try_from(the_mul.const_clone());
        if ov {
            mul2 == Err(u206265ToUnsigned {
                bytes_required: the_mul.significant_bytes()
            })
        } else {
            mul2 == Ok(mul)
        }
    }
}

quickcheck! {
    fn mul_small(lhs: u16, rhs: u16) -> bool {
        // arrange
        let (mul, ov) = lhs.overflowing_mul(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let (the_mul, the_ov) = crate::const_mul(&the_lhs, &the_rhs);

        // assert
        assert!(!the_ov, "Multiplying two u128 cannot result in overflow");
        let mul2 = u16::try_from(the_mul.const_clone());
        if ov {
            mul2 == Err(u206265ToUnsigned {
                bytes_required: the_mul.significant_bytes()
            })
        } else {
            mul2 == Ok(mul)
        }
    }
}
