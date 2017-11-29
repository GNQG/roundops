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

mod tests {
    use super::SuccPredUnchecked;
    use roundops::*;
    use super::{succ, pred};

    type SPf64 = SuccPredUnchecked<f64>;

    #[test]
    fn addition() {
        let (a, b) = (pred(1.), pred(10.));
        let (x, y) = (SPf64::add_up(a, b), SPf64::add_down(a, b));
        assert!(y == pred(a + b) && succ(a + b) == x);
    }

    #[test]
    fn subtraction() {
        let (a, b) = (pred(1.), pred(10.));
        let (x, y) = (SPf64::sub_up(a, b), SPf64::sub_down(a, b));
        assert!(y == pred(a - b) && succ(a - b) == x);
    }

    #[test]
    fn multiplication() {
        let (a, b) = (pred(1.), pred(10.));
        let (x, y) = (SPf64::mul_up(a, b), SPf64::mul_down(a, b));
        assert!(y == pred(a * b) && succ(a * b) == x);
    }

    #[test]
    fn division() {
        for &(a, b) in [
            (3., 123.),
            (2345.56, -74.12),
            (254634.13590234, 245.4556),
            (32.1, 123.122),
        ].iter()
        {
            let (x, y) = (SPf64::div_up(a, b), SPf64::div_down(a, b));
            assert!(y == pred(a / b) && succ(a / b) == x);
        }
    }

    #[test]
    fn sqrt() {
        for &a in [
            3.,
            123.,
            2345.56,
            74.12,
            254634.13590234,
            245.4556,
            32.1,
            123.122,
        ].iter()
        {
            let (x, y) = (SPf64::sqrt_up(a), SPf64::sqrt_down(a));
            assert!(y == pred(a.sqrt()) && succ(a.sqrt()) == x);
        }
    }
}
