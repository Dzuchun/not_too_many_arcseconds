use std::{dbg, eprintln};

use rand::{thread_rng, Rng};

use crate::{u206265, u206265ToUnsigned};

use super::ITERATIONS;

#[test]
fn add() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let lhs: u128 = rng.r#gen();
        let rhs: u128 = rng.r#gen();
        let (sum, ov) = lhs.overflowing_add(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let (the_sum, the_ov) = crate::const_add(&the_lhs, &the_rhs);

        // assert
        let sum2 = u128::try_from(the_sum.const_clone());
        dbg!(the_sum.significant_bytes());
        assert!(!the_ov, "Adding two u128 cannot result in overflow");
        if ov {
            assert_eq!(
                sum2,
                Err(u206265ToUnsigned {
                    bytes_required: 128 / 8 + 1
                }),
            );
        } else if sum2 != Ok(sum) {
            eprintln!("Failed for {lhs} + {rhs}");
            if let Ok(sum2) = sum2 {
                eprintln!("(\n\t{lhs:X} + \n\t{rhs:X} = \n\t{sum:X} != \n\t{sum2:X})");
            }
            panic!("");
        }
    }
}

#[test]
fn add_special() {
    // arrange
    let lhs = 15u32;
    let rhs = 256u32;
    let sum = lhs + rhs;

    let the_lhs = u206265::from(lhs);
    let the_rhs = u206265::from(rhs);

    // act
    let the_sum = the_lhs + the_rhs;

    // assret
    let sum2: u32 = the_sum
        .try_into()
        .expect("Should be able to convert, it's supposedly, like, 300");
    assert_eq!(sum, sum2, "Results should be the same");
}
