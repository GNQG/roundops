use core::clone::Clone;
use core::marker::PhantomData;

use roundops::*;
use float_traits::*;

#[inline]
pub fn roughsucc_add<T: Abs<Output = T> + BinaryFloat + Clone>(f: T) -> T {
    f.clone() + ((T::eps() / T::radix() * (T::one() + T::eps())) * f.abs())
}

#[inline]
pub fn roughpred_add<T: Abs<Output = T> + BinaryFloat + Clone>(f: T) -> T {
    f.clone() - ((T::eps() / T::radix() * (T::one() + T::eps())) * f.abs())
}

#[inline]
pub fn roughsucc_mul<T: Abs<Output = T> + BinaryFloat + Underflow + Clone>(f: T) -> T {
    (f.clone() + T::unit_underflow()) + ((T::eps() / T::radix() * (T::one() + T::eps())) * f.abs())
}

#[inline]
pub fn roughpred_mul<T: Abs<Output = T> + BinaryFloat + Underflow + Clone>(f: T) -> T {
    (f.clone() - T::unit_underflow()) - ((T::eps() / T::radix() * (T::one() + T::eps())) * f.abs())
}

pub struct RoughWrapping<T>(PhantomData<fn(T)>);

impl<T: Abs<Output = T> + BinaryFloat + Infinite + Underflow + BoundedFloat + Clone> RoundAdd
    for RoughWrapping<T> {
    type Num = T;
    fn add_up(a: T, b: T) -> T {
        let x = a.clone() + b.clone();
        if x == T::neg_infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            roughsucc_add(x)
        }
    }
    fn add_down(a: T, b: T) -> T {
        let x = a.clone() + b.clone();
        if x == T::infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::max_value()
            }
        } else {
            roughpred_add(x)
        }
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Infinite + Underflow + BoundedFloat + Clone> RoundSub
    for RoughWrapping<T> {
    type Num = T;
    fn sub_up(a: T, b: T) -> T {
        let x = a.clone() - b.clone();
        if x == T::neg_infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            roughsucc_add(x)
        }
    }
    fn sub_down(a: T, b: T) -> T {
        let x = a.clone() - b.clone();
        if x == T::infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::max_value()
            }
        } else {
            roughpred_add(x)
        }
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Infinite + Underflow + BoundedFloat + Clone> RoundMul
    for RoughWrapping<T> {
    type Num = T;
    fn mul_up(a: T, b: T) -> T {
        let x = a.clone() * b.clone();
        if x == T::neg_infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            roughsucc_mul(x)
        }
    }
    fn mul_down(a: T, b: T) -> T {
        let x = a.clone() * b.clone();
        if x == T::infinity() {
            if a == T::infinity() || b == T::infinity() {
                x
            } else {
                T::max_value()
            }
        } else {
            roughpred_mul(x)
        }
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Infinite + Underflow + BoundedFloat + Clone> RoundDiv
    for RoughWrapping<T> {
    type Num = T;
    fn div_up(a: T, b: T) -> T {
        let x = a.clone() / b.clone();
        if x == T::neg_infinity() {
            if b == T::zero() || a.abs() == T::infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            roughsucc_mul(x)
        }
    }
    fn div_down(a: T, b: T) -> T {
        let x = a.clone() / b.clone();
        if x == T::infinity() {
            if b == T::zero() || a.abs() == T::infinity() {
                x
            } else {
                T::max_value()
            }
        } else {
            roughpred_mul(x)
        }
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Infinite + Underflow + BoundedFloat + Sqrt<Output = T> + Clone> RoundSqrt
    for RoughWrapping<T> {
    fn sqrt_up(a: T) -> T {
        let r = a.sqrt();
/*
 *  may fail with ridicuous floating point format
 *  example:
 *  T::min_exponent() == -1 && T::bits() == 11
 *  roughsucc_add(T::unit_underflow().sqrt()) = sqrt(2^{-11}) is not representable
 *  as exact T and less than T::min_positive().
 *  roughsucc_add(r)
 */
        roughsucc_mul(r)
    }
    fn sqrt_down(a: T) -> T {
        let r = a.sqrt();
        if r == T::infinity() {
            T::max_value()
        } else {
            roughpred_add(r)
        }
    }
}
