use core::clone::Clone;
use core::marker::PhantomData;

use roundops::*;
use float_traits::*;

use super::roughwrap::{roughsucc_add, roughpred_add, roughsucc_mul, roughpred_mul};

pub struct RoughWrappingUnchecked<T>(PhantomData<fn(T)>);

impl<T: Abs<Output = T> + BinaryFloat + Clone> RoundAdd for RoughWrappingUnchecked<T> {
    type Num = T;
    #[inline]
    fn add_up(a: T, b: T) -> T {
        roughsucc_add(a + b)
    }
    #[inline]
    fn add_down(a: T, b: T) -> T {
        roughpred_add(a + b)
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Underflow + Clone> RoundSub for RoughWrappingUnchecked<T> {
    type Num = T;
    #[inline]
    fn sub_up(a: T, b: T) -> T {
        roughsucc_add(a - b)
    }
    #[inline]
    fn sub_down(a: T, b: T) -> T {
        roughpred_add(a - b)
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Underflow + Clone> RoundMul for RoughWrappingUnchecked<T> {
    type Num = T;
    #[inline]
    fn mul_up(a: T, b: T) -> T {
        roughsucc_mul(a * b)
    }
    #[inline]
    fn mul_down(a: T, b: T) -> T {
        roughpred_mul(a * b)
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Underflow + Clone> RoundDiv for RoughWrappingUnchecked<T> {
    type Num = T;
    #[inline]
    fn div_up(a: T, b: T) -> T {
        roughsucc_mul(a / b)
    }
    #[inline]
    fn div_down(a: T, b: T) -> T {
        roughpred_mul(a / b)
    }
}

impl<T: BinaryFloat + Abs<Output = T> + Underflow + Sqrt<Output = T> + Clone> RoundSqrt
    for RoughWrappingUnchecked<T> {
    #[inline]
    fn sqrt_up(a: T) -> T {
        roughsucc_mul(a.sqrt())
    }
    #[inline]
    fn sqrt_down(a: T) -> T {
        roughpred_mul(a.sqrt())
    }
}
