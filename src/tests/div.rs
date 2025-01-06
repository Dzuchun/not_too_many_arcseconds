use crate::u206265;

quickcheck! {
    fn div_rem(lhs: u128, rhs: u128) -> bool {
        // arrange
        let div = lhs.checked_div(rhs);
        let rem = lhs.checked_rem(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let the_result = crate::const_div_rem(&the_lhs, &the_rhs);

        // assert
        let (div, rem, the_div, the_rem) = match (div, rem, the_result) {
            (None, None, None) => return true, // correctly caught division by 0
            (Some(div), Some(rem), Some((the_div, the_rem))) => (div, rem, the_div, the_rem),
            _ => unreachable!("Option variants must be the same!"),
        };

        let div2 = u128::try_from(the_div).unwrap();
        let rem2 = u128::try_from(the_rem).unwrap();
        (div == div2) && (rem == rem2)
    }
}
