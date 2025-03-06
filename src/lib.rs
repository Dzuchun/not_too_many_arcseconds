#![no_std]
#![doc = include_str!("../README.md")]

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
/// An unsigned, 206265-bit integer.
///
/// Functions about how you would expect:
///
/// ```rust
/// # use not_too_many_arcseconds::u206265;
/// let a = u206265::from(2u8);
/// let b = u206265::from(3u128);
///
/// assert_eq!(a + b, 5u32.into());
/// ```
#[allow(non_camel_case_types, reason = "foolish little rust-analyser...")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "copy", derive(Copy))]
pub struct u206265([u8; BYTES]); // last byte should only use one bit

impl u206265 {
    /// A minimum value
    ///
    /// Equals zero
    pub const MIN: u206265 = create_bytes([]);
    #[allow(missing_docs)]
    pub const ZERO: u206265 = Self::MIN;
    #[allow(missing_docs)]
    pub const ONE: u206265 = create_bytes([0x01]);
    /// A maximum value
    ///
    /// Equals $2^{206265} - 1$
    pub const MAX: u206265 = u206265({
        let mut all_max = [0xff; BYTES];
        all_max[BYTES - 1] = 0b1;
        all_max
    });

    /// Number of bytes actually containing something.
    ///
    /// You are unlikely to find any use in this method, but it is here, in case you need it.
    #[inline]
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

    /// Same as [`Self::significant_bytes`], but returns `u32` instead of `usize`.
    #[inline]
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

    /// A byte slice of internal memory, containing integer's data.
    #[inline]
    pub fn significant_bytes_slice(&self) -> &[u8] {
        &self.0[..self.significant_bytes()]
    }

    /// Same as [`Clone::clone`], but `const`.
    #[inline]
    pub const fn const_clone(&self) -> Self {
        Self(self.0)
    }
}

mod pure_rust_impl;

use core::{
    fmt::{Display, LowerHex, UpperHex},
    iter::{Product, Sum},
    ops::Not,
};

pub use pure_rust_impl::{
    const_add, const_add_assign, const_bitand, const_bitand_assign, const_bitor,
    const_bitor_assign, const_bitxor, const_bitxor_assign, const_cmp, const_div, const_div_assign,
    const_div_rem, const_ilog, const_ilog2, const_ilog10, const_mul, const_mul_assign,
    const_not_assign, const_rem, const_rem_assign, const_shl, const_shl_assign, const_shr,
    const_shr_assign, const_sub, const_sub_assign, create_bytes,
};

/// Represents failure to convert [`u206265`] into a smaller integer.
///
/// The only field is the number of bytes operation would require. For example, if `bytes_required = 1`, conversion to `u8` will fail, but it will succeed for `u16`, `u32`, etc.
#[allow(non_camel_case_types, reason = "foolish little rust-analyser...")]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct u206265ToUnsigned {
    /// Minimum number of bytes required for successful conversion.
    pub bytes_required: usize,
}

macro_rules! impl_unsigned {
    ($type:ty) => {
        ::paste::paste! {
            impl u206265 {
                #[doc = concat!(r#""Attempts" to create [`u206265`] from [`"#, stringify!($type), "`].")]
                #[doc = concat!(r#"Since [`u206265`] is larger than [`"#, stringify!($type), "`], this operation will always succeed. This method is only intended for compatibility")]
                #[inline]
                pub const fn [<try_from_ $type>](value: $type) -> Option<Self> {
                    Some(create_bytes(value.to_le_bytes()))
                }

                #[doc = concat!(r#"Creates [`u206265`] from [`"#, stringify!($type), "`].")]
                #[inline]
                pub const fn [<from_ $type>](value: $type) -> Self {
                    create_bytes(value.to_le_bytes())
                }

                #[doc = concat!(r#"Attempts to convert [`u206265`] into [`"#, stringify!($type), "`].")]
                #[doc = concat!(r#"This operation can fail, since [`"#, stringify!($type), "`] might not have enough bytes to hold all of the data. [`u206265ToUnsigned`] is returned in this case, describing how many bytes is required to store the number")]
                #[inline]
                pub const fn [<try_into_ $type>](&self) -> Result<$type, u206265ToUnsigned> {
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
                            bytes_required: significant_length,
                        });
                    }
                    Ok($type::from_le_bytes(bytes))
                }
            }

            impl From<$type> for u206265 {
                #[inline]
                fn from(value: $type) -> Self {
                    Self::[<from_ $type>](value)
                }
            }

            impl<'from> From<&'from $type> for u206265 {
                #[inline]
                fn from(&value: &$type) -> Self {
                    // copying "normal" integer, ok to do
                    Self::from(value)
                }
            }

            impl TryFrom<u206265> for $type {
                type Error = u206265ToUnsigned;

                #[inline]
                fn try_from(value: u206265) -> Result<Self, Self::Error> {
                    Self::try_from(&value)
                }
            }

            impl<'from> TryFrom<&'from u206265> for $type {
                type Error = u206265ToUnsigned;

                #[inline]
                fn try_from(value: &u206265) -> Result<Self, Self::Error> {
                    u206265:: [<try_into_ $type>](value)
                }
            }
        }
    };
}

impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(u128);
impl_unsigned!(usize);

/// A unit-type error returned if attempted to convert negative integer into [`u206265`].
#[derive(Debug)]
pub struct NegativeIntError(());

/// An error type for conversion of [`u206265`] into a signed integer.
///
/// Variants could be understood better, if you consider that this operation essentially has no steps:
/// - convert to unsigned integer
/// - cost to signed integer
///
/// variants correspond to error during one of the steps.
#[allow(non_camel_case_types, reason = "foolish little rust-analyser...")]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum u206265ToSigned {
    /// Failed to convert from [`u206265`] into unsigned int
    Unsigned(u206265ToUnsigned),
    /// Failed to convert from unsigned int into signed int
    Signed,
}

macro_rules! impl_signed {
    ($itype:ty, $utype:ty) => {
        ::paste::paste! {
            impl u206265 {
                #[doc = concat!(r#"Attempts to create [`u206265`] from [`"#, stringify!($itype), "`].")]
                #[doc = "Since [`u206265`] is unsigned, this operation errors on negative input"]
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

                #[doc = concat!(r#"Attempts to convert [`u206265`] into [`"#, stringify!($itype), "`].")]
                #[doc = "This operation can fail in two ways:"]
                #[doc = concat!(r#"- [`"#, stringify!($utype), "`] might not have enough bytes to hold all of the data")]
                #[doc = concat!(r#"- resulting [`"#, stringify!($utype), "`] might be too large for [`", stringify!($itype), "`]")]
                #[doc = "In both cases, [`u206265ToSigned`] is returned"]
                #[inline]
                pub const fn [<try_into_ $itype>](&self) -> Result<$itype, u206265ToSigned> {
                    let unsigned: $utype = match u206265::[<try_into_ $utype>](self) {
                        Ok(unsigned) => unsigned,
                        Err(err) => return Err(u206265ToSigned::Unsigned(err)),
                    };

                    let signed: $itype;
                    #[allow(clippy::cast_possible_wrap, reason = "It's being checked for right after")]
                    {signed = unsigned as $itype}
                    if signed >= 0 {
                        Ok(signed)
                    } else {
                        Err(u206265ToSigned::Signed)
                    }
                }
            }

            impl TryFrom<$itype> for u206265 {
                type Error = NegativeIntError;

                #[inline]
                fn try_from(value: $itype) -> Result<Self, Self::Error> {
                    Self::[<try_from_ $itype>](value).ok_or(NegativeIntError(()))
                }
            }

            impl<'from> TryFrom<&'from $itype> for u206265 {
                type Error = NegativeIntError;

                #[inline]
                fn try_from(&value: &$itype) -> Result<Self, Self::Error> {
                    Self::try_from(value)
                }
            }

            impl TryFrom<u206265> for $itype {
                type Error = u206265ToSigned;

                #[inline]
                fn try_from(value: u206265) -> Result<Self, Self::Error> {
                    <$itype as TryFrom<&u206265>>::try_from(&value)
                }
            }

            impl<'from> TryFrom<&'from u206265> for $itype {
                type Error = u206265ToSigned;

                #[inline]
                fn try_from(value: &u206265) -> Result<Self, Self::Error> {
                    u206265::[<try_into_ $itype>](value)
                }
            }
        }
    };
}

impl_signed!(i8, u8);
impl_signed!(i16, u16);
impl_signed!(i32, u32);
impl_signed!(i64, u64);
impl_signed!(i128, u128);
impl_signed!(isize, usize);

macro_rules! max_const {
    ($type:ty) => {
        ::paste::paste! {
            #[doc = concat!("A [`u206265`] value, corresponding to the MAX constant of [`", stringify!($type), "`]")]
            pub const [<MAX_ $type:upper>]: Self = Self::[<try_from_ $type>]($type::MAX).unwrap();
        }
    };
}

impl u206265 {
    max_const!(u8);
    max_const!(u16);
    max_const!(u32);
    max_const!(u64);
    max_const!(u128);
    max_const!(usize);

