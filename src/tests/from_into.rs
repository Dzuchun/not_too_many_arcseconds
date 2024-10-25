use rand::{thread_rng, Rng};

use crate::u206265;

const ITERATIONS: usize = 10_000;

macro_rules! test_from_into {
    ($type:ty) => {
        ::paste::paste! {
            #[test]
            fn [<from_into_ $type>]() {
                let mut rng = thread_rng();
                for _ in 0..ITERATIONS {
                    // arrange
                    let input: $type = rng.r#gen();

                    // FORM
                    // act
                    let the_u206265 = u206265::try_from(input);

                    // assert
                    #[allow(unused_comparisons)]
                    {assert_eq!(the_u206265.is_ok(), input >= 0);}
                    #[allow(irrefutable_let_patterns)]
                    let Ok(the_u206265) = the_u206265 else {
                        continue;
                    };

                    // INTO
                    // act
                    let back = $type::try_from(the_u206265);

                    // assert
                    assert_eq!(
                        back,
                        Ok(input),
                        "From-Into convertion should return identical value"
                    );
                }
            }
        }
    };
}

test_from_into!(u8);
test_from_into!(u16);
test_from_into!(u32);
test_from_into!(u64);
test_from_into!(u128);
test_from_into!(usize);

test_from_into!(i8);
test_from_into!(i16);
test_from_into!(i32);
test_from_into!(i64);
test_from_into!(i128);
test_from_into!(isize);
