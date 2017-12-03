use core::marker::PhantomData;

use roundops::*;
use super::safeeft::{safetwosum_straight as twosum, safetwoproduct_branch as twoproduct};
use float_traits::IEEE754Float;
use utils::FloatSuccPred;

pub struct EmulationUnchecked<T>(PhantomData<fn(T)>);

impl<T: IEEE754Float + Clone> RoundAdd for EmulationUnchecked<T> {
    type Num = T;
    fn add_up(a: T, b: T) -> T {
        let (x, y) = twosum(a, b);
        if y > T::zero() { x.succ() } else { x }
    }
    fn add_down(a: T, b: T) -> T {
        let (x, y) = twosum(a, b);
        if y < T::zero() { x.pred() } else { x }
    }
}

impl<T: IEEE754Float + Clone> RoundSub for EmulationUnchecked<T> {
    type Num = T;
    #[inline]
    fn sub_up(a: T, b: T) -> T {
        Self::add_up(a, -b)
    }
    #[inline]
    fn sub_down(a: T, b: T) -> T {
        Self::add_down(a, -b)
    }
}

impl<T: IEEE754Float + Clone> RoundMul for EmulationUnchecked<T> {
    type Num = T;
    fn mul_up(a: T, b: T) -> T {
        let (x, y) = twoproduct(a, b);
        if y > T::zero() { x.succ() } else { x }
    }
    fn mul_down(a: T, b: T) -> T {
        let (x, y) = twoproduct(a, b);
        if y < T::zero() { x.pred() } else { x }
    }
}

impl<T: IEEE754Float + Clone> RoundDiv for EmulationUnchecked<T> {
    type Num = T;
    fn div_up(a: T, b: T) -> T {
        let (a, b) = if b < T::zero() { (-a, -b) } else { (a, b) };
        let d = a.clone() / b.clone();
        let (x, y) = twoproduct(d.clone(), b);
        if x < a || (x == a && y > T::zero()) {
            d.succ()
        } else {
            d
        }
    }
    fn div_down(a: T, b: T) -> T {
        let (a, b) = if b < T::zero() { (-a, -b) } else { (a, b) };
        let d = a.clone() / b.clone();
        let (x, y) = twoproduct(d.clone(), b);
        if x > a || (x == a && y < T::zero()) {
            d.pred()
        } else {
            d
        }
    }
}

impl<T: IEEE754Float + Clone> RoundSqrt for EmulationUnchecked<T> {
    fn sqrt_up(a: T) -> T {
        let r = a.clone().sqrt();
        let (x, y) = twoproduct(r.clone(), r.clone());
        if x < a || (x == a && y < T::zero()) {
            r.succ()
        } else {
            r
        }
    }
    fn sqrt_down(a: T) -> T {
        let r = a.clone().sqrt();
        let (x, y) = twoproduct(r.clone(), r.clone());
        if x > a || (x == a && y > T::zero()) {
            r.pred()
        } else {
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EmulationUnchecked;
    use roundops::*;
    use super::FloatSuccPred;
    type Emuf64 = EmulationUnchecked<f64>;

    #[test]
    fn addition() {
        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (Emuf64::add_up(a, b), Emuf64::add_down(a, b));
        assert!(x == y.succ() || x == y);
        assert!(y <= a + b && a + b <= x);
    }

    #[test]
    fn subtraction() {
        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (Emuf64::sub_up(a, b), Emuf64::sub_down(a, b));
        assert!(x == y.succ() || x == y);
        assert!(y <= a - b && a - b <= x);
    }

    #[test]
    fn multiplication() {
        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (Emuf64::mul_up(a, b), Emuf64::mul_down(a, b));
        assert!(x == y.succ() || x == y);
        assert!(y <= a * b || a * b <= x);
    }

    #[test]
    fn division() {
        for &(a, b) in [(3., 123.),
                        (2345.56, -74.12),
                        (254634.13590234, 245.4556),
                        (32.1, 123.122)]
                    .iter() {
            let (x, y) = (Emuf64::div_up(a, b), Emuf64::div_down(a, b));
            assert!(x == y.succ() || x == y);
            assert!(y <= a / b && a / b <= x);
        }
    }

    #[test]
    fn sqrt() {
        for &a in [3., 123., 2345.56, 74.12, 254634.13590234, 245.4556, 32.1, 123.122].iter() {
            use super::twoproduct;
            let (x, y) = (Emuf64::sqrt_up(a), Emuf64::sqrt_down(a));
            println!("{:e}, [{:e}, {:e}]", a.sqrt(), y, x);
            println!("{:?}", twoproduct(a.sqrt(), a.sqrt()));
            assert!(x == y.succ() || x == y);
            assert!(y <= a.sqrt() && a.sqrt() <= x);
        }
    }
}
