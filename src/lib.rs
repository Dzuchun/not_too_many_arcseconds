#![no_std]

#[cfg(test)]
extern crate std;

const BITS: usize = 206_265;
const BITS_U32: u32 = {
    use crate::BITS;
    let Some(max_u32) = 1usize.checked_shl(31) else {
        panic!("usize is less than 31 bits");
    };
    assert!(!(BITS > max_u32), "BITS should not be greater than 31 bits");
    #[allow(
        clippy::cast_possible_truncation,
        reason = "This cast was validated above - BITS is 31 bits at most"
    )]
    {
        BITS as u32
    }
};
const BYTES: usize = BITS / 8 + (if (BITS & 0b111) > 0 { 1 } else { 0 }); // 206_265 / 8 + 1

// little-endian
#[allow(non_camel_case_types, reason = "foolish little rust-analyser...")]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct u206265([u8; BYTES]); // last byte should only use one bit

impl u206265 {
    pub const MIN: u206265 = create_bytes([]);
    pub const ZERO: u206265 = Self::MIN;
    pub const ONE: u206265 = create_bytes([0x01]);
    pub const MAX: u206265 = u206265({
        let mut all_max = [0xff; BYTES];
        all_max[BYTES - 1] = 0b1;
        all_max
    });

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

    pub const fn significant_bytes_u32(&self) -> u32 {
        let res_usize = self.significant_bytes();
        debug_assert!(res_usize < BYTES);
        let _ = &BITS_U32;
        #[allow(
            clippy::cast_possible_truncation,
            reason = "BITS_U32 exists, and usize result is less than total number of bytes - it's safe to cast"
        )]
        let res = res_usize as u32;
        res
    }

    pub fn significant_bytes_slice(&self) -> &[u8] {
        &self.0[..self.significant_bytes()]
    }
}

mod pure_rust_impl;

use core::ops::{Add, AddAssign};

use pure_rust_impl::{
    const_add, const_bit_and, const_bit_or, const_bit_xor, const_cmp, const_div, const_ilog,
    const_ilog10, const_ilog2, const_mul, const_shl, const_shr, const_sub, create_bytes,
};

macro_rules! impl_from_unsigned {
    ($type:ty) => {
        ::paste::paste! {
            impl u206265 {
                #[inline]
                pub const fn [<from_ $type>](value: $type) -> Self {
                    create_bytes(value.to_le_bytes())
                }

                #[inline]
                pub const fn [<try_from_ $type>](value: $type) -> Option<Self> {
                    Some(Self::[<from_ $type>](value))
                }

                #[inline]
                pub const fn [<try_into_ $type>](self) -> Result<$type, u206265ToUnsigned> {
                    const BITS_U32: u32 = $type::BITS;
                    const TYPE_BITS: usize = BITS_U32 as usize;
                    const TYPE_BYTES: usize = TYPE_BITS >> 3;
                    use ::const_for::const_for;
                    let mut bytes: [u8; TYPE_BYTES] = [0u8; TYPE_BYTES];
                    const_for!(i in 0..TYPE_BYTES => {
                        bytes[i] = self.0[i];
                    });
                    let significant_length = self.significant_bytes();
                    if significant_length > bytes.len() {
                        return Err(u206265ToUnsigned {
                            min_bytes: significant_length,
                        });
                    }
                    Ok($type::from_le_bytes(bytes))
                }
            }
        }

        impl From<$type> for u206265 {
            #[inline]
            fn from(value: $type) -> Self {
                ::paste::paste! {
                    Self::[<from_ $type>](value)
                }
            }
        }

        impl<'from> From<&'from $type> for u206265 {
            #[inline]
            fn from(&value: &$type) -> Self {
                ::paste::paste! {
                    Self::[<from_ $type>](value)
                }
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

pub struct NegativeIntError(());

macro_rules! impl_from_signed {
    ($itype:ty, $utype:ty) => {
        ::paste::paste! {
            impl u206265 {
                #[inline]
                pub const fn [<try_from_ $itype>](value: $itype) -> Option<Self> {
                    if value >= 0 {
                        #[allow(clippy::cast_sign_loss, reason = "We're checking right above for that")]
                        let uvalue = value as $utype;
                        Some(Self::[<from_ $utype>](uvalue))
                    } else {
                        None
                    }
                }
            }
        }

        impl TryFrom<$itype> for u206265 {
            type Error = NegativeIntError;

            #[inline]
            fn try_from(value: $itype) -> Result<Self, Self::Error> {
                ::paste::paste!{
                    Self::[<try_from_ $itype>](value).ok_or(NegativeIntError(()))
                }
            }
        }

        impl<'from> TryFrom<&'from $itype> for u206265 {
            type Error = NegativeIntError;

            #[inline]
            fn try_from(&value: &$itype) -> Result<Self, Self::Error> {
                ::paste::paste!{
                    Self::[<try_from_ $itype>](value).ok_or(NegativeIntError(()))
                }
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
                ::paste::paste! {
                    u206265::[<try_into_ $type>](value)
                }
            }
        }

        impl<'from> TryFrom<&'from u206265> for $type {
            type Error = u206265ToUnsigned;

            #[inline]
            fn try_from(&value: &u206265) -> Result<Self, Self::Error> {
                ::paste::paste! {
                    u206265::[<try_into_ $type>](value)
                }
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
