use crate::u206265;

quickcheck! {
    fn lower_hex(val: u128) -> bool {
        format!("{:x}", val) == format!("{:x}", u206265::from(val))
    }
}

macro_rules! special_lower_hex {
    ($val:literal) => {
        ::paste::paste! {
            #[test]
            fn [<special_lower_hex_for_ $val:lower>]() {
                let val: u128 = $val;
                assert_eq!(format!("{:x}", val), format!("{:x}", u206265::from(val)))
            }
        }
    };
}

special_lower_hex!(256);
special_lower_hex!(272);
special_lower_hex!(1792);

quickcheck! {
    fn upper_hex(val: u128) -> bool {
        format!("{:X}", val) == format!("{:X}", u206265::from(val))
    }
}

quickcheck! {
    fn display(val: u128) -> bool {
        format!("{}", val) == format!("{}", u206265::from(val))
    }
}

macro_rules! special_display {
    ($val:literal) => {
        ::paste::paste! {
            #[test]
            fn [<special_display_for_ $val:lower>]() {
                let val: u128 = $val;
                assert_eq!(format!("{}", val), format!("{}", u206265::from(val)))
            }
        }
    };
}

special_display!(256);
special_display!(46_350_398_256_093_263_243_789_728_937_492_374_032);
special_display!(100_000_000_000_000_000_000_000_000_000_000_000_000);
special_display!(256_000_000_000_000_000_000_000_000_000_000_000_000);
special_display!(300_000_000_000_000_000_000_000_000_000_000_000_000);

#[test]
fn special_display_for_max() {
    let val: u128 = u128::MAX;
    assert_eq!(format!("{}", val), format!("{}", u206265::from(val)));
}
