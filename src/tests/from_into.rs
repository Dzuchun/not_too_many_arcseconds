use crate::u206265;

macro_rules! test_from_into {
    ($type:ty) => {
        ::paste::paste! {
            quickcheck! {
                fn [<from_into_ $type>](input: $type) -> bool {
                    // FROM
                    // act
                    let the_u206265 = u206265::[<try_from_ $type>](input);

                    // assert
                    let Some(the_u206265) = the_u206265 else {
                        return input < 0;
                    };

                    // INTO
                    // act
                    let back = $type::try_from(&the_u206265);

                    // assert
                    back == Ok(input)
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

macro_rules! special_from_into {
    ($input:ident, $type:ty) => {
        ::paste::paste! {
            #[test]
            fn [<special_from_into_ $input:lower>]() {
                let input: $type = $input;
                // FROM
                // act
                let the_u206265 = u206265::[<try_from_ $type>](input);

                // assert
                let Some(the_u206265) = the_u206265 else {
                    #[allow(unused_comparisons)]
                    {if input >= 0 {
                        panic!("Could not convert positive value {input}");
                    } else {
                        return;
                    }}
                };

                // INTO
                // act
                let back = $type::try_from(&the_u206265);

                // assert
                assert_eq!(back, Ok(input))
            }
        }
    };
}

const U128_MAX: u128 = u128::MAX;
special_from_into!(U128_MAX, u128);

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
