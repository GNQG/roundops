use core::marker::PhantomData;

use float_traits::IEEE754Float;

use roundops::*;
use super::safeeft::{safetwosum_straight as twosum, safetwoproduct_branch as twoproduct};
use utils::FloatSuccPred;

pub struct Emulation<T>(PhantomData<fn(T)>);

impl<T: IEEE754Float + Clone> RoundAdd for Emulation<T> {
    type Num = T;
    fn add_up(a: T, b: T) -> T {
        let (x, y) = twosum(a.clone(), b.clone());
        if x == T::infinity() {
            x
        } else if x == T::neg_infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            if y > T::zero() { x.succ() } else { x }
        }
    }
    fn add_down(a: T, b: T) -> T {
        let (x, y) = twosum(a.clone(), b.clone());
        if x == T::infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::max_value()
            }
        } else if x == T::neg_infinity() {
            x
        } else {
            if y < T::zero() { x.pred() } else { x }
        }
    }
}

impl<T: IEEE754Float + Clone> RoundSub for Emulation<T> {
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

impl<T: IEEE754Float + Clone> RoundMul for Emulation<T> {
    type Num = T;
    fn mul_up(a: T, b: T) -> T {
        let (x, y) = twoproduct(a.clone(), b.clone());
        if x == T::infinity() {
            x
        } else if x == T::neg_infinity() {
            if a == T::neg_infinity() || b == T::neg_infinity() {
                x
            } else {
                T::min_value()
            }
        } else {
            if x.clone().abs() > T::min_positive() / T::eps() * T::radix() {
                if y > T::zero() { x.succ() } else { x }
            } else {
                let (s_h, s_l) = twoproduct(a * T::unit_underflow().sqrt(),
                                            b * T::unit_underflow().sqrt()); // TODO: check
                let t = (x.clone() * T::unit_underflow().sqrt()) * T::unit_underflow().sqrt();
                if t < s_h || (t == s_h && s_l > T::zero()) {
                    x.succ()
                } else {
                    x
                }
            }
        }
    }
    fn mul_down(a: T, b: T) -> T {
        let (x, y) = twoproduct(a.clone(), b.clone());
        if x == T::infinity() {
            if a == T::infinity() || b == T::infinity() {
                // TODO: check
                x
            } else {
                T::max_value()
            }
        } else if x == T::neg_infinity() {
            x
        } else {
            if x.clone().abs() > T::min_positive() / T::eps() * T::radix() {
                if y < T::zero() { x.pred() } else { x }
            } else {
                let (s_h, s_l) = twoproduct(a * T::unit_underflow().sqrt(),
                                            b * T::unit_underflow().sqrt());
                let t = (x.clone() * T::unit_underflow().sqrt()) * T::unit_underflow().sqrt();
                if t > s_h || (t == s_h && s_l < T::zero()) {
                    x.pred()
                } else {
                    x
                }
            }
        }
    }
}

impl<T: IEEE754Float + Clone> RoundDiv for Emulation<T> {
    type Num = T;
    fn div_up(a: T, b: T) -> T {
        if a == T::zero() || b == T::zero() || a.clone().abs() == T::infinity() ||
           b.clone().abs() == T::infinity() || a != a || b != b {
            a / b
        } else {
            let (mut ss, mut bb) = (a, b);
            if bb < T::zero() {
                ss = ss * -T::one();
                bb = bb * -T::one();
            }
            if ss.clone().abs() < T::min_positive() / T::eps() * T::radix() {
                if bb.clone().abs() < T::min_positive() / T::eps() / T::eps() {
                    ss = ss * T::radix() * T::eps() * T::eps();
                    bb = bb * T::radix() * T::eps() * T::eps();
                } else {
                    if ss < T::zero() {
                        return T::zero();
                    } else {
                        return T::unit_underflow();
                    }
                }
            }
            let d = ss.clone() / bb.clone();
            if d.is_infinite() {
                if d > T::zero() { d } else { T::min_value() }
            } else {
                let (x, y) = twoproduct(d.clone(), bb.clone());
                if x < ss || (x == ss && y > T::zero()) {
                    d.succ()
                } else {
                    d
                }
            }
        }
    }
    fn div_down(a: T, b: T) -> T {
        if a == T::zero() || b == T::zero() || a.clone().abs() == T::infinity() ||
           b.clone().abs() == T::infinity() || a != a || b != b {
            a / b
        } else {
            let (mut ss, mut bb) = (a, b);

            if bb < T::zero() {
                ss = ss * -T::one();
                bb = bb * -T::one();
            }
            if ss.clone().abs() < T::min_positive() / T::eps() * T::radix() {
                if bb.clone().abs() < T::min_positive() / T::eps() / T::eps() {
                    ss = ss * T::radix() * T::eps() * T::eps();
                    bb = bb * T::radix() * T::eps() * T::eps();
                } else {
                    if ss < T::zero() {
                        return -T::unit_underflow();
                    } else {
                        return T::zero();
                    }
                }
            }
            let d = ss.clone() / bb.clone();
            if d.is_infinite() {
                if d > T::zero() { T::max_value() } else { d }
            } else {
                let (x, y) = twoproduct(d.clone(), bb.clone());
                if x > ss || (x == ss && y < T::zero()) {
                    d.pred()
                } else {
                    d
                }
            }
        }
    }
}

impl<T: IEEE754Float + Clone> RoundSqrt for Emulation<T> {
    fn sqrt_up(a: T) -> T {
        let r = a.clone().sqrt();
        if a < T::min_positive() / T::eps() * T::radix() {
            let (ss, rr) = (a * T::radix() / T::eps() * T::radix() / T::eps(),
                            r.clone() * T::radix() / T::eps());
            let (x, y) = twoproduct(ss.clone(), rr.clone());
            if x < ss || (x == ss && y < T::zero()) {
                r.succ()
            } else {
                r
            }
        } else {
            let (x, y) = twoproduct(r.clone(), r.clone());
            if x < a || (x == a && y < T::zero()) {
                r.succ()
            } else {
                r
            }
        }
    }
    fn sqrt_down(a: T) -> T {
        let r = a.clone().sqrt();
        if a < T::min_positive() / T::eps() * T::radix() {
            let (ss, rr) = (a * T::radix() / T::eps() * T::radix() / T::eps(),
                            r.clone() * T::radix() / T::eps());
            let (x, y) = twoproduct(ss.clone(), rr.clone());
            if x > ss || (x == ss && y > T::zero()) {
                r.pred()
            } else {
                r
            }
        } else {
            let (x, y) = twoproduct(r.clone(), r.clone());
            if x > a || (x == a && y > T::zero()) {
                r.pred()
            } else {
                r
            }
        }
    }
}

mod tests {
    #[test]
    fn addition() {
        use super::Emulation;
        use roundops::*;
        use super::FloatSuccPred;

        type Emuf64 = Emulation<f64>;

        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (EmuT::add_up(a, b), EmuT::add_down(a, b));
        assert!(x == y.succ() || x == y);
        assert!(y <= a + b && a + b <= x);
    }

