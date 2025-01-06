use crate::u206265;

quickcheck! {
    fn cmp(lhs: u128, rhs: u128) -> bool {
        // arrange
        let comp = lhs.cmp(&rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let the_comp = crate::const_cmp(&the_lhs, &the_rhs);

        // assert
        comp == the_comp
    }
}
