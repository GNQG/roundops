use core::clone::Clone;
use core::marker::PhantomData;

use roundops::*;
use utils::FloatSuccPred;
use float_traits::IEEE754Float;

pub struct SuccPredUnchecked<T>(PhantomData<fn(T)>);

impl<T: FloatSuccPred> RoundAdd for SuccPredUnchecked<T> {
    type Num = T;
    #[inline]
    fn add_up(a: T, b: T) -> T {
        (a + b).succ()
    }
    #[inline]
    fn add_down(a: T, b: T) -> T {
        (a + b).pred()
    }
}

impl<T: FloatSuccPred> RoundSub for SuccPredUnchecked<T> {
    type Num = T;
    #[inline]
    fn sub_up(a: T, b: T) -> T {
        (a - b).succ()
    }
    #[inline]
    fn sub_down(a: T, b: T) -> T {
        (a - b).pred()
    }
}

impl<T: FloatSuccPred> RoundMul for SuccPredUnchecked<T> {
    type Num = T;
    #[inline]
    fn mul_up(a: T, b: T) -> T {
        (a * b).succ()
    }
    #[inline]
    fn mul_down(a: T, b: T) -> T {
        (a * b).pred()
    }
}

impl<T: FloatSuccPred> RoundDiv for SuccPredUnchecked<T> {
    type Num = T;
    #[inline]
    fn div_up(a: T, b: T) -> T {
        (a / b).succ()
    }
    #[inline]
    fn div_down(a: T, b: T) -> T {
        (a / b).pred()
    }
}

impl<T: IEEE754Float + Clone> RoundSqrt for SuccPredUnchecked<T> {
    #[inline]
    fn sqrt_up(a: T) -> T {
        a.sqrt().succ()
    }
    #[inline]
    fn sqrt_down(a: T) -> T {
        a.sqrt().pred()
    }
}

mod tests {
    #[test]
    fn addition() {
        use super::SuccPredUnchecked;
        use roundops::*;
        use super::FloatSuccPred;

        type SPf64 = SuccPredUnchecked<f64>;

        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (SPf64::add_up(a, b), SPf64::add_down(a, b));
        assert!(y == (a + b).pred() && (a + b).succ() == x);
    }

    #[test]
    fn subtraction() {
        use super::SuccPredUnchecked;
        use roundops::*;
        use super::FloatSuccPred;

        type SPf64 = SuccPredUnchecked<f64>;

        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (SPf64::sub_up(a, b), SPf64::sub_down(a, b));
        assert!(y == (a - b).pred() && (a - b).succ() == x);
    }

    #[test]
    fn multiplication() {
        use super::SuccPredUnchecked;
        use roundops::*;
        use super::FloatSuccPred;

        type SPf64 = SuccPredUnchecked<f64>;

        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (SPf64::mul_up(a, b), SPf64::mul_down(a, b));
        assert!(y == (a * b).pred() && (a * b).succ() == x);
    }

    #[test]
    fn division() {
        use super::SuccPredUnchecked;
        use roundops::*;
        use super::FloatSuccPred;

        type SPf64 = SuccPredUnchecked<f64>;

        for &(a, b) in [
            (3., 123.),
            (2345.56, -74.12),
            (254634.13590234, 245.4556),
            (32.1, 123.122),
        ].iter()
        {
            let (x, y) = (SPf64::div_up(a, b), SPf64::div_down(a, b));
            assert!(y == (a / b).pred() && (a / b).succ() == x);
        }
    }

    #[test]
    fn sqrt() {
        use super::SuccPredUnchecked;
        use roundops::*;
        use super::FloatSuccPred;

        type SPf64 = SuccPredUnchecked<f64>;

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
            assert!(y == (a.sqrt().pred()) && (a.sqrt().succ()) == x);
        }
    }
}