    #[test]
    fn subtraction() {
        use super::Emulation;
        use roundops::*;
        use super::FloatSuccPred;

        type Emuf64 = Emulation<f64>;

        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (EmuT::sub_up(a, b), EmuT::sub_down(a, b));
        assert!(x == y.succ() || x == y);
        assert!(y <= a - b && a - b <= x);
    }

    #[test]
    fn multiplication() {
        use super::Emulation;
        use roundops::*;
        use super::FloatSuccPred;

        type Emuf64 = Emulation<f64>;

        let (a, b) = ((1.).pred(), (10.).pred());
        let (x, y) = (EmuT::mul_up(a, b), EmuT::mul_down(a, b));
        assert!(x == y.succ() || x == y);
        assert!(y <= a * b || a * b <= x);
    }

    #[test]
    fn division() {
        use super::Emulation;
        use roundops::*;
        use super::FloatSuccPred;

        type Emuf64 = Emulation<f64>;

        for &(a, b) in [(3., 123.),
                        (2345.56, -74.12),
                        (254634.13590234, 245.4556),
                        (32.1, 123.122)]
                    .iter() {
            let (x, y) = (EmuT::div_up(a, b), EmuT::div_down(a, b));
            assert!(x == y.succ() || x == y);
            assert!(y <= a / b && a / b <= x);
        }
    }

    #[test]
    fn sqrt() {
        use super::Emulation;
        use roundops::*;
        use super::FloatSuccPred;

        type Emuf64 = Emulation<f64>;

        for &a in [3., 123., 2345.56, 74.12, 254634.13590234, 245.4556, 32.1, 123.122].iter() {
            use super::twoproduct;
            let (x, y) = (EmuT::sqrt_up(a), EmuT::sqrt_down(a));
            println!("{:e}, [{:e}, {:e}]", a.sqrt(), y, x);
            println!("{:?}", twoproduct(a.sqrt().clone(), a.sqrt(.clone())));
            assert!(x == y.succ() || x == y);
            assert!(y <= a.sqrt() && a.sqrt() <= x);
        }
    }
}
