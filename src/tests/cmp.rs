use rand::{thread_rng, Rng};

use crate::u206265;

use super::ITERATIONS;

#[test]
fn cmp() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let lhs: u128 = rng.r#gen();
        let rhs: u128 = rng.r#gen();
        let comp = lhs.cmp(&rhs);

        let the_lhs = u206265::from(lhs);
        let the_rhs = u206265::from(rhs);

        // act
        let the_comp = crate::const_cmp(&the_lhs, &the_rhs);

        // assert
        assert_eq!(comp, the_comp, "Failed to compare {lhs} and {rhs}\n\t{lhs:X}\n\t{rhs:X}\n\t\tExpected: {comp:?}\n\t\tActual: {the_comp:?}");
    }
}
