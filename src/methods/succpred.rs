use core::f64;
use core::marker::PhantomData;

use roundops::*;
use utils::{succ, pred};

pub struct SuccPred<T>(PhantomData<fn(T)>);

impl RoundAdd for SuccPred<f64> {
    type Num = f64;
    fn add_up(a: f64, b: f64) -> f64 {
        let x = a + b;
        if x == f64::INFINITY {
            x
        } else if x == f64::NEG_INFINITY {
            if a == f64::NEG_INFINITY || b == f64::NEG_INFINITY {
                x
            } else {
                f64::MIN
            }
        } else {
            succ(x)
        }
    }
    fn add_down(a: f64, b: f64) -> f64 {
        let x = a + b;
        if x == f64::INFINITY {
            if a == f64::NEG_INFINITY || b == f64::NEG_INFINITY {
                x
            } else {
                f64::MAX
            }
        } else if x == f64::NEG_INFINITY {
            x
        } else {
            pred(x)
        }
    }
}

impl RoundSub for SuccPred<f64> {
    type Num = f64;
    #[inline]
    fn sub_up(a: f64, b: f64) -> f64 {
        Self::add_up(a, -b)
    }
    #[inline]
    fn sub_down(a: f64, b: f64) -> f64 {
        Self::add_down(a, -b)
    }
}

impl RoundMul for SuccPred<f64> {
    type Num = f64;
    fn mul_up(a: f64, b: f64) -> f64 {
        let x = a * b;
        if x == f64::INFINITY {
            x
        } else if x == f64::NEG_INFINITY {
            if a == f64::NEG_INFINITY || b == f64::NEG_INFINITY {
                x
            } else {
                f64::MIN
            }
        } else {
            succ(x)
        }
    }
    fn mul_down(a: f64, b: f64) -> f64 {
        let x = a * b;
        if x == f64::INFINITY {
            if a == f64::INFINITY || b == f64::INFINITY {
                x
            } else {
                f64::MAX
            }
        } else if x == f64::NEG_INFINITY {
            x
        } else {
            pred(x)
        }
    }
}

impl RoundDiv for SuccPred<f64> {
    type Num = f64;
    fn div_up(a: f64, b: f64) -> f64 {
        let x = a / b;
        if x == f64::INFINITY {
            x
        } else if x == f64::NEG_INFINITY {
            if b == 0. || a.abs() == f64::INFINITY {
                x
            } else {
                f64::MIN
            }
        } else {
            succ(x)
        }
    }
    fn div_down(a: f64, b: f64) -> f64 {
        let x = a / b;
        if x == f64::INFINITY {
            if b == 0. || a.abs() == f64::INFINITY {
                x
            } else {
                f64::MAX
            }
        } else if x == f64::NEG_INFINITY {
            x
        } else {
            pred(x)
        }
    }
}

impl RoundSqrt for SuccPred<f64> {
    fn sqrt_up(a: f64) -> f64 {
        succ(a.sqrt())
    }
    fn sqrt_down(a: f64) -> f64 {
        let r = a.sqrt();
        if r == f64::INFINITY {
            f64::MAX
        } else {
            pred(r)
        }
    }
}

mod tests {
    use super::SuccPred;
    use roundops::*;
    use super::{succ, pred};

    type SPf64 = SuccPred<f64>;

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
