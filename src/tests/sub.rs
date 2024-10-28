use std::{dbg, eprintln};

use rand::{thread_rng, Rng};

use crate::{u206265, u206265ToUnsigned, BYTES};

use super::ITERATIONS;

#[test]
fn sub() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let lhs: u128 = rng.gen();
        let rhs: u128 = rng.gen();
        let (sub, ov) = lhs.overflowing_sub(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let (the_sub, the_ov) = crate::const_sub(&the_lhs, &the_rhs);

        // assert
        assert_eq!(ov, the_ov, "Overflow flags should be the same");
        dbg!(the_sub.significant_bytes());
        let sub2 = u128::try_from(the_sub);
        if ov {
            assert_eq!(sub2, Err(u206265ToUnsigned { min_bytes: BYTES }),);
        } else if sub2 != Ok(sub) {
            eprintln!("Failed for {lhs} - {rhs}");
            if let Ok(sub2) = sub2 {
                eprintln!("(\n\t{lhs:X} - \n\t{rhs:X} = \n\t{sub:X} != \n\t{sub2:X})");
            }
            panic!("");
        }
    }
}
