use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Percentage(Decimal);

impl From<Decimal> for Percentage {
    fn from(p: Decimal) -> Self {
        Percentage(p)
    }
}

impl From<f64> for Percentage {
    fn from(p: f64) -> Self {
        Percentage(Decimal::from_f64(p).unwrap())
    }
}

impl From<f32> for Percentage {
    fn from(p: f32) -> Self {
        Percentage(Decimal::from_f32(p).unwrap())
    }
}

impl From<&str> for Percentage {
    fn from(p: &str) -> Self {
        Percentage(Decimal::from_str(p).unwrap())
    }
}

impl fmt::Debug for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{}%", self.0 * Decimal::from(100)))
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl Mul for Percentage {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Percentage::from(self.0 * rhs.0)
    }
}

impl<'a, 'b> Mul<&'a Percentage> for &'b Percentage {
    type Output = Percentage;
    fn mul(self, rhs: &'a Percentage) -> Self::Output {
        Percentage::from(rhs.0 * self.0)
    }
}

impl Mul<Decimal> for Percentage {
    type Output = Decimal;
    fn mul(self, rhs: Decimal) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<Percentage> for Decimal {
    type Output = Decimal;
    fn mul(self, rhs: Percentage) -> Self::Output {
        rhs * self
    }
}

macro_rules! impl_mul {
    ($T:ty, $from_ty:path, $to_ty:path) => {
        impl Mul<$T> for Percentage {
            type Output = $T;
            fn mul(self, rhs: $T) -> Self::Output {
                let d: Decimal = $from_ty(rhs).unwrap();
                $to_ty(&(self.0 * d)).unwrap()
            }
        }
        impl Mul<Percentage> for $T {
            type Output = $T;
            fn mul(self, rhs: Percentage) -> Self::Output {
                let d: Decimal = $from_ty(self).unwrap();
                $to_ty(&(rhs.0 * d)).unwrap()
            }
        }
    };
}

impl_mul!(isize, FromPrimitive::from_isize, ToPrimitive::to_isize);
impl_mul!(i8, FromPrimitive::from_i8, ToPrimitive::to_i8);
impl_mul!(i16, FromPrimitive::from_i16, ToPrimitive::to_i16);
impl_mul!(i32, FromPrimitive::from_i32, ToPrimitive::to_i32);
impl_mul!(i64, FromPrimitive::from_i64, ToPrimitive::to_i64);
impl_mul!(usize, FromPrimitive::from_usize, ToPrimitive::to_usize);
impl_mul!(u8, FromPrimitive::from_u8, ToPrimitive::to_u8);
impl_mul!(u16, FromPrimitive::from_u16, ToPrimitive::to_u16);
impl_mul!(u32, FromPrimitive::from_u32, ToPrimitive::to_u32);
impl_mul!(u64, FromPrimitive::from_u64, ToPrimitive::to_u64);
impl_mul!(f64, FromPrimitive::from_f64, ToPrimitive::to_f64);
impl_mul!(f32, FromPrimitive::from_f32, ToPrimitive::to_f32);

impl Add for Percentage {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.0 + rhs.0;

        Percentage::from(sum)
    }
}

impl Add<Decimal> for Percentage {
    type Output = Self;
    fn add(self, rhs: Decimal) -> Self::Output {
        self + Percentage::from(rhs)
    }
}

impl Add<Percentage> for Decimal {
    type Output = Percentage;
    fn add(self, rhs: Percentage) -> Self::Output {
        rhs + Percentage::from(self)
    }
}

macro_rules! impl_add {
    ($T:ty, $from_ty:path) => {
        impl Add<$T> for Percentage {
            type Output = Percentage;
            fn add(self, rhs: $T) -> Self::Output {
                let d: Decimal = $from_ty(rhs).unwrap();
                self + Percentage::from(d)
            }
        }
        impl Add<Percentage> for $T {
            type Output = Percentage;
            fn add(self, rhs: Percentage) -> Self::Output {
                let d: Decimal = $from_ty(self).unwrap();
                rhs + Percentage::from(d)
            }
        }
    };
}

