use core::ops::{BitAnd, BitOr, BitXor};

use rand::{thread_rng, Rng};

use crate::u206265;

use super::ITERATIONS;

#[test]
fn and() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let lhs: u128 = rng.gen();
        let rhs: u128 = rng.r#gen();
        let and = lhs.bitand(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let the_and = crate::const_bitand(&the_lhs, &the_rhs);

        // assert
        let and2 = u128::try_from(the_and).expect("ANDing two u128 should still be a valid u128");
        assert_eq!(and, and2);
    }
}

#[test]
fn or() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let lhs: u128 = rng.gen();
        let rhs: u128 = rng.r#gen();
        let or = lhs.bitor(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let the_or = crate::const_bitor(&the_lhs, &the_rhs);

        // assert
        let or2 = u128::try_from(the_or).expect("ANDing two u128 should still be a valid u128");
        assert_eq!(or, or2);
    }
}

#[test]
fn xor() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let lhs: u128 = rng.gen();
        let rhs: u128 = rng.r#gen();
        let xor = lhs.bitxor(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let the_xor = crate::const_bitxor(&the_lhs, &the_rhs);

        // assert
        let xor2 = u128::try_from(the_xor).expect("ANDing two u128 should still be a valid u128");
        assert_eq!(xor, xor2);
    }
}
