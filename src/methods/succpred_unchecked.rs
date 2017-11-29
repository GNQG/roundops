use core::f64;
use core::marker::PhantomData;

use roundops::*;
use utils::{succ, pred};

pub struct SuccPredUnchecked<T>(PhantomData<fn(T)>);

impl RoundAdd for SuccPredUnchecked<f64> {
    type Num = f64;
    #[inline]
    fn add_up(a: f64, b: f64) -> f64 {
        succ(a + b)
    }
    #[inline]
    fn add_down(a: f64, b: f64) -> f64 {
        pred(a + b)
    }
}

impl RoundSub for SuccPredUnchecked<f64> {
    type Num = f64;
    #[inline]
    fn sub_up(a: f64, b: f64) -> f64 {
        succ(a - b)
    }
    #[inline]
    fn sub_down(a: f64, b: f64) -> f64 {
        pred(a - b)
    }
}

impl RoundMul for SuccPredUnchecked<f64> {
    type Num = f64;
    #[inline]
    fn mul_up(a: f64, b: f64) -> f64 {
        succ(a * b)
    }
    #[inline]
    fn mul_down(a: f64, b: f64) -> f64 {
        pred(a * b)
    }
}

impl RoundDiv for SuccPredUnchecked<f64> {
    type Num = f64;
    #[inline]
    fn div_up(a: f64, b: f64) -> f64 {
        succ(a / b)
    }
    #[inline]
    fn div_down(a: f64, b: f64) -> f64 {
        pred(a / b)
    }
}

impl RoundSqrt for SuccPredUnchecked<f64> {
    #[inline]
    fn sqrt_up(a: f64) -> f64 {
        succ(a.sqrt())
    }
    #[inline]
    fn sqrt_down(a: f64) -> f64 {
        pred(a.sqrt())
    }
}