    max_const!(i8);
    max_const!(i16);
    max_const!(i32);
    max_const!(i64);
    max_const!(i128);
    max_const!(isize);
}

macro_rules! impl_op_common {
    ($op:ident) => {
        ::paste::paste! {
            impl<'rhs> ::core::ops::[<$op:camel Assign>]<&'rhs u206265> for u206265 {
                #[inline]
                fn [<$op:lower _assign>](&mut self, rhs: &'rhs u206265) {
                    [<const_ $op:lower _assign>](self, rhs);
                }
            }

            impl ::core::ops::[<$op:camel Assign>] for u206265 {
                #[inline]
                fn [<$op:lower _assign>](&mut self, rhs: u206265) {
                    <u206265 as ::core::ops::[<$op:camel Assign>]<&u206265>>::[<$op:lower _assign>](self, &rhs);
                }
            }

            impl<'lhs, 'rhs> ::core::ops::[<$op:camel>]<u206265> for &'lhs u206265 {
                type Output = u206265;

                #[inline]
                fn [<$op:lower>](self, rhs: u206265) -> Self::Output {
                    <&u206265 as ::core::ops::[<$op:camel>]>::[<$op:lower>](self, &rhs)
                }
            }

            impl<'rhs> ::core::ops::[<$op:camel>]<&'rhs u206265> for u206265 {
                type Output = u206265;

                #[inline]
                fn [<$op:lower>](self, rhs: &Self) -> Self::Output {
                    <&u206265 as ::core::ops::[<$op:camel>]>::[<$op:lower>](&self, rhs)
                }
            }

            impl ::core::ops::[<$op:camel>]<u206265> for u206265 {
                type Output = u206265;

                #[inline]
                fn [<$op:lower>](self, rhs: Self) -> Self::Output {
                    <&u206265 as ::core::ops::[<$op:camel>]>::[<$op:lower>](&self, &rhs)
                }
            }
        }
    };
}

impl_op_common!(Add);
impl_op_common!(Sub);
impl_op_common!(Mul);
impl_op_common!(Div);
impl_op_common!(Rem);
impl_op_common!(BitOr);
impl_op_common!(BitAnd);
impl_op_common!(BitXor);

macro_rules! impl_op_overflow {
    ($op:ident) => {
        ::paste::paste! {
            impl<'lhs, 'rhs> ::core::ops::[<$op:camel>]<&'rhs u206265> for &'lhs u206265 {
                type Output = u206265;

                #[inline]
                fn [<$op:lower>](self, rhs: &'rhs u206265) -> Self::Output {
                    let (result, overflow) = [<const_ $op:lower>](self, rhs);
                    debug_assert!(!overflow, concat!("u206265 ", stringify!([<$op:lower>]), " overflow"));
                    result
                }
            }
        }
    };
}

impl_op_overflow!(Add);
impl_op_overflow!(Sub);
impl_op_overflow!(Mul);

macro_rules! impl_op_division {
    ($op:ident) => {
        ::paste::paste! {
            impl<'lhs, 'rhs> ::core::ops::[<$op:camel>]<&'rhs u206265> for &'lhs u206265 {
                type Output = u206265;

                #[inline]
                fn [<$op:lower>](self, rhs: &'rhs u206265) -> Self::Output {
                    [<const_ $op:lower>](self, rhs).expect("Division by zero")
                }
            }
        }
    };
}

impl_op_division!(Div);
impl_op_division!(Rem);

macro_rules! impl_op {
    ($op:ident) => {
        ::paste::paste! {
            impl<'lhs, 'rhs> ::core::ops::[<$op:camel>]<&'rhs u206265> for &'lhs u206265 {
                type Output = u206265;

                #[inline]
                fn [<$op:lower>](self, rhs: &'rhs u206265) -> Self::Output {
                    [<const_ $op:lower>](self, rhs)
                }
            }
        }
    };
}

impl_op!(BitOr);
impl_op!(BitAnd);
impl_op!(BitXor);

