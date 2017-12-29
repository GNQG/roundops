use core::marker::PhantomData;

use roundops::*;
use utils::FloatSuccPred;
use float_traits::Sqrt;

#[derive(Clone)]
pub struct SuccPredUnchecked<T: FloatSuccPred>(PhantomData<fn(T)>);

impl<T: FloatSuccPred> RoundingMethod for SuccPredUnchecked<T> {
    type HostMethod = rmode::DefaultRounding;
    type Num = T;
}

impl<T: FloatSuccPred> RoundAdd for SuccPredUnchecked<T> {
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
    #[inline]
    fn div_up(a: T, b: T) -> T {
        (a / b).succ()
    }
    #[inline]
    fn div_down(a: T, b: T) -> T {
        (a / b).pred()
    }
}

impl<T: FloatSuccPred + Sqrt<Output = T>> RoundSqrt for SuccPredUnchecked<T> {
    #[inline]
    fn sqrt_up(a: T) -> T {
        a.sqrt().succ()
    }
    #[inline]
    fn sqrt_down(a: T) -> T {
        a.sqrt().pred()
    }
}

impl<T: FloatSuccPred> RoundedSession for SuccPredUnchecked<T> {
    type Num = T;
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    use roundops::*;
    use utils::FloatSuccPred;

    use super::SuccPredUnchecked;

    type SPf64 = SuccPredUnchecked<f64>;

    #[test]
    fn addition() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (SPf64::add_up(a, b), SPf64::add_down(a, b));
            if !((a + b).is_infinite() || a != a || b != b || a + b != a + b) {
                assert!(y <= a + b && a + b <= x);
                assert!(x.pred() == y.succ() || x.is_infinite() || y.is_infinite());
            }
        }
    }

    #[test]
    fn subtraction() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (SPf64::sub_up(a, b), SPf64::sub_down(a, b));
            if !((a - b).is_infinite() || a != a || b != b || a - b != a - b) {
                assert!(y <= a - b && a - b <= x);
                assert!(x.pred() == y.succ() || x.is_infinite() || y.is_infinite());
            }
        }
    }

    #[test]
    fn multiplication() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (SPf64::mul_up(a, b), SPf64::mul_down(a, b));
            if !((a * b).is_infinite() || a != a || b != b || a * b != a * b) {
                assert!(y <= a * b && a * b <= x);
                assert!(x.pred() == y.succ() || x.is_infinite() || y.is_infinite());
            }
        }
    }

    #[test]
    fn division() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let (a, b) = (rng.gen(), rng.gen());
            let (x, y) = (SPf64::div_up(a, b), SPf64::div_down(a, b));
            if !((a / b).is_infinite() || a != a || b != b || a / b != a / b) {
                assert!(y <= a / b && a / b <= x);
                assert!(x.pred() == y.succ() || x.is_infinite() || y.is_infinite());
            }
        }
    }

    #[test]
    fn sqrt() {
        let mut rng = thread_rng();
        for _ in 0..10000000 {
            let a = rng.gen();
            let (x, y) = (SPf64::sqrt_up(a), SPf64::sqrt_down(a));
            if !(a.is_infinite() || a != a || a.sqrt() != a.sqrt()) {
                assert!(y <= a.sqrt() && a.sqrt() <= x);
                assert!(x.pred() == y.succ() || x.is_infinite() || y.is_infinite());
            }
        }
    }
}
