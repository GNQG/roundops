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
        if x < a || (x == a && y < 0.) {
            succ(d)
        } else {
            d
        }
    }
    fn div_down(a: f64, b: f64) -> f64 {
        let (a, b) = if b < 0. { (-a, -b) } else { (a, b) };
        let d = a / b;
        let (x, y) = twoproduct(d, b);
        if x > a || (x == a && y > 0.) {
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
