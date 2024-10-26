use std::eprintln;

use rand::{thread_rng, Rng};

use crate::{u206265, u206265ToUnsigned};

use super::ITERATIONS;

#[test]
fn mul() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let lhs: u128 = rng.gen();
        let rhs: u128 = rng.gen();
        let (mul, ov) = lhs.overflowing_mul(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let (the_mul, the_ov) = crate::const_mul(&the_lhs, &the_rhs);

        // assert
        assert!(!the_ov, "Multiplying two u128 cannot result in overflow");
        let mul2 = u128::try_from(the_mul);
        if ov {
            assert_eq!(
                mul2,
                Err(u206265ToUnsigned {
                    min_bytes: the_mul.significant_bytes()
                }),
            );
        } else if mul2 != Ok(mul) {
            eprintln!("Failed for {lhs} - {rhs}");
            if let Ok(mul2) = mul2 {
                eprintln!("(\n\t{lhs:X} - \n\t{rhs:X} = \n\t{mul:X} != \n\t{mul2:X})");
            }
            panic!("");
        }
    }
}
