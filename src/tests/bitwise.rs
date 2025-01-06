use core::ops::{BitAnd, BitOr, BitXor};

use crate::u206265;

quickcheck! {
    fn and(lhs: u128, rhs: u128) -> bool {
        // arrange
        let and = lhs.bitand(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let the_and = crate::const_bitand(&the_lhs, &the_rhs);

        // assert
        let and2 = u128::try_from(the_and).expect("ANDing two u128 should still be a valid u128");
        and == and2
    }
}

quickcheck! {
    fn or(lhs: u128, rhs: u128) -> bool {
        // arrange
        let or = lhs.bitor(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let the_or = crate::const_bitor(&the_lhs, &the_rhs);

        // assert
        let and2 = u128::try_from(the_or).expect("ANDing two u128 should still be a valid u128");
        or == and2
    }
}

quickcheck! {
    fn xor(lhs: u128, rhs: u128) -> bool {
        // arrange
        let xor = lhs.bitxor(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let the_xor = crate::const_bitxor(&the_lhs, &the_rhs);

        // assert
        let and2 = u128::try_from(the_xor).expect("ANDing two u128 should still be a valid u128");
        xor == and2
    }
}
