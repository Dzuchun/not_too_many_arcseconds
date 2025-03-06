use crate::{BYTES, u206265, u206265ToUnsigned};

quickcheck! {
    fn sub(lhs: u128, rhs: u128) -> bool {
        // arrange
        let (sub, ov) = lhs.overflowing_sub(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let (the_sub, the_ov) = crate::const_sub(&the_lhs, &the_rhs);

        // assert
        assert_eq!(ov, the_ov, "Overflow flags should be the same");
        assert_eq!(ov, rhs > lhs, "Overflow MUST occur, if rhs is greater than lhs");
        let sub2 = u128::try_from(the_sub);
        if ov {
            sub2 == Err(u206265ToUnsigned {
                bytes_required: BYTES
            })
        } else {
            sub2 == Ok(sub)
        }
    }
}
