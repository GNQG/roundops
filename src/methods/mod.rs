extern crate safeeft;

mod hardware;
mod emulation;
mod emulation_plain;
mod succpred;
mod succpred_plain;

pub use self::emulation::Emulation;

fn succ(x: f64) -> f64 {
    let abs = x.abs();
    let (p53, pm53, pm105, pm969, pm1021, pm1071) = (
        2f64.powi(53),
        2f64.powi(-53),
        2f64.powi(-105),
        2f64.powi(-969),
        2f64.powi(-1021),
        2f64.powi(-1071),
    );
    if abs >= pm969 {
        x + abs * (pm53 + pm105)
    } else if abs < pm1021 {
        x + pm1071
    } else {
        let c = x * p53;
        let e = (pm53 + pm105) * c.abs();
        (c + e) * pm53
    }
}

fn pred(x: f64) -> f64 {
    let abs = x.abs();
    let (p53, pm53, pm105, pm969, pm1021, pm1071) = (
        2f64.powi(53),
        2f64.powi(-53),
        2f64.powi(-105),
        2f64.powi(-969),
        2f64.powi(-1021),
        2f64.powi(-1071),
    );
    if abs >= pm969 {
        x - abs * (pm53 + pm105)
    } else if abs < pm1021 {
        x - pm1071
    } else {
        let c = x * p53;
        let e = (pm53 + pm105) * c.abs();
        (c - e) * pm53
    }
}
