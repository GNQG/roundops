use core::f64;
use core::marker::PhantomData;

use roundops::*;
use super::safeeft::{safetwosum_straight as twosum, safetwoproduct_branch as twoproduct};
use utils::{succ, pred};

pub struct EmulationUnchecked<T>(PhantomData<fn(T)>);

impl RoundAdd for EmulationUnchecked<f64> {
    type Num = f64;
    fn add_up(a: f64, b: f64) -> f64 {
        let (x, y) = twosum(a, b);
        if y > 0. { succ(x) } else { x }
    }
    fn add_down(a: f64, b: f64) -> f64 {
        let (x, y) = twosum(a, b);
        if y < 0. { pred(x) } else { x }
    }
}

impl RoundSub for EmulationUnchecked<f64> {
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

impl RoundMul for EmulationUnchecked<f64> {
    type Num = f64;
    fn mul_up(a: f64, b: f64) -> f64 {
        let (x, y) = twoproduct(a, b);
        if y > 0. { succ(x) } else { x }
    }
    fn mul_down(a: f64, b: f64) -> f64 {
        let (x, y) = twoproduct(a, b);
        if y < 0. { pred(x) } else { x }
    }
}

impl RoundDiv for EmulationUnchecked<f64> {
    type Num = f64;
    fn div_up(a: f64, b: f64) -> f64 {
        let (a, b) = if b < 0. { (-a, -b) } else { (a, b) };
        let d = a / b;
        let (x, y) = twoproduct(d, b);
        if x < a || (x == a && y > 0.) {
            succ(d)
        } else {
            d
        }
    }
    fn div_down(a: f64, b: f64) -> f64 {
        let (a, b) = if b < 0. { (-a, -b) } else { (a, b) };
        let d = a / b;
        let (x, y) = twoproduct(d, b);
        if x > a || (x == a && y < 0.) {
            pred(d)
        } else {
            d
        }
    }
}

impl RoundSqrt for EmulationUnchecked<f64> {
    fn sqrt_up(a: f64) -> f64 {
        let r = a.sqrt();
        let (x, y) = twoproduct(r, r);
        if x < a || (x == a && y < 0.) {
            succ(r)
        } else {
            r
        }
    }
    fn sqrt_down(a: f64) -> f64 {
        let r = a.sqrt();
        let (x, y) = twoproduct(r, r);
        if x > a || (x == a && y > 0.) {
            pred(r)
        } else {
            r
        }
    }
}

mod tests {
    use super::EmulationUnchecked;
    use roundops::*;
    use super::{succ, pred};

    type Emuf64 = EmulationUnchecked<f64>;

    #[test]
    fn addition() {
        let (a, b) = (pred(1.), pred(10.));
        let (x, y) = (Emuf64::add_up(a, b), Emuf64::add_down(a, b));
        assert!(x == succ(y) || x == y);
        assert!(y <= a + b && a + b <= x);
    }

    #[test]
    fn subtraction() {
        let (a, b) = (pred(1.), pred(10.));
        let (x, y) = (Emuf64::sub_up(a, b), Emuf64::sub_down(a, b));
        assert!(x == succ(y) || x == y);
        assert!(y <= a - b && a - b <= x);
    }

    #[test]
    fn multiplication() {
        let (a, b) = (pred(1.), pred(10.));
        let (x, y) = (Emuf64::mul_up(a, b), Emuf64::mul_down(a, b));
        assert!(x == succ(y) || x == y);
        assert!(y <= a * b || a * b <= x);
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
            let (x, y) = (Emuf64::div_up(a, b), Emuf64::div_down(a, b));
            assert!(x == succ(y) || x == y);
            assert!(y <= a / b && a / b <= x);
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
            use super::twoproduct;
            let (x, y) = (Emuf64::sqrt_up(a), Emuf64::sqrt_down(a));
            println!("{:e}, [{:e}, {:e}]", a.sqrt(), y, x);
            println!("{:?}",twoproduct(a.sqrt(),a.sqrt()));
            assert!(x == succ(y) || x == y);
            assert!(y <= a.sqrt() && a.sqrt() <= x);
        }
    }
}
