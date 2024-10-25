#![no_std]

const BYTES: usize = 25_783 + 1; // 206_265 / 8 + 1

// little-endian
#[allow(non_camel_case_types, reason = "foolish little rust-analyser...")]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct u206265([u8; BYTES]); // last byte should only use one bit

pub const MIN: u206265 = u206265([0; BYTES]);
pub const MAX: u206265 = u206265({
    let mut all_max = [0xff; BYTES];
    all_max[0] = 0b1;
    all_max
});

mod pure_rust_impl;
use pure_rust_impl::create_bytes;

macro_rules! impl_from_unsigned {
    ($type:ty) => {
        impl From<$type> for u206265 {
            fn from(value: $type) -> Self {
                create_bytes(value.to_le_bytes())
            }
        }

        impl<'from> From<&'from $type> for u206265 {
            fn from(value: &$type) -> Self {
                create_bytes(value.to_le_bytes())
            }
        }
    };
}

impl_from_unsigned!(u8);
impl_from_unsigned!(u16);
impl_from_unsigned!(u32);
impl_from_unsigned!(u64);
impl_from_unsigned!(u128);
impl_from_unsigned!(usize);

macro_rules! impl_from_signed {
    ($itype:ty, $utype:ty) => {
        impl TryFrom<$itype> for u206265 {
            type Error = <$utype as TryFrom<$itype>>::Error;

            fn try_from(value: $itype) -> Result<Self, Self::Error> {
                let unsigned: $utype = value.try_into()?;
                Ok(u206265::from(unsigned))
            }
        }
    };
}

impl_from_signed!(i8, u8);
impl_from_signed!(i16, u16);
impl_from_signed!(i32, u32);
impl_from_signed!(i64, u64);
impl_from_signed!(i128, u128);
impl_from_signed!(isize, usize);

#[allow(non_camel_case_types, reason = "foolish little rust-analyser...")]
pub struct u206265ToUnsigned {
    pub min_bytes: usize,
}

macro_rules! impl_try_from_unsigned {
    ($type:ty) => {
        impl TryFrom<u206265> for $type {
            type Error = u206265ToUnsigned;

            fn try_from(u206265(value): u206265) -> Result<Self, Self::Error> {
                let bytes = *value
                    .last_chunk()
                    .expect("Primitive integers should not be larger than 206265 bytes");
                let first_significant = value.iter().copied().position(|b| b > 0).unwrap_or(BYTES);
                let significant_length = BYTES - first_significant;
                if significant_length > bytes.len() {
                    return Err(u206265ToUnsigned {
                        min_bytes: significant_length,
                    });
                }
                Ok(Self::from_le_bytes(bytes))
            }
        }

        impl<'from> TryFrom<&'from u206265> for $type {
            type Error = u206265ToUnsigned;

            fn try_from(u206265(value): &u206265) -> Result<Self, Self::Error> {
                let bytes = *value
                    .last_chunk()
                    .expect("Primitive integers should not be larger than 206265 bytes");
                let first_significant = value.iter().copied().position(|b| b > 0).unwrap_or(BYTES);
                let significant_length = BYTES - first_significant;
                if significant_length > bytes.len() {
                    return Err(u206265ToUnsigned {
                        min_bytes: significant_length,
                    });
                }
                Ok(Self::from_le_bytes(bytes))
            }
        }
    };
}

impl_try_from_unsigned!(u8);
impl_try_from_unsigned!(u16);
impl_try_from_unsigned!(u32);
impl_try_from_unsigned!(u64);
impl_try_from_unsigned!(u128);
impl_try_from_unsigned!(usize);

#[allow(non_camel_case_types, reason = "foolish little rust-analyser...")]
pub enum u206265ToSigned<Unsigned, Signed: TryFrom<Unsigned>> {
    Unsigned(u206265ToUnsigned),
    Sign(Signed::Error),
}

macro_rules! impl_try_from_signed {
    ($utype:ty, $itype:ty) => {
        impl TryFrom<u206265> for $itype {
            type Error = u206265ToSigned<$utype, $itype>;

            fn try_from(value: u206265) -> Result<Self, Self::Error> {
                let unsigned: $utype = value.try_into().map_err(u206265ToSigned::Unsigned)?;
                unsigned.try_into().map_err(u206265ToSigned::Sign)
            }
        }
    };
}

impl_try_from_signed!(u8, i8);
impl_try_from_signed!(u16, i16);
impl_try_from_signed!(u32, i32);
impl_try_from_signed!(u64, i64);
impl_try_from_signed!(u128, i128);
impl_try_from_signed!(usize, isize);
