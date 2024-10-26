#![no_std]

#[cfg(test)]
extern crate std;

const BITS: usize = 206_265;
const BYTES: usize = BITS / 8 + (if (BITS & 0b111) > 0 { 1 } else { 0 }); // 206_265 / 8 + 1

// little-endian
#[allow(non_camel_case_types, reason = "foolish little rust-analyser...")]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct u206265([u8; BYTES]); // last byte should only use one bit

pub const MIN: u206265 = u206265([0; BYTES]);
pub const MAX: u206265 = u206265({
    let mut all_max = [0xff; BYTES];
    all_max[BYTES - 1] = 0b1;
    all_max
});

impl u206265 {
    pub const fn significant_bytes(&self) -> usize {
        let mut i = BYTES - 1;
        loop {
            if self.0[i] > 0 {
                return i + 1;
            }
            if i == 0 {
                return 1;
            }
            i -= 1;
        }
    }

    pub fn significant_bytes_slice(&self) -> &[u8] {
        &self.0[..self.significant_bytes()]
    }
}

mod pure_rust_impl;

use core::ops::{Add, AddAssign};

use pure_rust_impl::{
    const_add, const_cmp, const_mul, const_shl, const_shr, const_sub, create_bytes,
};

macro_rules! impl_from_unsigned {
    ($type:ty) => {
        impl From<$type> for u206265 {
            #[inline]
            fn from(value: $type) -> Self {
                create_bytes(value.to_le_bytes())
            }
        }

        impl<'from> From<&'from $type> for u206265 {
            #[inline]
            fn from(value: &$type) -> Self {
                Self::from(*value)
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

            #[inline]
            fn try_from(value: $itype) -> Result<Self, Self::Error> {
                let unsigned: $utype = value.try_into()?;
                Ok(u206265::from(unsigned))
            }
        }

        impl<'from> TryFrom<&'from $itype> for u206265 {
            type Error = <$utype as TryFrom<$itype>>::Error;

            #[inline]
            fn try_from(&value: &$itype) -> Result<Self, Self::Error> {
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct u206265ToUnsigned {
    pub min_bytes: usize,
}

macro_rules! impl_try_from_unsigned {
    ($type:ty) => {
        impl TryFrom<u206265> for $type {
            type Error = u206265ToUnsigned;

            fn try_from(value: u206265) -> Result<Self, Self::Error> {
                let bytes = *value
                    .0
                    .first_chunk()
                    .expect("Primitive integers should not be larger than 206265 bytes");
                let significant_length = value.significant_bytes();
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

            #[inline]
            fn try_from(value: &u206265) -> Result<Self, Self::Error> {
                Self::try_from(*value)
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'lhs, 'rhs> Add<&'rhs u206265> for &'lhs u206265 {
    type Output = u206265;

    #[inline]
    fn add(self, rhs: &'rhs u206265) -> Self::Output {
        let (result, overflow) = const_add(self, rhs);
        debug_assert!(!overflow, "u206265 add overflow!");
        result
    }
}

impl<'rhs> Add<&'rhs u206265> for u206265 {
    type Output = u206265;

    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        let (sum, overflow) = const_add(&self, rhs);
        debug_assert!(!overflow, "u206265 add overflow!");
        sum
    }
}

impl Add for u206265 {
    type Output = u206265;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl<'rhs> AddAssign<&'rhs u206265> for u206265 {
    #[inline]
    fn add_assign(&mut self, rhs: &'rhs u206265) {
        let (sum, overflow) = const_add(self, rhs);
        debug_assert!(!overflow, "u206265 add overflow!");
        *self = sum;
    }
}

impl AddAssign for u206265 {
    #[inline]
    fn add_assign(&mut self, rhs: u206265) {
        *self += &rhs;
    }
}

#[cfg(test)]
mod tests;