macro_rules! impl_sh_rhs {
    ($op:ident, $rhs:ident) => {
        ::paste::paste! {
            impl ::core::ops::[<$op:camel Assign>]<$rhs> for u206265 {
                #[inline]
                fn [<$op:lower _assign>](&mut self, rhs: $rhs) {
                    [<const_ $op:lower _assign>](self, u32::try_from(rhs).expect("Shift overflow"));
                }
            }

            impl<'rhs> ::core::ops::[<$op:camel Assign>]<&'rhs $rhs> for u206265 {
                #[inline]
                fn [<$op:lower _assign>](&mut self, rhs: &$rhs) {
                    <u206265 as ::core::ops::[<$op:camel Assign>]<$rhs>>::[<$op:lower _assign>](self, rhs.clone());
                }
            }

            impl ::core::ops::[<$op:camel>]<$rhs> for u206265 {
                type Output = u206265;

                #[inline]
                fn [<$op:lower>](mut self, rhs: $rhs) -> Self::Output {
                    <u206265 as ::core::ops::[<$op:camel Assign>]<$rhs>>::[<$op:lower _assign>](&mut self, rhs);
                    self
                }
            }

            impl<'rhs> ::core::ops::[<$op:camel>]<&'rhs $rhs> for u206265 {
                type Output = u206265;

                #[inline]
                fn [<$op:lower>](self, rhs: &'rhs $rhs) -> Self::Output {
                    <u206265 as ::core::ops::[<$op:camel>]<$rhs>>::[<$op:lower>](self, rhs.clone())
                }
            }

            impl<'lhs> ::core::ops::[<$op:camel>]<$rhs> for &'lhs u206265 {
                type Output = u206265;

                #[inline]
                fn [<$op:lower>](self, rhs: $rhs) -> Self::Output {
                    <u206265 as ::core::ops::[<$op:camel>]<$rhs>>::[<$op:lower>](self.clone(), rhs)
                }
            }

            impl<'lhs, 'rhs> ::core::ops::[<$op:camel>]<&'rhs $rhs> for &'lhs u206265 {
                type Output = u206265;

                #[inline]
                fn [<$op:lower>](self, rhs: &'rhs $rhs) -> Self::Output {
                    <u206265 as ::core::ops::[<$op:camel>]<$rhs>>::[<$op:lower>](self.clone(), rhs.clone())
                }
            }
        }
    };
}

macro_rules! impl_sh {
    ($op:ident) => {
        impl_sh_rhs!($op, u8);
        impl_sh_rhs!($op, u16);
        impl_sh_rhs!($op, u32);
        impl_sh_rhs!($op, u64);
        impl_sh_rhs!($op, u128);
        impl_sh_rhs!($op, usize);

        impl_sh_rhs!($op, i8);
        impl_sh_rhs!($op, i16);
        impl_sh_rhs!($op, i32);
        impl_sh_rhs!($op, i64);
        impl_sh_rhs!($op, i128);
        impl_sh_rhs!($op, isize);

        impl_sh_rhs!($op, u206265);
    };
}

impl_sh!(Shl);
impl_sh!(Shr);

impl PartialOrd for u206265 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for u206265 {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        const_cmp(self, other)
    }
}

impl Not for u206265 {
    type Output = Self;

    #[inline]
    fn not(mut self) -> Self::Output {
        const_not_assign(&mut self);
        self
    }
}

impl Not for &u206265 {
    type Output = u206265;

    #[inline]
    fn not(self) -> Self::Output {
        self.const_clone().not()
    }
}

impl Sum for u206265 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = u206265::ZERO;
        for num in iter {
            sum += num;
        }
        sum
    }
}

impl Product for u206265 {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut prod = u206265::ONE;
        for num in iter {
            prod *= num;
        }
        prod
    }
}

impl LowerHex for u206265 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let highest_byte = self.significant_bytes().saturating_sub(1);
        <u8 as core::fmt::LowerHex>::fmt(&self.0[highest_byte], f)?;
        for i in (0..highest_byte).rev() {
            write!(f, "{:02x}", self.0[i])?;
        }
        Ok(())
    }
}

impl UpperHex for u206265 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let highest_byte = self.significant_bytes().saturating_sub(1);
        <u8 as core::fmt::UpperHex>::fmt(&self.0[highest_byte], f)?;
        for i in (0..highest_byte).rev() {
            write!(f, "{:02X}", self.0[i])?;
        }
        Ok(())
    }
}

impl Display for u206265 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const TEN_38: u206265 = u206265::from_u128(10u128.pow(38));
        const MAX_LENGTH: usize = 1612; // approx log10(u206265::MAX) / log10(u128::MAX)
        if self == &Self::ZERO {
            return write!(f, "{}", 0);
        }
        let mut buf: [u128; MAX_LENGTH] = [0; MAX_LENGTH];
        let mut buf_i = 0;
        let mut val = self.const_clone();

        while val != u206265::ZERO {
            let (div, rem) = const_div_rem(&val, &TEN_38).unwrap();
            val = div;
            buf[buf_i] = rem.try_into_u128().unwrap();
            buf_i += 1;
        }

        buf_i -= 1;
        write!(f, "{}", buf[buf_i])?;
        for i in (0..buf_i).rev() {
            write!(f, "{:038}", buf[i])?;
        }

        Ok(())
    }
}

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate quickcheck;

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests;
