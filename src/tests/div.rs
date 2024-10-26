use rand::{thread_rng, Rng};

use crate::u206265;

use super::ITERATIONS;

#[cfg_attr(debug_assertions, ignore)]
#[test]
fn div() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let lhs: u128 = rng.gen();
        let rhs: u128 = rng.gen();
        let div = lhs.checked_div(rhs);
        let rem = lhs.checked_rem(rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let the_res = crate::const_div(&the_lhs, &the_rhs);

        // assert
        assert!(
            !(the_res.is_some() != div.is_some() || (the_res.is_some() != rem.is_some())),
            "Option variants are not the same:\n{lhs} / {rhs}\n({div:?}, {rem:?})"
        );
        let (Some(div), Some(rem), Some((the_div, the_rem))) = (div, rem, the_res) else {
            continue;
        };
        let div2 = u128::try_from(the_div).unwrap();
        let rem2 = u128::try_from(the_rem).unwrap();
        assert_eq!(div, div2, "Failed comparing divs");
        assert_eq!(rem, rem2, "Failed comparing remainders");
    }
}
