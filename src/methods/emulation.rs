use core::f64;
use core::marker::PhantomData;

use roundops::*;
use super::safeeft::{safetwosum_straight as twosum, safetwoproduct_branch as twoproduct};
use super::{succ,pred};

pub struct Emulation<T>(PhantomData<fn(T)>);

impl RoundAdd for Emulation<f32>{
    type Num = f32;
    fn add_up(a: f32, b: f32) -> f32 {
        unimplemented!()
    }
    fn add_down(a: f32, b: f32) -> f32 {
        unimplemented!()
    }
}

impl RoundAdd for Emulation<f64>{
    type Num = f64;
    fn add_up(a: f64, b: f64) -> f64 {
        let (x, _) = twosum(a, b);
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
        let (x, _) = twosum(a, b);
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

impl RoundSub for Emulation<f64>{
    type Num = f64;
    fn sub_up(a: f64, b: f64) -> f64 {
        Self::add_up(a, -b)
    }
    fn sub_down(a: f64, b: f64) -> f64 {
        Self::add_down(a, -b)
    }
}

impl RoundMul for Emulation<f64>{
    type Num = f64;
    fn mul_up(a: f64, b: f64) -> f64 {
        let (x, y) = twoproduct(a, b);
        if x == f64::INFINITY {
            x
        } else if x == f64::NEG_INFINITY {
            if a == f64::NEG_INFINITY || b == f64::NEG_INFINITY {
                x
            } else {
                f64::MIN
            }
        } else {
            let (p537, pm969) = (2f64.powi(537), 2f64.powi(-969));
            if x.abs() > pm969 {
                if y > 0. {
                    succ(x)
                } else {
                    x
                }
            } else {
                let (s_h, s_l) = twoproduct(a * p537, b * p537); // TODO: check
                let t = (x * p537) * p537;
                if t < s_h || (t == s_h && s_l > 0.) {
                    succ(x)
                } else {
                    x
                }
            }
        }
    }
    fn mul_down(a: f64, b: f64) -> f64 {
        let (x, y) = twoproduct(a, b);
        if x == f64::INFINITY {
            if a == f64::INFINITY || b == f64::INFINITY {
                // TODO: check
                x
            } else {
                f64::MAX
            }
        } else if x == f64::NEG_INFINITY {
            x
        } else {
            let (p537, pm969) = (2f64.powi(537), 2f64.powi(-969));
            if x.abs() > pm969 {
                if y < 0. {
                    pred(x)
                } else {
                    x
                }
            } else {
                let (s_h, s_l) = twoproduct(a * p537, b * p537);
                let t = (x * p537) * p537;
                if t > s_h || (t == s_h && s_l < 0.) {
                    pred(x)
                } else {
                    x
                }
            }
        }
    }
}

impl RoundDiv for Emulation<f64>{
    type Num = f64;
    fn div_up(a: f64, b: f64) -> f64 {
        if a == 0. || b == 0. || a.abs() == f64::INFINITY || b.abs() == f64::INFINITY ||
            a != a || b != b
        {
            a / b
        } else {
            let (p105, p918, pm969, pm1074) = (
                2f64.powi(105),
                2f64.powi(918),
                2f64.powi(-969),
                2f64.powi(-1074),
            );
            let (mut ss, mut bb) = (a, b);
            if b < 0. {
                ss *= -1.;
                bb *= -1.;
            }
            if ss.abs() < pm969 {
                if bb.abs() < p918 {
                    ss *= p105;
                    bb *= p105;
                } else {
                    if ss < 0. {
                        return 0f64;
                    } else {
                        return pm1074;
                    }
                }
            }
            let d = ss / bb;
            if d.is_infinite() {
                if d > 0. {
                    d
                } else {
                    f64::MIN
                }
            } else {
                let (x, y) = twoproduct(d, bb);
                if x < ss || (x == ss && y < 0.) {
                    succ(d)
                } else {
                    d
                }
            }
        }
    }
    fn div_down(a: f64, b: f64) -> f64 {
        if a == 0. || b == 0. || a.abs() == f64::INFINITY || b.abs() == f64::INFINITY ||
            a != a || b != b
        {
            a / b
        } else {
            let (p105, p918, pm969, pm1074) = (
                2f64.powi(105),
                2f64.powi(918),
                2f64.powi(-969),
                2f64.powi(-1074),
            );
            let (mut ss, mut bb) = (a, b);

            if b < 0. {
                ss *= -1.;
                bb *= -1.;
            }
            if ss.abs() < pm969 {
                if bb.abs() < p918 {
                    ss *= p105;
                    bb *= p105;
                } else {
                    if ss < 0. {
                        return -pm1074;
                    } else {
                        return 0f64;
                    }
                }
            }
            let d = ss / bb;
            if d.is_infinite() {
                if d > 0. {
                    f64::MAX
                } else {
                    d
                }
            } else {
                let (x, y) = twoproduct(d, bb);
                if x > ss || (x == ss && y > 0.) {
                    pred(d)
                } else {
                    d
                }
            }
        }
    }
}

impl RoundSqrt<f64> for Emulation<f64>{
    fn sqrt_up(a: f64) -> f64 {
        let (p53, pm969) = (2f64.powi(53), 2f64.powi(-969));
        let r = a.sqrt();
        if a < pm969 {
            let (ss, rr) = (a * p53 * p53, r * p53);
            let (x, y) = twoproduct(ss, rr);
            if x < ss || (x == ss && y < 0.) {
                succ(r)
            } else {
                r
            }
        } else {
            let (x, y) = twoproduct(a, a); // TODO: check
            if x < a || (x == a && y < 0.) {
                succ(r)
            } else {
                r
            }
        }
    }
    fn sqrt_down(a: f64) -> f64 {
        let (p53, pm969) = (2f64.powi(53), 2f64.powi(-969));
        let r = a.sqrt();
        if a < pm969 {
            let (ss, rr) = (a * p53 * p53, r * p53);
            let (x, y) = twoproduct(ss, rr);
            if x > ss || (x == ss && y > 0.) {
                pred(r)
            } else {
                r
            }
        } else {
            let (x, y) = twoproduct(a, a); // TODO: check
            if x > a || (x == a && y > 0.) {
                pred(r)
            } else {
                r
            }
        }
    }
}