impl_add!(isize, FromPrimitive::from_isize);
impl_add!(i8, FromPrimitive::from_i8);
impl_add!(i16, FromPrimitive::from_i16);
impl_add!(i32, FromPrimitive::from_i32);
impl_add!(i64, FromPrimitive::from_i64);
impl_add!(usize, FromPrimitive::from_usize);
impl_add!(u8, FromPrimitive::from_u8);
impl_add!(u16, FromPrimitive::from_u16);
impl_add!(u32, FromPrimitive::from_u32);
impl_add!(u64, FromPrimitive::from_u64);
impl_add!(f64, FromPrimitive::from_f64);
impl_add!(f32, FromPrimitive::from_f32);

impl Sub for Percentage {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let dif = self.0 - rhs.0;

        Percentage::from(dif)
    }
}

macro_rules! impl_sub {
    ($T:ty, $from_ty:path) => {
        impl Sub<$T> for Percentage {
            type Output = Percentage;
            fn sub(self, rhs: $T) -> Self::Output {
                let d: Decimal = $from_ty(rhs).unwrap();
                self - Percentage::from(d)
            }
        }
        impl Sub<Percentage> for $T {
            type Output = Percentage;
            fn sub(self, rhs: Percentage) -> Self::Output {
                let d: Decimal = $from_ty(self).unwrap();
                Percentage::from(d) - rhs
            }
        }
    };
}

impl_sub!(isize, FromPrimitive::from_isize);
impl_sub!(i8, FromPrimitive::from_i8);
impl_sub!(i16, FromPrimitive::from_i16);
impl_sub!(i32, FromPrimitive::from_i32);
impl_sub!(i64, FromPrimitive::from_i64);
impl_sub!(usize, FromPrimitive::from_usize);
impl_sub!(u8, FromPrimitive::from_u8);
impl_sub!(u16, FromPrimitive::from_u16);
impl_sub!(u32, FromPrimitive::from_u32);
impl_sub!(u64, FromPrimitive::from_u64);
impl_sub!(f64, FromPrimitive::from_f64);
impl_sub!(f32, FromPrimitive::from_f32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentage() {
        let p1 = Percentage::from(0.5);
        let p2 = Percentage::from("0.00000015");
        let p3 = Percentage::from(0.2);
        let p4 = Percentage::from(1.2);

        assert_eq!(1 - p1, Percentage::from(0.5));
        assert_eq!(p1 - 0.2, Percentage::from(0.3));
        assert_eq!(1u8 - p1, Percentage::from(0.5));
        assert_eq!(p1 - 0.2f32, Percentage::from(0.3));
        assert_eq!(1 + p1, Percentage::from(1.5));
        assert_eq!(p1 + 1, Percentage::from(1.5));

        assert_eq!(p1 - p3, Percentage::from(0.3));
        assert_eq!(p1 + p2, Percentage::from("0.50000015"));
        assert_eq!(p4 - p3, 1.0.into());

        assert_eq!(p1 * 100, 50);
        assert_eq!(p1 * 100u8, 50u8);
        assert_eq!(p1 * -100i8, -50i8);
        assert_eq!(p1 * 100u16, 50u16);
        assert_eq!(p1 * -100i16, -50i16);
        assert_eq!(p1 * 100u32, 50u32);
        assert_eq!(p1 * -100i32, -50i32);
        assert_eq!(p1 * 100u64, 50u64);
        assert_eq!(p1 * -100i64, -50i64);
        assert_eq!(p1 * 100usize, 50usize);
        assert_eq!(p1 * -100isize, -50isize);
        assert_eq!(p1 * 90.0, 45.0);
        assert_eq!(p1 * 90.0f32, 45.0f32);

        assert_eq!(100 * p1, 50);
        assert_eq!(100u8 * p1, 50u8);
        assert_eq!(-100i8 * p1, -50i8);
        assert_eq!(100u16 * p1, 50u16);
        assert_eq!(-100i16 * p1, -50i16);
        assert_eq!(100u32 * p1, 50u32);
        assert_eq!(-100i32 * p1, -50i32);
        assert_eq!(100u64 * p1, 50u64);
        assert_eq!(-100i64 * p1, -50i64);
        assert_eq!(100usize * p1, 50usize);
        assert_eq!(-100isize * p1, -50isize);
        assert_eq!(90.0 * p1, 45.0);
        assert_eq!(90.0f32 * p1, 45.0f32);
    }
}
