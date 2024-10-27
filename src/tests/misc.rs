macro_rules! test_singificant_bytes {
    ($input:literal, $expected:literal) => {
        ::paste::paste! {
            #[test]
            #[allow(non_snake_case, reason = "This name is generated, have mercy")]
            fn [<significant_bytes_for_ $input _should_ $expected>]() {
                // arrange
                let input = $input;
                let expected = $expected;

                // act
                let value = $crate::u206265::from(input);
                let result = value.significant_bytes();

                // assert
                assert_eq!(result, expected);
            }
        }
    };
}

test_singificant_bytes!(0u8, 1);
test_singificant_bytes!(1u8, 1);
test_singificant_bytes!(255u8, 1);
test_singificant_bytes!(0x00FFu16, 1);
test_singificant_bytes!(0x01FFu16, 2);
