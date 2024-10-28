use rand::{thread_rng, Rng};

use crate::u206265;

use super::ITERATIONS;

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
                    let the_u206265 = u206265::[<try_from_$type>](input);

                    // assert
                    #[allow(unused_comparisons, reason = "This macro tests both signed and unsigned types")]
                    {assert_eq!(the_u206265.is_some(), input >= 0);}
                    #[allow(irrefutable_let_patterns, reason = "This macro tests both signed and unsigned types")]
                    let Some(the_u206265) = the_u206265 else {
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

macro_rules! test_from_repr {
    ($from:expr, [$($expected:literal), +]) => {
        ::paste::paste!{
            #[test]
            #[allow(non_snake_case, reason = "These names are generated, and thus allowed to be weird")]
            fn [<repr_as_ $($expected)_+>]() {
                // arrange
                let from = $from;

                // act
                let result = u206265::from(from);

                // assert
                assert_eq!(result.significant_bytes_slice(), [$($expected),+]);
            }
        }
    };
}

test_from_repr!(1u8, [1]);
test_from_repr!(0xAA_BBu32, [0xBB, 0xAA]);
