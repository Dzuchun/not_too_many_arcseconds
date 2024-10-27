use rand::{thread_rng, Rng};

use crate::u206265;

use super::ITERATIONS;

#[test]
fn ilog() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let val: u128 = rng.gen();
        let base: u128 = rng.r#gen();
        let log = val.checked_ilog(base);

        let the_val = u206265::from(val);
        let the_base = u206265::from(base);

        // act
        let log2 = crate::const_ilog(&the_val, &the_base);

        // assert
        assert_eq!(log, log2, "For log({base}, {val})");
    }
}

macro_rules! test_ilog_special {
    ($value:literal, $base:literal) => {
        ::paste::paste! {
            #[test]
            fn [<special_ilog_for_ $value _base_ $base>]() {
                // arrange
                let value = $value;
                let the_value = $crate::u206265::from(value);
                let base = $base;
                let the_base = $crate::u206265::from(base);

                // act
                let log = value.checked_ilog(base);
                let the_log = $crate::const_ilog(&the_value, &the_base);

                // assert
                assert_eq!(log, the_log);
            }
        }
    };
}

test_ilog_special!(9u128, 3);
test_ilog_special!(10u128, 3);
test_ilog_special!(10u128, 10);
test_ilog_special!(11u128, 10);
test_ilog_special!(100u128, 10);
test_ilog_special!(999_999_999_999u128, 10);

#[test]
fn ilog10() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let val: u128 = rng.gen();
        let log = val.checked_ilog10();

        let the_val = u206265::from(val);

        // act
        let log2 = crate::const_ilog10(&the_val);

        // assert
        assert_eq!(log, log2, "For log10({val})");
    }
}

#[test]
fn ilog2() {
    let mut rng = thread_rng();

    for _ in 0..ITERATIONS {
        // arrange
        let val: u128 = rng.gen();
        let log = val.checked_ilog2();

        let the_val = u206265::from(val);

        // act
        let log2 = crate::const_ilog2(&the_val);

        // assert
        assert_eq!(log, log2, "For log2({val})");
    }
}

macro_rules! test_ilog2_special {
    ($value:literal) => {
        ::paste::paste! {
            #[test]
            fn [<special_ilog2_for_ $value>]() {
                // arrange
                let value = $value;
                let the_value = $crate::u206265::from(value);

                // act
                let log = value.checked_ilog2();
                let the_log = $crate::const_ilog2(&the_value);

                // assert
                assert_eq!(log, the_log);
            }
        }
    };
}

test_ilog2_special!(0u8);
test_ilog2_special!(1u8);
test_ilog2_special!(2u8);
test_ilog2_special!(3u8);
test_ilog2_special!(16u8);
test_ilog2_special!(17u8);
test_ilog2_special!(98_459_632_038_454_933_985_890_629_644_117_624_406_u128);
