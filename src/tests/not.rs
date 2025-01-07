use core::ops::Not;

use crate::u206265;

quickcheck! {
    #[allow(clippy::clone_on_copy)]
    fn not_not_should_equal(val: u128) -> bool {
        let val = u206265::from(val);
        let not_not = val.clone().not().not();

        val == not_not
    }

    #[allow(clippy::clone_on_copy)]
    fn not_xor_should_max(val: u128) -> bool {
        let val = u206265::from(val);
        let not = !(val.clone());

        (val ^ not) == u206265::MAX
    }

    #[allow(clippy::clone_on_copy)]
    fn not_and_should_zero(val: u128) -> bool {
        let val = u206265::from(val);
        let not = !(val.clone());

        (val & not) == u206265::ZERO
    }
}

macro_rules! not_not_should_equal_special {
    ($val:literal) => {
        ::paste::paste! {
            #[test]
            fn [<special_not_not_should_equal_for_ $val:lower>]() {
                let val: u128 = $val;
                let val = u206265::from(val);
                let not_not = val.clone().not().not();

                assert_eq!(val, not_not);
            }
        }
    };
}

not_not_should_equal_special!(338_953_138_925_153_547_590_470_800_371_487_866_880);

macro_rules! not_xor_should_max_special {
    ($val:literal) => {
        ::paste::paste! {
            #[test]
            fn [<special_not_or_should_max_for_ $val:lower>]() {
                let val: u128 = $val;
                let val = u206265::from(val);
                let not = !(val.clone());

                assert_eq!((val ^ not), u206265::MAX);
            }
        }
    };
}

not_xor_should_max_special!(0);
