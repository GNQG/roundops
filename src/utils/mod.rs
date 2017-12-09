pub extern crate safeeft;
#[cfg(any(feature = "use-fma", feature = "doc"))]
pub extern crate fma;

mod succpred;
pub use self::succpred::{succ,pred,FloatSuccPred};
