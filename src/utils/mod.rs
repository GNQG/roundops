#[doc(hidden)]
pub extern crate safeeft;
#[cfg(feature = "use-fma")]
#[doc(hidden)]
pub extern crate fma;

mod succpred;
pub use self::succpred::{succ,pred,FloatSuccPred};
